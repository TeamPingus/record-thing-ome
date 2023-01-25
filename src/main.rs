extern crate getopts;
use getopts::{HasArg, Occur, Options};
use json::object;
use reqwest::{Body, Client};
use std::collections::HashMap;
use std::env;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

async fn startrecord() -> Result<()> {
    let data = object! {
    id: "stream",
    stream: {
      name: "stream"
            },
        segmentationRule : "continuity"
      };

    let client = Client::new();
    let res = client
        .post("<yourapidomain>/v1/vhosts/default/apps/app:startRecord")
         // Auth code from Server.xml in Base64
	.header("authorization", "Basic <your auth code>")
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
        .post("<yourapidomain>/v1/vhosts/default/apps/app:stopRecord")
         // Auth code from Server.xml in Base64
	.header("authorization", "Basic <your auth code>")
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
        .post("<yourapidomain>/v1/vhosts/default/apps/app:records")
         // Auth code from Server.xml in Base64
	.header("authorization", "Basic <your auth code>")
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
    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} program [options]", program);
        print!("{}", opts.usage(&brief));
    }

    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.opt(
        "1",
        "start",
        "start Recording",
        "",
        HasArg::No,
        Occur::Optional,
    );
    opts.opt(
        "2",
        "stop",
        "stop Recording",
        "",
        HasArg::No,
        Occur::Optional,
    );
    opts.opt(
        "3",
        "status",
        "see status of recording",
        "",
        HasArg::No,
        Occur::Optional,
    );
    opts.opt(
        "h",
        "help",
        "print this help",
        "",
        HasArg::No,
        Occur::Optional,
    );

    let matches = match opts.parse(&args[..]) {
        Ok(m) => m,
        Err(f) => {
            print!("hello");
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
    }

    if matches.opt_present("1") {
        startrecord().await?;
    }
    if matches.opt_present("2") {
        stoprecord().await?;
    }
    if matches.opt_present("3") {
        records().await?;
    }

    Ok(())
}

//
