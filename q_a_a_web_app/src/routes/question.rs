use std::collections::HashMap;

use handle_errors::Error;

use crate::{store::Store, types::{pagination::extract_pagination, question::{Question,  QuestionId}}};
use warp::http::StatusCode;



pub async fn get_questions(params: HashMap<String,String>,store:Store)->Result<impl warp::Reply,warp::Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

pub async fn update_question(id:String,store:Store,question:Question)->Result<impl warp::Reply,warp::Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }
    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

pub async fn delete_question(id:String,store:Store)->Result<impl warp::Reply,warp::Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(_) => (),
        None => return Err(warp::reject::custom(Error::QuestionNotFound))
    }
    Ok(warp::reply::with_status("Question deleted", StatusCode::OK))
}

pub async fn add_question(store:Store,question:Question)->Result<impl warp::Reply,warp::Rejection> {
    store.questions.write().await.insert(question.clone().id,question );
    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}