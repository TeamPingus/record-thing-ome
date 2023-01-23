use json::object;
use reqwest::{Body, Client};
use std::collections::HashMap;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

async fn startrecord() -> Result<()> {
    let data = object! {
    id: "stream",
    stream: {
      name: "stream"
            }
      };

    //let mut map = HashMap::new();
    //map.insert("id", "stream");
    //map.insert("name", "stream");

    let client = Client::new();
    let res = client
        .post("http://api.pingusmc.org:8081/v1/vhosts/default/apps/app:startRecord")
        .header("authorization", "Basic b21lLWFjY2Vzcy10b2tlbg==")
        .body(Body::from(data.dump()))
        .send()
        .await?;
    println!("{}", res.status());

    let body = res.bytes().await?;

    let v = body.to_vec();
    let s = String::from_utf8_lossy(&v);
    println!("Startrecord: response: {} ", s);

    Ok(())
}

async fn stoprecord() -> Result<()> {
    let mut map = HashMap::new();
    map.insert("id", "stream");

    let client = Client::new();
    let res = client
        .post("http://api.pingusmc.org:8081/v1/vhosts/default/apps/app:stopRecord")
        .header("authorization", "Basic b21lLWFjY2Vzcy10b2tlbg==")
        .json(&map)
        .send()
        .await?;
    println!("{}", res.status());

    let body = res.bytes().await?;

    let v = body.to_vec();
    let s = String::from_utf8_lossy(&v);
    println!("Stoprecord: response: {} ", s);

    Ok(())
}

async fn records() -> Result<()> {
    let mut map = HashMap::new();
    map.insert("id", "");

    let client = Client::new();
    let res = client
        .post("http://api.pingusmc.org:8081/v1/vhosts/default/apps/app:records")
        .header("authorization", "Basic b21lLWFjY2Vzcy10b2tlbg==")
        .json(&map)
        .send()
        .await?;
    println!("{}", res.status());

    let body = res.bytes().await?;

    let v = body.to_vec();
    let s = String::from_utf8_lossy(&v);
    println!("Records: response: {} ", s);

    Ok(())
}
#[tokio::main]
async fn main() -> Result<()> {
    //let args: Vec<String> = env::args().collect();
    startrecord().await?;
    stoprecord().await?;
    records().await?;

    Ok(())
}
//
