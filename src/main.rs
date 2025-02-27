extern crate getopts;

use getopts::{HasArg, Occur, Options};
use reqwest::Client;
use serde::Serialize;

// api stuff (send help btw)

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct Stream {
    name: String,
}

#[derive(Serialize, Debug)]
struct RecordStopCheck {
    id: String,
}

#[derive(Serialize, Debug)]
struct RecordThingi {
    id: String,
    stream: Option<Stream>,
}

impl RecordThingi {
    async fn start(url: String, token: String, id: String, name: String) -> Result<(), reqwest::Error> {
        let client = Client::new();
        let url = format!("{}/v1/vhosts/default/apps/app:startRecord", url);
        let stream = Stream { name };
        let body = RecordThingi { id, stream: Some(stream) };
        
        let res = client
            .post(url)
            .header("Authorization", token)
            .json(&body)
            .send()
            .await?;

        println!("{}", res.text().await?);

        Ok(())

    }
    async fn stop(url: String, token: String, id: String) -> Result<(), reqwest::Error> {
        let client = Client::new();
        let url = format!("{}/v1/vhosts/default/apps/app:stopRecord", url);
        let body = RecordStopCheck { id };

        let res = client
            .post(url)
            .header("Authorization", token)
            .json(&body)
            .send()
            .await?;

        println!("{}", res.text().await?);

        Ok(())
    }
    async fn check(url: String, token: String, id: String) -> Result<(), reqwest::Error> {
        let client = Client::new();
        let url = format!("{}/v1/vhosts/default/apps/app:records", url);
        let body = RecordStopCheck { id };
        
        let res = client
            .post(url)
            .header("Authorization", token)
            .json(&body)
            .send()
            .await?;

        println!("{}", res.text().await?);

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = dotenvy::var("DOMAIN").unwrap().to_string();
    let token = format!("Basic {}", dotenvy::var("TOKEN").unwrap());
    let id = dotenvy::var("RECORD_ID").unwrap().to_string();
    let name = dotenvy::var("RECORD_NAME").unwrap().to_string();
    // cli stuff
    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
    }

    let args: Vec<_> = std::env::args().collect();
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
        RecordThingi::start(url.clone(), token.clone(), id.clone(), name).await?;
    }
    if matches.opt_present("2") {
        RecordThingi::stop(url.clone(), token.clone(), id.clone()).await?;
    }
    if matches.opt_present("3") {
        RecordThingi::check(url, token, id).await?;
    }
    if matches.opt_present("c") {
        println!("Not implemented yet, deal with it");
    }
    Ok(())
}

//
