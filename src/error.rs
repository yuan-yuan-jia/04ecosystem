use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Serialize json error: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("Error: {0:?}")]
    BigError(Box<BigError>),
    // 单个item包含的结构太大，会影响栈分配的大小和速度
    // 可以将要大结构放在堆上， 只存放指针。（指针的大小通常是比较小的）
    #[error("Custom error: {0}")]
    Custom(String),
}

#[allow(unused)]
#[derive(Debug)]
pub struct BigError {
    a: String,
    b: Vec<String>,
    c: [u8; 64],
    d: u64,
}
