use std::cell::RefCell;
use std::env;
use std::io::Write;
use std::rc::Rc;

use qwen::{client::Qwen, converse::Conversation, Result};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("QWEN_API_KEY").unwrap();
    let client = Qwen::new(api_key).unwrap();
    let mut converse_client = Conversation::new(client, "You are a helpful assistant.".to_string());
    loop {
        let ouput = Rc::new(RefCell::new(String::new()));
        let mut buf = String::new();
        print!("User:");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("Faild ot read line");
        if buf.starts_with(":q") {
            break;
        }
        let res = converse_client.send_message_streaming(buf).await?;
        print!("Qwen:");
        res.for_each(|s| {
            let o = Rc::clone(&ouput);
            async move {
                print!("{s}");
                std::io::stdout().flush().unwrap();
                o.borrow_mut().push_str(s.as_str());
            }
        })
        .await;
        println!();
        converse_client.add_history(ouput.as_ref().borrow().clone());
    }
    Ok(())
}
