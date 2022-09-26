use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstMsg {
  
}

#[cw_serde]
pub enum ExecuteMsg {
   
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    

}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
   
}
