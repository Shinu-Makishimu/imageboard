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
    let account2: Account = worker.dev_create_account().await?;
    let account3: Account = worker.dev_create_account().await?;
    
    

    let subaccount: Account = account.
                                create_subaccount("lahtabot1").
                                initial_balance(near_units::parse_near!("5")).
                                transact().
                                await?.
                                into_result()?;


    let subaccount2: Account = account.
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

    account.
        call(contract.id(), "add_thread").
        args_json(serde_json::json!({
            "text": random_thread_string
        })).
        transact().
        await?.
        into_result()?;

    
    let number: i32 = 1 ;
    let thread: String = contract.
                    call("get_the_thread").
                    args_json((number,)).
                    view().
                    await?.
                    json()?;
    
    log!("generated: {:?}", random_thread_string);
    log!("from contract: {:?}", thread);

    assert_eq!(random_thread_string, thread);


//////////////////////////////////////////////////////

    account2.
        call(contract.id(),"add_thread").
        args_json(serde_json::json!({
            "text": common::generate_random_string()
        })).
        transact().
        await?.
        into_result()?;

    account3.
        call(contract.id(),"add_thread").
        args_json(serde_json::json!({
            "text": common::generate_random_string()
        })).
        transact().
        await?.
        into_result()?;
    subaccount.
        call(contract.id(),"add_thread").
        args_json(serde_json::json!({
            "text": common::generate_random_string()
        })).
        deposit(near_units::parse_near!("1")).
        transact().
        await?.
        into_result()?;

    
    let thread_count:i32 = contract.
                        call("get_count").
                        view().
                        await?.
                        json()?;

    log!("thread count: {:?}", thread_count);
    
    let all_threads:Vec<(i32, String, String, bool)> = contract.
                            call("get_threads").
                            view().
                            await?.
                            json()?;
    log!("all threads: {:?}", all_threads);

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
    
    let check2 = contract. 
        call("add_answers").
        args_json(serde_json::json!({
            "thread_number": number,
            "text": random_answer_string,
        })).
        transact().
        await?.
        into_result()?;

    log!("add answ. status: {:?}", check2.json::<String>()?);


    let answer: String = contract.
            call("get_thread_answers").
            args_json(serde_json::json!({
                "thread_number": number,
            })).
            view().
            await?.
            json()?;

    log!("from thread answ = {:?}", answer);

/////////////////////////////////////////////////////  
    log!("{:?}", subaccount2.id());

    let add_moder = account.
        call(contract.id(),"add_moder").
        args_json(serde_json::json!({
            "user_id": account2.id(),
        })).
        transact().
        await?.
        into_result()?;
        
    log!("add moder{:?}", add_moder.json::<String>()?); 

    let list_mods: Vec<String> = contract.
        call("get_moders").
        view().
        await?.
        json()?;

    log!("list_moders: {:?}", list_mods);

            
    account.
            call(contract.id(), "ban").
            args_json(serde_json::json!({
                "user": account3.id(),
            })).
            transact().
            await?.
            into_result()?;
    let list_bans: Vec<String> = contract.
                        call("get_bans").
                        view().
                        await?.
                        json()?;
    
    log!("list_bans: {:?}", list_bans);
    
    let check = account3.
        call(contract.id(), "add_thread").
        args_json(serde_json::json!({
            "text": random_thread_string
        })).
        transact().
        await?.
        into_result()?;

    
        log!("check: {:?}", check);


    Ok(())

}   
