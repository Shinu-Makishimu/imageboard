
use std::collections::HashMap;
use std::convert::TryInto;

use near_units::{parse_gas, parse_near};
use workspaces::network::Sandbox;
use workspaces::{Account, AccountId, Contract, Worker};
use workspaces::{BlockHeight, DevNetwork};

const FT_CONTRACT_FILEPATH: &str = "res/fungible_token.wasm";





#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    let ft = create_ft(&owner, &worker).await?;
}




async fn create_ft(owner: &Account, worker: &Worker<Sandbox>) -> anyhow::Result<Contract> {
    /// Idea of this fn is creat Fungible token. 
    let ft: Contract = worker.dev_deploy(&std::fs::read(FT_CONTRACT_FILEPATH)?).await?;
    ft.
        call("new_default_meta").
        args_json(serde_json!({
            "owner_id": owner.id(),
            "total_supply": parse_near!("1,000,000,000 N".to_string()),
        })).
        transact().
        await?.
        into_result()?;
    ();
    Ok(ft);

}

