use near_sdk::{log};
//use near_units::parse_near;
use serde_json;
use workspaces::{AccountId, Account, Contract, Worker, network::Sandbox};

mod common;


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
    /*let answer = contract.
            call("get_thread_answers").
            args_json(serde_json::json!({
                "thread_number": number,
            })).
            view().
            await?;

    log!("from thread answ = {:?}", answer);*/
    

    /*let thread_answ = contract.
                call("get_thread_answers").
                args_json((number,)).
                view().
                await?;
    
    log!("generated: {:?}", random_answer_string);
    log!("from contract: {:?}", thread_answ.json::<Vec<String>>()?);
    

    let subaccount: Account = account.
                        create_subaccount("mocher").
                        transact().
                        await?.
                        into_result()?;


    contract.
        call("add_moder").
        args_json(serde_json::json!({
            "user_id": subaccount,
        })).
        transact().
        await?.
        into_result()?;*/

    Ok(())

}   
