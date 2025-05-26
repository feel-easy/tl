use clap::Parser;
use reqwest::Client;
use serde::Deserialize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Text to translate
    text: Vec<String>,
}

#[derive(Deserialize)]
struct YoudaoResp {
    translation: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let text = args.text.join(" ");
    let url = format!(
        "https://aidemo.youdao.com/trans?from=auto&to=auto&q={}",
        urlencoding::encode(&text)
    );
    let client = Client::new();
    let resp = client.get(&url).send().await?.json::<YoudaoResp>().await?;
    if let Some(trans) = resp.translation.get(0) {
        println!("{}", trans);
    } else {
        println!("[翻译失败]");
    }
    Ok(())
}