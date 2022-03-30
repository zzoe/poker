use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("手牌不能为空")]
    Empty,
    #[error("Invalid card value: {0}")]
    InvalidCardValue(String),
}
