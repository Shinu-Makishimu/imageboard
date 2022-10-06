//use near_sdk::BlockHeight;
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

    contract
        .call("new")
        .args_json(serde_json::json!({
            "owner": account.id()
        }))
        .transact()
        .await?
        .into_result()?;

    let owner_id = contract
        .call("get_owner")
        .view()
        .await?;
    let downer = owner_id.json::<String>()?;
    assert_eq!(account.id().to_string(), downer);
    Ok(())

}   
