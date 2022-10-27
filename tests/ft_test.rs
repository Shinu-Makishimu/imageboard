use near_sdk::json_types::U128;
use near_sdk::{log};
use serde_json::json;
use std::convert::TryInto;
use near_sdk::ONE_YOCTO;
use anyhow::Ok;
use near_units::{parse_gas, parse_near};
use workspaces::network::Sandbox;
use workspaces::{Account, AccountId, Contract, Worker, DevNetwork};
use workspaces::BlockHeight;

const WASM_FILEPATH: &str = "imageboard.wasm";

const BLOCK_HEIGHT: BlockHeight = 50_000_000;


mod common;


async fn create_testname(
    owner: &Account,
    worker: &Worker<impl DevNetwork>,
) -> anyhow::Result<Contract> {
    let gp: Contract = worker
        .dev_deploy(&std::fs::read(WASM_FILEPATH)?)
        .await?;

    gp.call("new")
        .args_json(json!({
            "owner": owner.id(),
        }))
        .transact()
        .await?
        .into_result()?;
    ();

    Ok(gp)
}


async fn create_wnear(
        owner: &Account, 
        worker: &Worker<Sandbox>
    )-> anyhow::Result<Contract> {

    let testnet = workspaces::testnet_archival().await?;
    let wrap_id: AccountId = "wrap.testnet".to_string().try_into()?;

    let wnear = worker
        .import_contract(&wrap_id, &testnet)
        .block_height(BLOCK_HEIGHT)
        .transact()
        .await?;

    owner
        .call(wnear.id(), "new")
        .args_json(serde_json::json!({
            "owner_id": owner.id(),
            "total_supply": parse_near!("1000 N"),
        }))
        .transact()
        .await?
        .into_result()?;

    owner
        .call(wnear.id(), "storage_deposit")
        .args_json(json!({}))
        .deposit(parse_near!("0.00125 N"))
        .transact()
        .await?
        .into_result()?;

    owner
        .call(wnear.id(), "near_deposit")
        .deposit(parse_near!("20000 N"))
        .transact()
        .await?
        .into_result()?;
    Ok(wnear)
}




#[tokio::test]
async fn test_deploy() -> anyhow::Result<()> {
    //create worker and owner
    let worker: Worker<Sandbox> = workspaces::sandbox().await?;
    let owner: Account = worker.root_account()?;
    
    let subaccount: Account = owner.
        create_subaccount("anon0").
        initial_balance(near_units::parse_near!("5")).
        transact().
        await?.
        into_result()?;


    let subaccount1: Account = owner.
        create_subaccount("anon1").
        initial_balance(near_units::parse_near!("5")).
        transact().
        await?.
        into_result()?;

    let subaccount2: Account = owner.
        create_subaccount("anon2").
        initial_balance(near_units::parse_near!("5")).
        transact().
        await?.
        into_result()?;
