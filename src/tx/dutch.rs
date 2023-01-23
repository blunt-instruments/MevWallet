use ethers::{
    providers::Middleware,
    signers::Signer,
    types::{I256, U256},
};

use crate::{MevTx, MevWalletV1, SignedMevTx};

use super::{BuilderError, MevTxBuilderInternal, SignedMevTxBuilderInternal};

/// Policy for escalating
pub trait EscalationPolicy: IntoIterator<Item = (U256, I256)> {}

impl<T> EscalationPolicy for T where T: IntoIterator<Item = (U256, I256)> {}

/// A tx builder that produces an ascending-tip MEV gas auction
pub struct DutchBuilder<M, T> {
    /// Internal builder
    builder: MevTxBuilderInternal<MevWalletV1<M>>,
    /// Fee escalation policy
    policy: T,
}

impl<M, T> DutchBuilder<M, T>
where
    T: EscalationPolicy,
    M: Middleware + 'static,
{
    /// Add a signer to the builder
    pub fn with_signer<S: Signer>(self, signer: S) -> SignedDutchBuilder<M, T, S> {
        SignedDutchBuilder {
            builder: self.builder.with_signer(signer),
            policy: self.policy,
        }
    }

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
/// A Reverse-Dutch MEV auction transaction builder
pub struct SignedDutchBuilder<M, T, S> {
    /// Internal builder
    builder: SignedMevTxBuilderInternal<MevWalletV1<M>, S>,
    /// Fee escalation policy
    policy: T,
}

impl<M, T, S> SignedDutchBuilder<M, T, S>
where
    T: EscalationPolicy,
    M: Middleware + 'static,
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
