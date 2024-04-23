use starknet::accounts::Account;
use starknet::macros::felt;

mod graph;
mod retrieve;
mod types;
mod utils;
use tokio::time::{sleep, Duration};
use types::Token::*;

#[tokio::main]
async fn main() {
    let pairs = retrieve::decode_pair_data();
    let account = types::create_account();

    let (update_index, nonce) = retrieve::initialized_nonce(account.address()).await;
    let amount_in = felt!("500000000000000000");
    loop {
        sleep(Duration::from_millis(300)).await;
        let (update_index, nonce) =
            retrieve::update_nonce(account.address(), nonce, update_index).await;

        let raw_states = retrieve::retrieve().await;

        let dex_states = retrieve::compile_states(&pairs, &raw_states);

        let usdc = graph::task2(ETH, USDC, &pairs);
        let usdt = graph::task2(ETH, USDC, &pairs);
        let strk = graph::task2(ETH, STRK, &pairs);
        let dai = graph::task2(ETH, DAI, &pairs);

        let task_usdc = graph::bridge2(&account, nonce, &amount_in, ETH, usdc, &dex_states);
        let task_strk = graph::bridge2(&account, nonce, &amount_in, ETH, strk, &dex_states);
        let task_usdt = graph::bridge2(&account, nonce, &amount_in, ETH, usdt, &dex_states);
        let task_dai = graph::bridge2(&account, nonce, &amount_in, ETH, dai, &dex_states);
        tokio::join!(task_dai, task_strk, task_usdc, task_usdt);
    }
}

#[tokio::test]
async fn test1() {
    let pairs = retrieve::decode_pair_data();
    let r = retrieve::retrieve().await;
    //println!("{} \n{:?}",r.len(),r);
    let p = &pairs[17];
    let mut mapping = retrieve::compile_states(&pairs, &r);
    let s = mapping.get(p).unwrap();
    dbg!("{:?} {:?}", p, s);
    assert_eq!(true, true);
}
