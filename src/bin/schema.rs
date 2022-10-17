use cosmwasm_schema::write_api;

use iks_oks::msg::{ InitMsg, QueryMsg, HandleMsg};

fn main() {
    // write_api! {
    //     instantiate: InitMsg,
    //     execute: HandleMsg,
    //     query: QueryMsg,
    // }
    write_api! {
        instantiate: InitMsg,
        execute: HandleMsg,
        query: QueryMsg,
    }
}
