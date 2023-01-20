#!/bin/sh

# MevWeth Binary
export MWB="6080604052600436106101fd5760003560e01c80638237e5381161010d578063cae9ca51116100a0578063d9d98ce41161006f578063d9d98ce4146108f2578063dd62ed3e1461092b578063ea3e3e3e14610966578063f801cf601461099f578063f8e5c102146109d85761023d565b8063cae9ca51146107e7578063cd0d009614610877578063d0e30db01461088c578063d505accf146108945761023d565b806395d89b41116100dc57806395d89b411461075e57806399c2463114610773578063a9059cbb14610788578063b760faf9146107c15761023d565b80638237e538146106be578063848316fb146106d35780638b28d32f146107065780639555a9421461071b5761023d565b8063313ce567116101905780635cffe9de1161015f5780635cffe9de1461050c5780635ddb7d7e146105a7578063613255ab1461062557806370a08231146106585780637ecebe001461068b5761023d565b8063313ce567146104125780633644e5151461043d5780634000aea0146104525780635992bfdd146104e25761023d565b8063205c2878116101cc578063205c28781461035557806323b872dd146103905780632e1a7d4d146103d357806330adf81f146103fd5761023d565b806306fdde0314610242578063095ea7b3146102cc57806313c9b33a1461031957806318160ddd146103405761023d565b3661023d57336000818152602081815260408083208054349081019091558151908152905160008051602061257b833981519152929181900390910190a3005b600080fd5b34801561024e57600080fd5b50610257610a02565b6040805160208082528351818301528351919283929083019185019080838360005b83811015610291578181015183820152602001610279565b50505050905090810190601f1680156102be5780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b3480156102d857600080fd5b50610305600480360360408110156102ef57600080fd5b506001600160a01b038135169060200135610a26565b604080519115158252519081900360200190f35b34801561032557600080fd5b5061032e610a7a565b60408051918252519081900360200190f35b34801561034c57600080fd5b5061032e610a80565b34801561036157600080fd5b5061038e6004803603604081101561037857600080fd5b506001600160a01b038135169060200135610a88565b005b34801561039c57600080fd5b50610305600480360360608110156103b357600080fd5b506001600160a01b03813581169160208101359091169060400135610ba9565b3480156103df57600080fd5b5061038e600480360360208110156103f657600080fd5b5035610e82565b34801561040957600080fd5b5061032e610f99565b34801561041e57600080fd5b50610427610fbd565b6040805160ff9092168252519081900360200190f35b34801561044957600080fd5b5061032e610fc2565b34801561045e57600080fd5b506103056004803603606081101561047557600080fd5b6001600160a01b0382351691602081013591810190606081016040820135600160201b8111156104a457600080fd5b8201836020820111156104b657600080fd5b803590602001918460018302840111600160201b831117156104d757600080fd5b509092509050611022565b3480156104ee57600080fd5b5061038e6004803603602081101561050557600080fd5b50356112ac565b34801561051857600080fd5b506103056004803603608081101561052f57600080fd5b6001600160a01b03823581169260208101359091169160408201359190810190608081016060820135600160201b81111561056957600080fd5b82018360208201111561057b57600080fd5b803590602001918460018302840111600160201b8311171561059c57600080fd5b50909250905061133e565b610305600480360360408110156105bd57600080fd5b6001600160a01b038235169190810190604081016020820135600160201b8111156105e757600080fd5b8201836020820111156105f957600080fd5b803590602001918460018302840111600160201b8311171561061a57600080fd5b50909250905061173a565b34801561063157600080fd5b5061032e6004803603602081101561064857600080fd5b50356001600160a01b0316611836565b34801561066457600080fd5b5061032e6004803603602081101561067b57600080fd5b50356001600160a01b0316611862565b34801561069757600080fd5b5061032e600480360360208110156106ae57600080fd5b50356001600160a01b0316611874565b3480156106ca57600080fd5b5061032e611886565b3480156106df57600080fd5b5061038e600480360360208110156106f657600080fd5b50356001600160a01b03166118aa565b34801561071257600080fd5b5061032e6118f8565b34801561072757600080fd5b5061038e6004803603606081101561073e57600080fd5b506001600160a01b038135811691602081013590911690604001356118fe565b34801561076a57600080fd5b50610257611b15565b34801561077f57600080fd5b5061038e611b35565b34801561079457600080fd5b50610305600480360360408110156107ab57600080fd5b506001600160a01b038135169060200135611b79565b61038e600480360360208110156107d757600080fd5b50356001600160a01b0316611d63565b3480156107f357600080fd5b506103056004803603606081101561080a57600080fd5b6001600160a01b0382351691602081013591810190606081016040820135600160201b81111561083957600080fd5b82018360208201111561084b57600080fd5b803590602001918460018302840111600160201b8311171561086c57600080fd5b509092509050611da8565b34801561088357600080fd5b5061032e611e7d565b61038e611ea1565b3480156108a057600080fd5b5061038e600480360360e08110156108b757600080fd5b506001600160a01b03813581169160208101359091169060408101359060608101359060ff6080820135169060a08101359060c00135611edc565b3480156108fe57600080fd5b5061032e6004803603604081101561091557600080fd5b506001600160a01b038135169060200135612179565b34801561093757600080fd5b5061032e6004803603604081101561094e57600080fd5b506001600160a01b03813581169160200135166121e1565b34801561097257600080fd5b5061038e6004803603604081101561098957600080fd5b506001600160a01b0381351690602001356121fe565b3480156109ab57600080fd5b5061038e600480360360408110156109c257600080fd5b506001600160a01b038135169060200135612259565b3480156109e457600080fd5b5061038e600480360360208110156109fb57600080fd5b50356123cf565b6040518060400160405280600881526020016709acaec40aecae8d60c31b81525081565b3360008181526002602090815260408083206001600160a01b0387168085529083528184208690558151868152915193949093909260008051602061259b833981519152928290030190a350600192915050565b60045481565b600354470190565b3360009081526020819052604090205481811015610ad75760405162461bcd60e51b81526004018080602001828103825260218152602001806125bb6021913960400191505060405180910390fd5b3360008181526020818152604080832086860390558051868152905192939260008051602061257b833981519152929181900390910190a36040516000906001600160a01b0385169084908381818185875af1925050503d8060008114610b5a576040519150601f19603f3d011682016040523d82523d6000602084013e610b5f565b606091505b5050905080610ba3576040805162461bcd60e51b81526020600482015260196024820152600080516020612537833981519152604482015290519081900360640190fd5b50505050565b60006001600160a01b0384163314610c7c576001600160a01b03841660009081526002602090815260408083203384529091529020546000198114610c7a5782811015610c2b576040805162461bcd60e51b815260206004820152601f6024820152600080516020612517833981519152604482015290519081900360640190fd5b6001600160a01b03851660008181526002602090815260408083203380855290835292819020878603908190558151818152915190949260008051602061259b833981519152928290030190a3505b505b6001600160a01b03831615801590610c9d57506001600160a01b0383163014155b15610d51576001600160a01b03841660009081526020819052604090205482811015610cfa5760405162461bcd60e51b81526004018080602001828103825260258152602001806124f26025913960400191505060405180910390fd5b6001600160a01b0380861660008181526020818152604080832088870390559388168083529184902080548801905583518781529351919360008051602061257b833981519152929081900390910190a350610e78565b6001600160a01b03841660009081526020819052604090205482811015610da95760405162461bcd60e51b81526004018080602001828103825260218152602001806125bb6021913960400191505060405180910390fd5b6001600160a01b03851660008181526020818152604080832087860390558051878152905192939260008051602061257b833981519152929181900390910190a3604051600090339085908381818185875af1925050503d8060008114610e2c576040519150601f19603f3d011682016040523d82523d6000602084013e610e31565b606091505b5050905080610e75576040805162461bcd60e51b81526020600482015260196024820152600080516020612537833981519152604482015290519081900360640190fd5b50505b5060019392505050565b3360009081526020819052604090205481811015610ed15760405162461bcd60e51b81526004018080602001828103825260218152602001806125bb6021913960400191505060405180910390fd5b3360008181526020818152604080832086860390558051868152905192939260008051602061257b833981519152929181900390910190a3604051600090339084908381818185875af1925050503d8060008114610f4b576040519150601f19603f3d011682016040523d82523d6000602084013e610f50565b606091505b5050905080610f94576040805162461bcd60e51b81526020600482015260196024820152600080516020612537833981519152604482015290519081900360640190fd5b505050565b7f000000000000000000000000000000000000000000000000000000000000000081565b601281565b6000467f00000000000000000000000000000000000000000000000000000000000000008114610ffa57610ff581612420565b61101c565b7f00000000000000000000000000000000000000000000000000000000000000005b91505090565b60006001600160a01b038516156110d75733600090815260208190526040902054848110156110825760405162461bcd60e51b81526004018080602001828103825260258152602001806124f26025913960400191505060405180910390fd5b3360008181526020818152604080832089860390556001600160a01b038a168084529281902080548a0190558051898152905192939260008051602061257b833981519152929181900390910190a3506111ec565b33600090815260208190526040902054848110156111265760405162461bcd60e51b81526004018080602001828103825260218152602001806125bb6021913960400191505060405180910390fd5b3360008181526020818152604080832089860390558051898152905192939260008051602061257b833981519152929181900390910190a3604051600090339087908381818185875af1925050503d80600081146111a0576040519150601f19603f3d011682016040523d82523d6000602084013e6111a5565b606091505b50509050806111e9576040805162461bcd60e51b81526020600482015260196024820152600080516020612537833981519152604482015290519081900360640190fd5b50505b846001600160a01b031663a4c0ed36338686866040518563ffffffff1660e01b815260040180856001600160a01b03168152602001848152602001806020018281038252848482818152602001925080828437600081840152601f19601f82011690508083019250505095505050505050602060405180830381600087803b15801561127757600080fd5b505af115801561128b573d6000803e3d6000fd5b505050506040513d60208110156112a157600080fd5b505195945050505050565b33600090815260208190526040902054818110156112fb5760405162461bcd60e51b81526004018080602001828103825260218152602001806125bb6021913960400191505060405180910390fd5b336000818152602081815260409182902085850390558151858152915130939260008051602061257b83398151915292908290030190a350600480549091019055565b60006001600160a01b038516301461139d576040805162461bcd60e51b815260206004820152601c60248201527f574554483a20666c617368206d696e74206f6e6c792057455448313000000000604482015290519081900360640190fd5b6001600160701b038411156113e35760405162461bcd60e51b81526004018080602001828103825260248152602001806125576024913960400191505060405180910390fd5b600380548501908190556001600160701b031015611448576040805162461bcd60e51b815260206004820152601f60248201527f574554483a20746f74616c206c6f616e206c696d697420657863656564656400604482015290519081900360640190fd5b6001600160a01b0386166000818152602081815260408083208054890190558051888152905160008051602061257b833981519152929181900390910190a37f0000000000000000000000000000000000000000000000000000000000000000866001600160a01b03166323e30c8b333088600089896040518763ffffffff1660e01b815260040180876001600160a01b03168152602001866001600160a01b03168152602001858152602001848152602001806020018281038252848482818152602001925080828437600081840152601f19601f820116905080830192505050975050505050505050602060405180830381600087803b15801561154d57600080fd5b505af1158015611561573d6000803e3d6000fd5b505050506040513d602081101561157757600080fd5b5051146115cb576040805162461bcd60e51b815260206004820152601760248201527f574554483a20666c617368206c6f616e206661696c6564000000000000000000604482015290519081900360640190fd5b6001600160a01b0386166000908152600260209081526040808320308452909152902054600019811461168a578481101561163b576040805162461bcd60e51b815260206004820152601f6024820152600080516020612517833981519152604482015290519081900360640190fd5b6001600160a01b03871660008181526002602090815260408083203080855290835292819020898603908190558151818152915190949260008051602061259b833981519152928290030190a3505b6001600160a01b038716600090815260208190526040902054858110156116e25760405162461bcd60e51b81526004018080602001828103825260218152602001806125bb6021913960400191505060405180910390fd5b6001600160a01b0388166000818152602081815260408083208a8603905580518a8152905192939260008051602061257b833981519152929181900390910190a3505060038054859003905550600195945050505050565b6001600160a01b03831660008181526020818152604080832080543490810190915581519081529051929392849260008051602061257b833981519152928290030190a3604051635260769b60e11b815233600482018181523460248401819052606060448501908152606485018790526001600160a01b0389169463a4c0ed36949389928992608401848480828437600081840152601f19601f82011690508083019250505095505050505050602060405180830381600087803b15801561180257600080fd5b505af1158015611816573d6000803e3d6000fd5b505050506040513d602081101561182c57600080fd5b5051949350505050565b60006001600160a01b038216301461184f57600061185c565b6003546001600160701b03035b92915050565b60006020819052908152604090205481565b60016020526000908152604090205481565b7f000000000000000000000000000000000000000000000000000000000000000081565b600480546001600160a01b038316600081815260208181526040808320805486019055919094558051838152905192939192309260008051602061257b833981519152928290030190a35050565b60035481565b6001600160a01b03831633146119cf576001600160a01b038316600090815260026020908152604080832033845290915290205460001981146119cd578181101561197e576040805162461bcd60e51b815260206004820152601f6024820152600080516020612517833981519152604482015290519081900360640190fd5b6001600160a01b03841660008181526002602090815260408083203380855290835292819020868603908190558151818152915190949260008051602061259b833981519152928290030190a3505b505b6001600160a01b03831660009081526020819052604090205481811015611a275760405162461bcd60e51b81526004018080602001828103825260218152602001806125bb6021913960400191505060405180910390fd5b6001600160a01b03841660008181526020818152604080832086860390558051868152905192939260008051602061257b833981519152929181900390910190a36040516000906001600160a01b0385169084908381818185875af1925050503d8060008114611ab3576040519150601f19603f3d011682016040523d82523d6000602084013e611ab8565b606091505b5050905080611b0e576040805162461bcd60e51b815260206004820152601b60248201527f574554483a204574686572207472616e73666572206661696c65640000000000604482015290519081900360640190fd5b5050505050565b6040518060400160405280600481526020016309a8aa8960e31b81525081565b6004805433600081815260208181526040808320805486019055919094558051838152905192939192309260008051602061257b833981519152928290030190a350565b60006001600160a01b03831615801590611b9c57506001600160a01b0383163014155b15611c45573360009081526020819052604090205482811015611bf05760405162461bcd60e51b81526004018080602001828103825260258152602001806124f26025913960400191505060405180910390fd5b3360008181526020818152604080832087860390556001600160a01b038816808452928190208054880190558051878152905192939260008051602061257b833981519152929181900390910190a350611d5a565b3360009081526020819052604090205482811015611c945760405162461bcd60e51b81526004018080602001828103825260218152602001806125bb6021913960400191505060405180910390fd5b3360008181526020818152604080832087860390558051878152905192939260008051602061257b833981519152929181900390910190a3604051600090339085908381818185875af1925050503d8060008114611d0e576040519150601f19603f3d011682016040523d82523d6000602084013e611d13565b606091505b5050905080611d57576040805162461bcd60e51b81526020600482015260196024820152600080516020612537833981519152604482015290519081900360640190fd5b50505b50600192915050565b6001600160a01b0381166000818152602081815260408083208054349081019091558151908152905160008051602061257b833981519152929181900390910190a350565b3360008181526002602090815260408083206001600160a01b0389168085529083528184208890558151888152915193949093909260008051602061259b833981519152928290030190a3846001600160a01b031662ba451f338686866040518563ffffffff1660e01b815260040180856001600160a01b03168152602001848152602001806020018281038252848482818152602001925080828437600081840152601f19601f82011690508083019250505095505050505050602060405180830381600087803b15801561127757600080fd5b7f000000000000000000000000000000000000000000000000000000000000000081565b336000818152602081815260408083208054349081019091558151908152905160008051602061257b833981519152929181900390910190a3565b83421115611f28576040805162461bcd60e51b815260206004820152601460248201527315d155120e88115e1c1a5c9959081c195c9b5a5d60621b604482015290519081900360640190fd5b6001600160a01b0380881660008181526001602081815260408084208054938401905580517f00000000000000000000000000000000000000000000000000000000000000008184015280820195909552948b166060850152608084018a905260a084019190915260c08084018990528451808503909101815260e09093019093528151919092012046917f00000000000000000000000000000000000000000000000000000000000000008314611fe857611fe383612420565b61200a565b7f00000000000000000000000000000000000000000000000000000000000000005b82604051602001808061190160f01b81525060020183815260200182815260200192505050604051602081830303815290604052805190602001209050600060018288888860405160008152602001604052604051808581526020018460ff1681526020018381526020018281526020019450505050506020604051602081039080840390855afa1580156120a3573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b038116158015906120d957508a6001600160a01b0316816001600160a01b0316145b612121576040805162461bcd60e51b815260206004820152601460248201527315d155120e881a5b9d985b1a59081c195c9b5a5d60621b604482015290519081900360640190fd5b6001600160a01b03808c166000818152600260209081526040808320948f16808452948252918290208d905581518d8152915160008051602061259b8339815191529281900390910190a35050505050505050505050565b60006001600160a01b03831630146121d8576040805162461bcd60e51b815260206004820152601c60248201527f574554483a20666c617368206d696e74206f6e6c792057455448313000000000604482015290519081900360640190fd5b50600092915050565b600260209081526000928352604080842090915290825290205481565b6004548181101561220e57600080fd5b6001600160a01b0383166000818152602081815260409182902080548601905584840360045581518581529151309260008051602061257b83398151915292908290030190a3505050565b6001600160a01b038216331461232a576001600160a01b0382166000908152600260209081526040808320338452909152902054600019811461232857818110156122d9576040805162461bcd60e51b815260206004820152601f6024820152600080516020612517833981519152604482015290519081900360640190fd5b6001600160a01b03831660008181526002602090815260408083203380855290835292819020868603908190558151818152915190949260008051602061259b833981519152928290030190a3505b505b6001600160a01b038216600090815260208190526040902054818110156123825760405162461bcd60e51b81526004018080602001828103825260218152602001806125bb6021913960400191505060405180910390fd5b6001600160a01b0383166000818152602081815260409182902085850390558151858152915130939260008051602061257b83398151915292908290030190a35060048054909101905550565b600454818110156123df57600080fd5b336000818152602081815260409182902080548601905584840360045581518581529151309260008051602061257b83398151915292908290030190a35050565b604080518082018252600881526709acaec40aecae8d60c31b6020918201528151808301835260018152603160f81b9082015281517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818301527fa3b25ed7788bb6df2d164c238f0a3c29680e8a8d21b2285bbc420a79a43f1688818401527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc6606082015260808101939093523060a0808501919091528251808503909101815260c090930190915281519101209056fe574554483a207472616e7366657220616d6f756e7420657863656564732062616c616e6365574554483a2072657175657374206578636565647320616c6c6f77616e636500574554483a20455448207472616e73666572206661696c656400000000000000574554483a20696e646976696475616c206c6f616e206c696d6974206578636565646564ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925574554483a206275726e20616d6f756e7420657863656564732062616c616e6365a2646970667358221220ddc664129365bd5d90a96b819e4f4df73a30f1c7886c7fbf66462fa789aab92d64736f6c63430007060033"

export FOUNDRY_PROFILE=tests
export FOUNDRY_GAS_PRICE=1000000000
export FOUNDRY_BASE_FEE_PER_GAS=1000000000

forge test "$@"