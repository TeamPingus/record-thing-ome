extern crate getopts;
extern crate lazy_static;

use getopts::{HasArg, Occur, Options};
use json::object;
use reqwest::{Body, Client};
use std::collections::HashMap;
use std::env;
use std::ops::Add;
use std::path::Path;
use std::process::exit;
use crate::config::{CONFIG, create_config};

mod config;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

// api stuff (send help btw)

async fn startrecord() -> Result<()> {
    let data = object! {
    id: "stream",
    stream: {
      name: "stream"
            },
        segmentationRule : "continuity"
      };

    let domain = &CONFIG.domain.clone().add("/v1/vhosts/default/apps/app:startRecord");
    let token = &CONFIG.token;

    let client = Client::new();
    let res = client
        .post(domain)
        .header("authorization", token)
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

    let domain = &CONFIG.domain.clone().add("/v1/vhosts/default/apps/app:stopRecord");
    let token = &CONFIG.token;

    let client = Client::new();
    let res = client
        .post(domain)
        .header("authorization", token)
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

    let domain = &CONFIG.domain.clone().add("/v1/vhosts/default/apps/app:records");
    let token = &CONFIG.token;

    let client = Client::new();
    let res = client
        .post(domain)
        .header("authorization", token)
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

fn iwanttofuckingdie() {
    let domain = &CONFIG.domain;
    let token = &CONFIG.token;

    if domain.contains("DOMAIN") {
        println!("\x1b[0;31myou might want to consider replacing 'DOMAIN' in stuff.conf with your actual api domain\x1b[0m");
        exit(1);
    }
    if token.contains("TOKEN") {
        println!("\x1b[0;31myou might want to consider replacing 'TOKEN' in stuff.conf with your actual api token\x1b[0m");
        exit(1);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // create config if doesnt exist
    if Path::new("stuff.conf").exists() {
        println!("Using config: stuff.conf");
    } else {
        create_config().expect("failed to create config");
    }
    iwanttofuckingdie();
    // cli stuff
    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} [options]", program);
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
    opts.opt(
        "c",
        "config",
        "config location",
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
    if matches.opt_present("c") {
        println!("Not implemented yet, deal with it");
    }
    Ok(())
}

//
