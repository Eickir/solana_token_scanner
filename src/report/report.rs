use contract::ContractAnalysis;
use holders::Holders;
use overview::Overview;

#[derive(Debug)]
struct TokenReport {
    pub contract_analysis: ContractAnalysis, 
    pub token_overview: Overview, 
    pub holders: Holders,
}

impl TokenReport {

    pub fn new(contract_analysis: ContractAnalysis, token_overview: Overview, holders: Holders) -> Self {
        Self {
            contract_analysis,
            token_overview,
            holders,
        }
    }

}