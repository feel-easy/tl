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
    let text = if args.text.is_empty() {
        use std::io::{self, Read};
        let mut buffer = String::new();
        if atty::is(atty::Stream::Stdin) {
            println!("请提供要翻译的文本，例如: tl hello world 或 echo hello | tl");
            return Ok(());
        }
        io::stdin().read_to_string(&mut buffer)?;
        buffer.trim().to_string()
    } else {
        args.text.join(" ")
    };
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