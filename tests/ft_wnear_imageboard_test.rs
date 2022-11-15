use std::collections::HashMap;
use std::convert::TryInto;
use near_sdk::log;
use near_units::{parse_gas, parse_near};
use workspaces::network::Sandbox;
use workspaces::{Account, AccountId, Contract, Worker};
use workspaces::{BlockHeight, DevNetwork};
const REF_FINANCE_ACCOUNT_ID: &str = "v2.ref-finance.near";
const WASM_FILEPATH: &str = "imageboard.wasm";

const FT_WASM_FILEPATH: &str = "res/fungible_token.wasm";

const BLOCK_HEIGHT: BlockHeight = 50_000_000;


//mod common;


async fn create_imageboard(
    owner: &Account,
    worker: &Worker<impl DevNetwork>,
) -> anyhow::Result<Contract> {
    let gp: Contract = worker
        .dev_deploy(&std::fs::read(WASM_FILEPATH)?)
        .await?;

    gp.call("new")
        .args_json(serde_json::json!({
            "owner": owner.id(),
        }))
        .transact()
        .await?
        .into_result()?;
    ();

    Ok(gp)
}


async fn create_ft(
        owner: &Account, 
        worker: &Worker<Sandbox>
    )-> anyhow::Result<Contract> {
    
        let ft: Contract = worker
        .dev_deploy(&std::fs::read(FT_WASM_FILEPATH)?)
        .await?;

    // Initialize our FT contract with owner metadata and total supply available
    // to be traded and transfered into other contracts such as Ref-Finance
    ft.call("new_default_meta")
        .args_json(serde_json::json!({
            "owner_id": owner.id(),
            "total_supply": parse_near!("4200 N").to_string(),
        }))
        .transact()
        .await?
        .into_result()?;
    ();

    Ok(ft)
}



async fn create_ref(owner: &Account, worker: &Worker<Sandbox>) -> anyhow::Result<Contract> {
    let mainnet = workspaces::mainnet_archival().await?;
    let ref_finance_id: AccountId = REF_FINANCE_ACCOUNT_ID.parse()?;

    // This will pull down the relevant ref-finance contract from mainnet. We're going
    // to be overriding the initial balance with 1000N instead of what's on mainnet.
    let ref_finance = worker
        .import_contract(&ref_finance_id, &mainnet)
        .initial_balance(parse_near!("1000 N"))
        .block_height(BLOCK_HEIGHT)
        .transact()
        .await?;

    // NOTE: We are not pulling down the contract's data here, so we'll need to initalize
    // our own set of metadata. This is because the contract's data is too big for the rpc
    // service to pull down (i.e. greater than 50mb).

    owner
        .call(ref_finance.id(), "new")
        .args_json(serde_json::json!({
            "owner_id": ref_finance.id(),
            "exchange_fee": 4,
            "referral_fee": 2,
        }))
        .transact()
        .await?
        .into_result()?;

    owner
        .call(ref_finance.id(), "storage_deposit")
        .args_json(serde_json::json!({}))
        .deposit(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    Ok(ref_finance)
}


async fn create_pool_with_liquidity(
    owner: &Account,
    ref_finance: &Contract,
    tokens: HashMap<&AccountId, u128>,
) -> anyhow::Result<u64> {
    let (token_ids, token_amounts): (Vec<String>, Vec<String>) = tokens
        .iter()
        .map(|(id, amount)| (id.to_string(), amount.to_string()))
        .unzip();
    log!("1");

    ref_finance
        .call("extend_whitelisted_tokens")
        .args_json(serde_json::json!({ "tokens": token_ids }))
        .transact()
        .await?
        .into_result()?;
    log!("2 token id {:?}", token_ids);

    // тут ломается
    let pool_id: u64 = ref_finance
        .call("add_simple_pool")
        .args_json(serde_json::json!({
            "tokens": token_ids,
            "fee": 25
        }))
        .deposit(parse_near!("3 N"))
        .transact()
        .await?
        .json()?;
    log!("3");

    owner
        .call(ref_finance.id(), "register_tokens")
        .args_json(serde_json::json!({
            "token_ids": token_ids,
        }))
        .deposit(1)
        .transact()
        .await?
        .into_result()?;
    log!("4");
    
    deposit_tokens(owner, &ref_finance, tokens).await?;

    owner
        .call(ref_finance.id(), "add_liquidity")
        .args_json(serde_json::json!({
            "pool_id": pool_id,
            "amounts": token_amounts,
        }))
        .deposit(parse_near!("1 N"))
        .transact()
        .await?
        .into_result()?;
    log!("5");

    Ok(pool_id)

    
}
async fn create_wnear(owner: &Account, worker: &Worker<Sandbox>) -> anyhow::Result<Contract> {
    let mainnet = workspaces::mainnet_archival().await?;
    let wnear_id: AccountId = "wrap.near".to_string().try_into()?;
    let wnear = worker
        .import_contract(&wnear_id, &mainnet)
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
        .args_json(serde_json::json!({}))
        .deposit(parse_near!("0.0125 N"))
        .transact()
        .await?
        .into_result()?;

    owner
        .call(wnear.id(), "near_deposit")
        .deposit(parse_near!("200 N"))
        .transact()
        .await?
        .into_result()?;

    Ok(wnear)
}



async fn deposit_tokens(
    owner: &Account,
    ref_finance: &Contract,
    tokens: HashMap<&AccountId, u128>,
) -> anyhow::Result<()> {
    for (contract_id, amount) in tokens {
        ref_finance
            .as_account()
            .call(contract_id, "storage_deposit")
            .args_json(serde_json::json!({
                "registration_only": true,
            }))
            .deposit(parse_near!("1 N"))
            .transact()
            .await?
            .into_result()?;

        owner
            .call(contract_id, "ft_transfer_call")
            .args_json(serde_json::json!({
                "receiver_id": ref_finance.id(),
                "amount": amount.to_string(),
                "msg": "",
            }))
            .gas(parse_gas!("200 Tgas") as u64)
            .deposit(1)
            .transact()
            .await?
            .into_result()?;
    }

    Ok(())
}

#[tokio::test]
async fn test_deploy() -> anyhow::Result<()> {
    //create worker and owner
    let worker: Worker<Sandbox> = workspaces::sandbox().await?;
    let main_owner: Account = worker.root_account()?;
    
    let owner: Account = main_owner.
        create_subaccount("anon").
        initial_balance(near_units::parse_near!("500 N")).
        transact().
        await?.
        into_result()?;


    // Deploy relevant contracts such as FT, and Ref-Finance

    let ft:Contract = create_ft(&owner, &worker).await?;
    log!("ft");

    let ref_finance:Contract = create_ref(&owner, &worker).await?;
    log!("rf");

    let ib: Contract = create_imageboard(&owner, &worker).await?;
    log!("ib");

    let wnear = create_wnear(&owner, &worker).await?;
    log!("wnear");

    let pool_id = create_pool_with_liquidity(
        &owner,
        &ref_finance,
        maplit::hashmap! {
            ft.id() => parse_near!("5 N"),
            wnear.id() => parse_near!("10 N"),

        },
    )
    .await?;


    log!(
        "Created a liquid pool on {} with id {}",
        ref_finance.id(),
        pool_id
    );

    deposit_tokens(
        &owner,
        &ref_finance,
        maplit::hashmap! {
            ft.id() => parse_near!("100 N"),
            wnear.id() => parse_near!("100 N"),

        },
    )
    .await?;

    //  View our deposited/transferred tokens in ref-finance

    let ft_deposit: String = worker
        .view(
            ref_finance.id(),
            "get_deposit",
            serde_json::json!({
                "account_id": owner.id(),
                "token_id": ft.id(),
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()?;
    log!("Current FT deposit: {}", ft_deposit);

let wnear_deposit: String = worker
    .view(
        ref_finance.id(),
        "get_deposit",
        serde_json::json!({
            "account_id": owner.id(),
            "token_id": wnear.id(),
        })
        .to_string()
        .into_bytes(),
    )
    .await?
    .json()?;

println!("Current WNear deposit: {}", wnear_deposit);

    //assert_eq!(ft_deposit, parse_near!("100 N").to_string());

    let register_result = owner
        .call(ft.id(), "storage_deposit")
        .args_json(serde_json::json!({
            "account_id": ib.id(),
            "registration_only": true,  // in wnear it has'nt any effect
        }))
        .deposit(parse_near!("0.00125 N"))
        .transact()
        .await?
        .into_result()?;

    log!("register result {:?}", register_result);


    Ok(())
}