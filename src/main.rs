use dotenv::dotenv;
use ethers::{
    providers::{Middleware, Provider, StreamExt, Ws},
    types::H256,
};
use std::{env, sync::Arc, time::Duration, vec};
use tokio::task::JoinHandle;
pub mod messages_handler;
use messages_handler::Handler;
use serenity::{futures, prelude::*};

#[tokio::main]
async fn main() {
    let mut thread_handles: Vec<JoinHandle<()>> = vec![];
    dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    // let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // let intents = GatewayIntents::GUILD_MESSAGES
    //     | GatewayIntents::DIRECT_MESSAGES
    //     | GatewayIntents::MESSAGE_CONTENT;
    // let mut client = Client::builder(&token, intents)
    //     .event_handler(Handler)
    //     .await
    //     .expect("Err creating client");

    // if let Err(why) = client.start().await {
    //     println!("Client error: {:?}", why);
    // }

    let node_ws = env::var("WSS_NODE").expect("WSS Node endpoint is missing");
    let url = String::from(node_ws);
    let ws = Ws::connect(url).await.unwrap();
    let provider = Arc::new(Provider::new(ws).interval(Duration::from_millis(2000)));
    let p1 = provider.clone();
    let p2 = provider.clone();
    let transactions: Arc<Mutex<Vec<H256>>> = Arc::new(Mutex::new(vec![]));

    let t1 = transactions.clone();
    let t2 = transactions.clone();

    thread_handles.push(tokio::spawn(async move {
        let l1 = p1;
        let mut stream = l1.provider().subscribe_pending_txs().await.unwrap();
        while let Some(tx_hash) = stream.next().await {
            t1.lock().await.push(tx_hash);
            println!("New pending transaction: {:#?}", tx_hash);
        }
    }));

    thread_handles.push(tokio::spawn(async move {
        let l2 = p2;
        let mut stream = l2.watch_blocks().await.unwrap();
        while let Some(block) = stream.next().await {
            let block = provider.get_block(block).await.unwrap().unwrap();
            if block.transactions.len() > 0 {
                let mut index = 0;
                let mut indexes: Vec<usize> = vec![];
                for tx in t2.lock().await.iter() {
                    if block.transactions.contains(tx) {
                        println!("{:?} has been confirmed", tx);
                    }
                    indexes.push(index);
                    index = index + 1;
                }

                for i in indexes.into_iter().rev() {
                    t2.lock().await.remove(i);
                }
            }
        }
    }));

    let join_res = futures::future::join_all(thread_handles).await;
}
