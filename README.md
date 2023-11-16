## Tele Api

> Example for use

``` rust
use TeleApi; 

#[tokio::main]
async fn main() {
    let mut bot = TeleApi::TeleApi::new(""); //Telegram bot token

    loop {
        let updates = bot.get_updates().await;
        for update in updates.unwrap().as_array().unwrap_or(&vec![]) {
            if let Some(message_text) = update["message"]["text"].as_str() {
                if message_text == "/start" {
                    bot.execute("sendMessage", &[
                        ("chat_id", update["message"]["chat"]["id"].to_string().as_str()),
                        ("text", "Hi"),
                    ]).await.unwrap();
                }
            }
        }
    }
}

```

### Community

- Join the telegram channel: https://t.me/tshreeb_programming
"# TeleApi" 
