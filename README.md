## Tele Api

> Example for use

``` rust
use TeleApi; 

#[tokio::main]
async fn main() {
    let bot = TeleApi::new(""); // Telegram bot token 

    let chat_id: i64 = 12345678; // User id
    let text = "Hi";
    
    let text = "Click me";
    let keyboard = serde_json::json!({
        "keyboard": [[{
            "text": text
        }]],
        "one_time_keyboard": true,
        "resize_keyboard": true
    });

    let keyboard_str = keyboard.to_string();

    bot.sendMessage(
        &[
            ("chat_id", &chat_id.to_string()),
            ("text", text),
            ("reply_markup", &keyboard_str),
        ]
    ).await.unwrap();

    bot.setMyName(&[("name", "Test")]).await.unwrap();
}

```

### Community

- Join the telegram channel: https://t.me/tshreeb_programming

