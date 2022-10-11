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

    let subaccount: Account = account.
                                create_subaccount("lahtabot1").
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

    contract.
        call("add_thread").
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
    
    let all_threads:Vec<(i32, String, String)> = contract.
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
let add_moder = contract.
        call("add_moder").
        args_json(serde_json::json!({
            "user_id": subaccount2.id(),
        })).
        transact().
        await?.
        into_result()?;
        
log!("add moder{:?}", add_moder.json::<String>()?); //can't check this cos signer_acc is not owner and idk why.

/*let list_mods: Vec<String> = contract.
        call("get_moders").
        view().
        await?.
        json()?;

    log!("list_bans: {:?}", list_mods);*/

            
    /*contract.
            call("ban").
            args_json(serde_json::json!({
                "user": subaccount.id(),
            })).
            transact().
            await?.
            into_result()?;
    let list_bans: Vec<String> = contract.
                        call("get_bans").
                        view().
                        await?.
                        json()?;
    
    log!("account {:?}", subaccount.id().to_string());
    log!("list_bans: {:?}", list_bans);
    
    let ban_check: String = contract.
                        call("is_banned").
                        args_json(serde_json::json!({
                                "name": subaccount.id()
                        })).
                        view().
                        await?.
                        json()?;

    
    log!("Banned1? {:?}", ban_check);

    contract.
        call("remove_ban").
        args_json(serde_json::json!({
            "user":subaccount.id()
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
    
    
    let add_moder = contract.
        call("add_moder").
        args_json(serde_json::json!({
            "user_id": subaccount2.id(),
        })).
        transact().
        await?.
        into_result()?;
    log!("add moder{:?}", add_moder);

    let list_mods: Vec<String> = contract.
        call("get_moders").
        view().
        await?.
        json()?;

    log!("list_bans: {:?}", list_mods);  */  


    Ok(())

}   
