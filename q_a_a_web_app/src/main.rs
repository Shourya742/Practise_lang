use std::str::FromStr;
use std::io::{Error,ErrorKind};

use serde::Serialize;
use warp::Filter;
use warp::reject::{Reject, Rejection};
use warp::reply::Reply;
use warp::http::{StatusCode,Method};
use warp::filters::cors::CorsForbidden;

#[derive(Debug,Serialize)]
struct QuestionId(String);

#[derive(Debug,Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>
}


#[derive(Debug)]

struct InvalidId;

impl Reject for InvalidId {}


impl Question {
    fn new(id: QuestionId,title:String,content: String,tags:Option<Vec<String>>)-> Self {
        Question {
            id,title,content,tags
        }
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput,"No id provided"))
        }
    }
}

async fn get_question() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(QuestionId::from_str("1").expect("No id provided"),"First Questi0on".to_string(),"Content of question".to_string(),Some(vec!("faq".to_string())));
    match question.id.0.parse::<i32>() {
        Err(_) => {
            Err(warp::reject::custom(InvalidId))
        },
        Ok(_) => {
            Ok(warp::reply::json(&question))
        }
    }
}

async fn return_error(r:Rejection) -> Result<impl Reply,Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(error.to_string(), StatusCode::FORBIDDEN))
    }
    else if let Some(InvalidId) = r.find() {
        Ok(warp::reply::with_status("No valid Id Presented".to_string(), StatusCode::UNPROCESSABLE_ENTITY))
    } else {
        Ok(warp::reply::with_status("Route not found".to_string(), StatusCode::NOT_FOUND))
    }
}



#[tokio::main]
async fn main() {
    let cors = warp::cors().allow_any_origin().allow_header("content-type").allow_methods(&[Method::PUT,Method::DELETE,Method::GET,Method::POST]);
    let get_items = warp::get().and(warp::path("questions")).and(warp::path::end()).and_then(get_question).recover(return_error);
    let routes = get_items.with(cors);
    warp::serve(routes).run(([127,0,0,1],3030)).await;
}