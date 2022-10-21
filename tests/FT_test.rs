
use near_sdk::json_types::U128;
use near_sdk::{log};
use serde_json::json;
use workspaces::result::ExecutionResult;
use std::convert::TryInto;

use anyhow::Ok;
use near_units::{parse_gas, parse_near};
use workspaces::network::Sandbox;
use workspaces::{Account, AccountId, Contract, Worker, DevNetwork};
use workspaces::BlockHeight;

const WASM_FILEPATH: &str = "imageboard.wasm";

const BLOCK_HEIGHT: BlockHeight = 50_000_000;

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
            "total_supply": parse_near!("1,000,000,000 N"),
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
    //let owner_id = owner.id();
    
    //let wasm: Vec<u8> = std::fs::read(WASM_FILEPATH).unwrap();

    let wnear: Contract = create_wnear(&owner, &worker).await?;
    let imageboard: Contract = create_testname(&owner, &worker).await?;
    /*let imageboard: Contract = worker.
                                dev_deploy(&wasm).
                                await?;*/

    let register_result = owner
        .call(wnear.id(), "storage_deposit")
        .args_json(json!({
            "account_id": imageboard.id(),
            "registration_only": true,
        }))
        .deposit(parse_near!("0.00125 N"))
        .transact()
        .await?
        .into_result()?;






    /*imageboard.
        call("new").
        args_json(serde_json::json!({
            "owner": owner.id()
        })).
        transact().
        await?.
        into_result()?;*/

    let owner_id: AccountId = imageboard 
        .call("get_owner")
        .view()
        .await?
        .json()?;
    
    log!("from contract: {:?}", owner_id);
    log!("from account {:?}", owner.id());


    log!("wnear22{:?}", wnear.id());
    /*let transfer = owner.
                    call(wnear.id(), "ft_transer_call").
                    args_json(json!({
                        "receiver_id": owner_id,
                        "amount": parse_near!("420 N").to_string(),
                        "msg": "idk0what1i2must3write4here".to_string(),
                    })).
                    gas(parse_gas!("200 Tgas") as u64).
                    deposit(1).
                    transact().
                    await?.
                    into_result()?;*/


    let transfer2 = wnear.
                    call("ft_transfer_call").
                    args_json(json!({
                        "receiver_id": imageboard.id(),
                        "amount": parse_near!("420 N").to_string(),
                        "msg": "idk0what1i2must3write4here",
                    })).
                    gas(parse_gas!("200 Tgas") as u64).
                    deposit(1).
                    transact().
                    await?.
                    into_result()?;


    /*log!("transfer{:?}", transfer);
    let balanse:U128 = imageboard 
                            .call("get_balance")
                            .view()
                            .await?
                            .json()?;
    
    log!("balanse {:?}", balanse);*/
    Ok(())

}
