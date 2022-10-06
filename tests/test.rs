use near_sdk::log;
//use near_units::parse_near;
use serde_json;


const WASM_FILEPATH: &str = "imageboard.wasm";

//const BLOCK_HEIGHT: BlockHeight = 102001114;


#[tokio::test]
async fn deploy() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(WASM_FILEPATH).unwrap();
    let contract = worker.dev_deploy(&wasm).await?;
    let account = worker.dev_create_account().await?;

    contract.
        call("new").
        args_json(serde_json::json!({
            "owner": account.id()
        })).
        transact().
        await?.
        into_result()?;

    /*let owner_id = contract //shis shit generic error coonection to RPC
        .call("get_owner")
        .view()
        .await?;
    let downer = owner_id.borsh::<String>()?;
    let downer = contract.call("get_owner").view().await?;
    log!("from contract: {:?}", downer);*/

    contract.
        call("add_thread").
        args_json(serde_json::json!({
            "text": "we are all gonna die!".to_string()
        })).
        transact().
        await?.
        into_result()?;
    let number = 1 as i32;
    let thread = contract.
                    call("get_the_thread").
                    args_json((number,)).
                    view().
                    await?;
    log!("from contract: {:?}", thread.json::<String>()?);

    assert_eq!("we are all gonna die!".to_string(), thread.json::<String>()?);



    Ok(())
}   
