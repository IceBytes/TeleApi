## Tele Api

> Example for use

``` rust
use TeleApi; 

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let bot = TeleApi::TeleApi::new(""); // Telegram bot token

    let chat_id: i64 = 123456789;
    let text = "Hi TeleApi";

    bot.sendMessage(params!("chat_id" => chat_id, "text" => text)).await?;

    Ok(())
}

```

### Community

- Join the telegram channel: https://t.me/tshreeb_programming
"# TeleApi" 
