use ethers::{
    providers::Middleware,
    signers::Signer,
    types::{I256, U256},
};

use crate::{delegate_builder, delegate_builder_populate, MevTx, MevWalletV1, SignedMevTx};

use super::{BuilderError, MevTxBuilderInternal, SignedMevTxBuilderInternal};

/// Policy for escalating
pub trait EscalationPolicy: IntoIterator<Item = (U256, I256)> {}

impl<T> EscalationPolicy for T where T: IntoIterator<Item = (U256, I256)> {}

/// A tx builder that produces an ascending-tip MEV gas auction
pub struct DutchBuilder<M, T> {
    /// Internal builder
    builder: MevTxBuilderInternal<M>,
    /// Fee escalation policy
    policy: T,
}

impl<M, T> DutchBuilder<M, T> {
    /// Add a signer to the builder
    pub fn with_signer<S: Signer>(self, signer: S) -> SignedDutchBuilder<M, T, S> {
        SignedDutchBuilder {
            builder: self.builder.with_signer(signer),
            policy: self.policy,
        }
    }
}

impl<M, T> DutchBuilder<MevWalletV1<M>, T>
where
    T: EscalationPolicy,
    M: Middleware + 'static,
{
    /// Build an array of transactions adjusting the tip according to the escalation policy
    pub fn build(self) -> Result<Vec<MevTx>, BuilderError> {
        let mut txns = vec![];
        for (not_before, new_tip) in self.policy.into_iter() {
            let tx = self
                .builder
                .clone()
                .tip(new_tip)
                .not_before(not_before)
                .build()?;
            txns.push(tx);
        }
        Ok(txns)
    }
}

delegate_builder!(DutchBuilder<M, T>);
delegate_builder_populate!(DutchBuilder<MevWalletV1<M>, T>);

/// A Reverse-Dutch MEV auction transaction builder
pub struct SignedDutchBuilder<M, T, S> {
    /// Internal builder
    builder: SignedMevTxBuilderInternal<M, S>,
    /// Fee escalation policy
    policy: T,
}

impl<M, T, S> SignedDutchBuilder<MevWalletV1<M>, T, S>
where
    T: EscalationPolicy,
    M: Middleware + 'static + Clone,
    S: Signer + Clone, // TODO: fix
{
    /// Build the dutch auction and sign the txns
    pub async fn build(self) -> Result<Vec<SignedMevTx>, BuilderError> {
        let mut txns = vec![];
        for (not_before, new_tip) in self.policy.into_iter() {
            let tx = self
                .builder
                .clone()
                .tip(new_tip)
                .not_before(not_before)
                .build()
                .await?;
            txns.push(tx);
        }
        Ok(txns)
    }
}

delegate_builder!(SignedDutchBuilder<M, T, S>);
delegate_builder_populate!(SignedDutchBuilder<MevWalletV1<M>, T, S>);
