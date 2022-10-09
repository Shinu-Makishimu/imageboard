use std::vec;

use near_sdk::{log};
//use near_units::parse_near;
use serde_json;
use workspaces::{AccountId, Account, Contract, Worker, network::Sandbox, result::ExecutionResult};

mod common;


const WASM_FILEPATH: &str = "imageboard.wasm";

//const BLOCK_HEIGHT: BlockHeight = 102001114;


#[tokio::test]
async fn deploy() -> anyhow::Result<()> {
    let worker: Worker<Sandbox> = workspaces::sandbox().await?;
    let wasm: Vec<u8> = std::fs::read(WASM_FILEPATH).unwrap();
    let contract: Contract = worker.dev_deploy(&wasm).await?;
    let account: Account = worker.dev_create_account().await?;

    let subaccount: Account = account.
                                create_subaccount("lahtabot1").
                                transact().
                                await?.
                                into_result()?;

    let subaccount2: Account = account.
                                create_subaccount("lahtabot2").
                                transact().
                                await?.
                                into_result()?;

    let _subaccount2: Account = account.
                                create_subaccount("anon").
                                transact().
                                await?.
                                into_result()?;

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



    //////////////////////////////////////////////////////
    let random_thread_string: String = common::generate_random_string();

    contract.
        call("add_thread").
        args_json(serde_json::json!({
            "text": random_thread_string
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
    
    log!("generated: {:?}", random_thread_string);
    log!("from contract: {:?}", thread.json::<String>()?);

    assert_eq!(random_thread_string, thread.json::<String>()?);


//////////////////////////////////////////////////////

    contract.
        call("add_thread").
        args_json(serde_json::json!({
            "text": common::generate_random_string()
        })).
        transact().
        await?.
        into_result()?;

    contract.
        call("add_thread").
        args_json(serde_json::json!({
            "text": common::generate_random_string()
        })).
        transact().
        await?.
        into_result()?;
    
    let thread_count = contract.
                                                call("get_count").
                                                view().
                                                await?;

    log!("thread count: {:?}", thread_count.json::<i32>()?);
    


//////////////////////////////////////////////////////

    let random_answer_string: String = common::generate_random_string();

    let check = contract.
        call("add_answers").
        args_json(serde_json::json!({
            "thread_number": number,
            "text": random_answer_string,
        })).
        transact().
        await?.
        into_result()?;
    
    log!("add answ. status: {:?}", check.json::<String>()?); 
    
    let check2 = contract. // Here is kukojzjzj
        call("add_answers").
        args_json(serde_json::json!({
            "thread_number": number,
            "text": random_answer_string,
        })).
        transact().
        await?.
        into_result()?;

    log!("add answ. status: {:?}", check2.json::<String>()?);

/////////////////////////////////////////////////////    
    let answer = contract.
            call("get_thread_answers").
            args_json(serde_json::json!({
                "thread_number": number,
            })).
            view().
            await?.;

    log!("from thread answ = {:?}", answer);
    

            
    contract.
            call("ban").
            args_json(serde_json::json!({
                "user": subaccount.id(),
            })).
            transact().
            await?.
            into_result()?;
    
    let ban_check: String = contract.
                        call("is_banned").
                        args_json(serde_json::json!({
                                "name": subaccount2.id()
                        })).
                        view().
                        await?.
                        json()?;

    let ban_check2: String = contract.
                        call("is_banned").
                        args_json(serde_json::json!({
                                "name": subaccount.id()
                        })).
                        view().
                        await?.
                        json()?;



    log!("Banned1? {:?}", ban_check);
    log!("Banned2? {:?}", ban_check2);

    contract.
        call("remove_ban").
        args_json(serde_json::json!({
            "user":subaccount.id()
        })).
        transact().
        await?.
        into_result()?;
    log!("Banned1? {:?}", ban_check);
    
    
    /*contract.
        call("add_moder").
        args_json(serde_json::json!({
            "user_id": subaccount.id(),
        })).
        transact().
        await?.
        into_result()?;*/

    Ok(())

}   
