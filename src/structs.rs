use serde::{Deserialize,Serialize};

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct UserInput{
    pub address:String,
    pub start_date:String,
    pub end_date:String
}