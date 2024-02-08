// use ethers::types::Address;
// use std::error::Error;
// use std::fs::File;
// use std::io;
// use std::io::prelude::*;
// use tokio::time::{sleep, Duration};
// // use csv::ReaderBuilder;
// use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

// const MAX_RETRIES: u32 = 5;
// const MAX_PROXY_CHECK_ATTEMPTS: u32 = 3;

// const WEBSITE_KEY: &str = "6LfOA04pAAAAAL9ttkwIz40hC63_7IsaU2MgcwVH";
// const WEBSITE_URL: &str = "https://artio.faucet.berachain.com";

// async fn airdrop_test_token(address: Address) -> Result<(), Box<dyn Error>> {
//     // let config_path = "../../config/runner.json";
//     // let wallet_path = // set the path to the wallet file

//     let addresses = process_addresses(wallet_path).await?;

//     println!("开始领取测试币");

//     let mut proxy_verified = false;
//     let mut proxy_attempts = 0;

//     while !proxy_verified && proxy_attempts < MAX_PROXY_CHECK_ATTEMPTS {
//         println!("测试代理IP是否正常");
//         match send_request("https://myip.ipip.net").await {
//             Ok(response) => {
//                 println!("验证成功, IP信息: {:?}", response);
//                 proxy_verified = true;
//             }
//             Err(_) => {
//                 proxy_attempts += 1;
//                 println!("代理失效，等待1分钟后重新验证");
//                 sleep(Duration::from_secs(60)).await;
//             }
//         }
//     }

//     if !proxy_verified {
//         println!("代理验证失败，无法继续执行任务");
//         return Ok(());
//     }

//     for address in addresses {
//         println!("领取地址: {}", address);

//         let mut attempts = 0;
//         while attempts < MAX_RETRIES {
//             match process_address(address.clone()).await {
//                 Ok(_) => {
//                     println!("领取成功✅，地址：{}", address);
//                     attempts = MAX_RETRIES;
//                 }
//                 Err(error) => {
//                     if error.to_string() == "Faucet is overloading, please try again" {
//                         attempts += 1;
//                         println!("地址{}正在重试第 {} 次...", address, attempts);
//                         sleep(Duration::from_secs(5)).await;
//                     } else {
//                         eprintln!("领取失败❌，地址：{}: {}", address, error);
//                         break;
//                     }
//                 }
//             }
//         }
//     }

//     Ok(())
// }

// // async fn process_addresses(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
// //     let mut addresses = Vec::new();

// //     let file = File::open(file_path)?;
// //     let mut rdr = ReaderBuilder::new().from_reader(file);

// //     for result in rdr.records() {
// //         let record = result?;
// //         let address = record.get(0).ok_or("No address in record")?;
// //         addresses.push(address.to_string());
// //     }

// //     println!("地址读取完毕");
// //     Ok(addresses)
// // }
