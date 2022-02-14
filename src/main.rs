use teloxide::prelude2::*;
use regex::Regex;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");
    let token = "";
    let bot = Bot::new(token).auto_send();
    
    // listen to any kind of message that matches the URL regex and return the URL
    teloxide::repls2::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        match message.text() {
            Some(text) => {
                let re = Regex::new(r"((https?://)?([a-zA-Z]+.)?twitter.com/[a-zA-Z0-9_]+/status/[0-9]+)\?[a-zA-Z]=[a-zA-Z0-9_]+(&[a-zA-Z]=[a-zA-Z0-9_]+)*").unwrap();
                if re.is_match(&text) {
                    let mut restext = String::from("不潔貓！清理后的 link 是:");
                    for cap in re.captures_iter(text) {
                        println!("Clean link: {}", &cap[1]);
                        restext = format!("{}\n{}", restext, &cap[1]);
                    }
                    bot.send_message(message.chat.id, restext).await?;
                } else {
                    bot.send_message(message.chat.id, "No URL found").await?;
                }
            }
            None => {
                
            }
        };
        respond(())
    }).await;
}