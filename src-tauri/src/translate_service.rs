use serde::{Deserialize, Serialize};

// pub mod volcengine_translator;
pub mod baidu_translator;

#[derive(Deserialize, Serialize, Debug)]
pub struct TranslateResult {
    pub dst: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TranslateResults {
    pub from: String,
    pub to: String,
    pub trans_result: Vec<TranslateResult>,
}
