use clap::Parser;
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

// arg struct
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, help = "Start Recording")]
    start_record: bool,
    #[arg(short, long, help = "Stop Recording")]
    end_record: bool,
    #[arg(short, long, help = "Check Recording")]
    check_record: bool,
}

// api stuff (send help btw)

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct Stream {
    name: String,
}

#[derive(Serialize, Debug)]
struct RecordThingi {
    id: String,
    stream: Option<Stream>,
}

impl RecordThingi {
    async fn start(
        url: String,
        token: String,
        id: String,
        name: String,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/v1/vhosts/default/apps/app:startRecord", url);
        let stream = Stream { name };
        let body = RecordThingi {
            id,
            stream: Some(stream),
        };

        Self::request(url, token, body).await?;

        Ok(())
    }
    async fn stop(url: String, token: String, id: String) -> Result<(), reqwest::Error> {
        let url = format!("{}/v1/vhosts/default/apps/app:stopRecord", url);
        let body = RecordThingi { id, stream: None };

        Self::request(url, token, body).await?;

        Ok(())
    }
    async fn check(url: String, token: String, id: String) -> Result<(), reqwest::Error> {
        let url = format!("{}/v1/vhosts/default/apps/app:records", url);
        let body = RecordThingi { id, stream: None };

        Self::request(url, token, body).await?;

        Ok(())
    }

    async fn request<T: Serialize>(
        url: String,
        token: String,
        body: T,
    ) -> Result<String, reqwest::Error> {
        let client = Client::new();
        let res = client
            .post(url)
            .header("Authorization", token)
            .json(&body)
            .send()
            .await?
            .json::<Value>()
            .await?;

        let res = serde_json::to_string_pretty(&res).unwrap();
        println!("{}", res);

        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = dotenvy::var("DOMAIN").unwrap().to_string();
    let token = format!("Basic {}", dotenvy::var("TOKEN").unwrap());
    let id = dotenvy::var("RECORD_ID").unwrap().to_string();
    let name = dotenvy::var("RECORD_NAME").unwrap().to_string();

    let args = Cli::parse();

    if args.start_record && args.end_record {
        println!("Error: Cannot start and stop recording simultaneously.");
        return Ok(());
    }

    if args.start_record {
        RecordThingi::start(url.clone(), token.clone(), id.clone(), name).await?;
    }
    if args.end_record {
        RecordThingi::stop(url.clone(), token.clone(), id.clone()).await?;
    }
    if args.check_record {
        RecordThingi::check(url, token, id).await?;
    }

    Ok(())
}