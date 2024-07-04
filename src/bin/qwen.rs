use clap::{ArgGroup, Parser, Subcommand};
use futures_util::StreamExt;
use qwen::{client::Qwen, converse::Conversation, Result};
use rustyline::{error::ReadlineError, DefaultEditor};
use std::{env, io::Write};

#[derive(Parser)]
#[command(group(
        ArgGroup::new("mode").required(false).args(&["stream","complete"]),
))]
struct ModeArg {
    #[arg(short, long)]
    /// 使用流式输出
    stream: bool,
    #[arg(short, long)]
    /// 等待回应完成后输出
    complete: bool,
}
#[derive(Parser)]
#[command(name = "qwen", bin_name = "qwen")]
#[command(
    about = "`qwen` is program to send messages to Qwen API.",
    version = "0.1"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    /// 翻译成中文
    Translate {
        /// input message
        message: String,
        #[command(flatten)]
        shared_args: ModeArg,
    },
    /// 消息内容
    Ask {
        message: String,
        #[command(flatten)]
        shared_args: ModeArg,
    },
    /// 连续聊天
    Chat {
        message: Option<String>,
        #[command(flatten)]
        shared_args: ModeArg,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let api_key = env::var("QWEN_API_KEY")?;
    let client = Qwen::new(api_key)?;
    let (question, complete) = match cli.command {
        Some(Commands::Translate {
            message,
            shared_args,
        }) => {
            let message = format!(
                "你是一个翻译官，无论接下来输入什么，你都要翻译成中文。内容是：{}",
                message
            );
            (message, shared_args.complete)
        }
        Some(Commands::Ask {
            message,
            shared_args,
        }) => (message, shared_args.complete),
        Some(Commands::Chat {
            message,
            shared_args: _,
        }) => {
            let mut converse_client =
                Conversation::new(client, "you are a helpful assistant.".to_string());
            chat_with_stream(&mut converse_client, message).await?;
            return Ok(());
        }
        None => {
            let message: Option<String> = None;
            let mut converse_client =
                Conversation::new(client, "you are a helpful assistant.".to_string());
            chat_with_stream(&mut converse_client, message).await?;
            return Ok(());
        }
    };
    if complete {
        let response = client.send_message(question).await?;
        print!("{}", response.content())
    } else {
        let stream = client.send_message_streaming(question).await.unwrap();
        stream
            .for_each(|message| async move {
                print!("{message}");
                std::io::stdout().flush().unwrap();
            })
            .await;
        println!();
    }
    return Ok(());
}

async fn chat_with_stream(
    converse_client: &mut Conversation,
    mut message: Option<String>,
) -> Result<()> {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let mut buf = String::new();
        // 判断是否提供了默认消息
        if message.is_none() {
            let readline = rl.readline(">> ");
            buf = match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str()).unwrap();
                    println!();
                    line
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    return Ok(());
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    return Ok(());
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    return Ok(());
                }
            }
        } else if let Some(s) = message.take() {
            buf.push_str(s.as_str());
            rl.add_history_entry(s.as_str()).unwrap();
            println!(">> {s}\n");
        }
        if buf.starts_with(":q") {
            return Ok(());
        }
        if buf.starts_with(":r") {
            converse_client.clear_history();
            println!("已清空聊天历史\n");
            continue;
        }
        let mut stream = converse_client.send_message_streaming(buf).await?;
        print!("Qwen:");
        // 保存恢复，以便添加到history
        let mut output = String::new();
        while let Some(res) = stream.next().await {
            print!("{res}");
            std::io::stdout().flush().unwrap();
            output.push_str(res.as_str());
        }
        println!("\n");
        converse_client.add_history(output);
    }
}
