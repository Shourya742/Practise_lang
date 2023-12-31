use handle_errors::Error;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use sqlx::postgres::{PgPoolOptions,PgPool,PgRow};
use sqlx::Row;

use crate::types::question::{Question, QuestionId, NewQuestion};

// use crate::types::{
//     answer::{Answer,AnswerId},
//     question::{Question,QuestionId}
// };

#[derive(Debug,Clone)]
pub struct Store {
    // pub questions: Arc<RwLock<HashMap<QuestionId,Question>>>,
    // pub answers: Arc<RwLock<HashMap<AnswerId,Answer>>>
    pub connection: PgPool
}


impl Store {
    pub async fn new(db_url: &str)->Self {
        let db_pool = match PgPoolOptions::new().max_connections(5).connect(db_url).await {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {}",e)
        };

        Store {
            connection: db_pool
        }
    }

    pub async fn get_questions(&self,limit:Option<u32>,offset:u32)-> Result<Vec<Question>,sqlx::Error> {
        match sqlx::query("select * from questions limit $1 offset $2").bind(limit).bind(offset).map(|row:PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content:row.get("content"),
            tags: row.get("tags")
        }).fetch_all(&self.connection).await {
            Ok(questions) => Ok(questions),
            Err(e) => Err(e)
        }
    }

    pub async fn add_question(&self,new_question:NewQuestion) -> Result<Question,sqlx::Error> {
        match sqlx::query("insert into questions (title, content, tags) values ($1, $2, $3)").bind(new_question.title).bind(new_question.content).bind(new_question.tags).map(|row:PgRow| Question {id: QuestionId(row.get("id")),title:row.get("title"),content:row.get("content"),tags:row.get("tags")}).fetch_one(&self.connection).await {
            Ok(question) => Ok(question),
            Err(e) => Err(e)
        }
    }

    // pub fn new() -> Self {
    //     Store {
    //         // questions: Arc::new(RwLock::new(Self::init())),
    //         // answers: Arc::new(RwLock::new(HashMap::new()))
    //     }
    // }
    // fn init() -> HashMap<QuestionId,Question> {
    //     let file = include_str!("../questions.json");
    //     serde_json::from_str(file).expect("can't read questions.json")
    // }
}