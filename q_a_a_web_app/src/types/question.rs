use serde::{Deserialize, Serialize};

#[derive(Deserialize,Debug,Clone,PartialEq,Eq,Hash,Serialize)]
pub struct QuestionId(pub i32);


#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>
}


#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct NewQuestion {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>
}