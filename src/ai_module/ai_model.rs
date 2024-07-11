use reqwest::Error;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug)]
pub struct Message(pub String);

pub trait AiModel {
    fn comment(
        &self,
        diff: String,
        model: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Message>, Error>> + '_>>;
}
