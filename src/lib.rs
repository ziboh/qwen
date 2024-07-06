#![doc = include_str!("../README.md")]
/// 包含了一些Qwen客户端功能
pub mod client;
/// 包含一些Api配置
pub mod config;
/// 常见的一些错误
pub mod err;
/// http请求和回应的数据结果
pub mod types;
/// 包含了会话功能的函数
pub mod converse;

pub type Result<T> = std::result::Result<T, err::Error>;

#[cfg(test)]
mod test {
    use std::env;

    use futures_util::StreamExt;

    use super::*;
    #[tokio::test]
    async fn test_complete() {
        let api_key = env::var("QWEN_API_KEY").unwrap();
        let qwen_client = client::Qwen::new(api_key).unwrap();
        let res = qwen_client.send_message("1+1=").await.unwrap().content();
        assert_eq!("2".to_string(), res.trim())
    }

    #[tokio::test]
    async fn test_stream(){
        let api_key = env::var("QWEN_API_KEY").unwrap();
        let qwen_client = client::Qwen::new(api_key).unwrap();
        let mut stream = qwen_client.send_message_streaming("1+1=").await.unwrap();
        let mut output = String::new();
        while let Some(res) = stream.next().await{
            output.push_str(res.as_str());
        }
        assert_eq!("2".to_string(), output.trim())
    }
}
