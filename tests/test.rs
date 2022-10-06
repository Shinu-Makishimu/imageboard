use near_sdk::{log};
//use near_units::parse_near;
use serde_json;
use workspaces::{AccountId, Account, Contract, Worker, network::Sandbox};


const WASM_FILEPATH: &str = "imageboard.wasm";

//const BLOCK_HEIGHT: BlockHeight = 102001114;


#[tokio::test]
async fn deploy() -> anyhow::Result<()> {
    let worker: Worker<Sandbox> = workspaces::sandbox().await?;
    let wasm: Vec<u8> = std::fs::read(WASM_FILEPATH).unwrap();
    let contract: Contract = worker.dev_deploy(&wasm).await?;
    let account: Account = worker.dev_create_account().await?;

    contract.
        call("new").
        args_json(serde_json::json!({
            "owner": account.id()
        })).
        transact().
        await?.
        into_result()?;

    let owner_id: AccountId = contract 
        .call("get_owner")
        .view()
        .await?
        .json()?;
    

    
    log!("from contract: {:?}", owner_id);
    log!("from account {:?}", account.id());

    assert_eq!(&owner_id, account.id());

    contract.
        call("add_thread").
        args_json(serde_json::json!({
            "text": "we are all gonna die!".to_string()
        })).
        transact().
        await?.
        into_result()?;
    let number: i32 = 1 ;
    let thread = contract.
                    call("get_the_thread").
                    args_json((number,)).
                    view().
                    await?;
    log!("from contract: {:?}", thread.json::<String>()?);

    assert_eq!("we are all gonna die!".to_string(), thread.json::<String>()?);



    Ok(())
}   
