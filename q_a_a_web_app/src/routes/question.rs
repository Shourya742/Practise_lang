use std::collections::HashMap;

use handle_errors::Error;
use sqlx::postgres::PgRow;

use crate::{store::Store, types::{pagination::{extract_pagination, self, Pagination}, question::{Question,  QuestionId, NewQuestion}}};
use warp::http::StatusCode;



pub async fn get_questions(params: HashMap<String,String>,store:Store,id: String)->Result<impl warp::Reply,warp::Rejection> {
    log::info!("{} Start querying questions",id);
    let mut pagination = Pagination::default();
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        log::info!("{} Pagination set {:?}",id,&pagination);
    } 

    let res: Vec<Question> = match store.get_questions(pagination.limit, pagination.offset).await{
        Ok(res) => res,
        Err(e) => {
            return Err(warp::reject::custom(Error::DatabaseQueryError(e)))
        }
    };
    Ok(warp::reply::json(&res))
}

pub async fn update_question(id:i32,store:Store,question:Question)->Result<impl warp::Reply,warp::Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }
    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

pub async fn delete_question(id:i32,store:Store)->Result<impl warp::Reply,warp::Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(_) => (),
        None => return Err(warp::reject::custom(Error::QuestionNotFound))
    }
    Ok(warp::reply::with_status("Question deleted", StatusCode::OK))
}

// pub async fn add_question(store:Store,question:Question)->Result<impl warp::Reply,warp::Rejection> {
//     store.questions.write().await.insert(question.clone().id,question );
//     Ok(warp::reply::with_status("Question added", StatusCode::OK))
// }

pub async fn add_question(store: Store,new_question:NewQuestion)->Result<impl warp::Reply,warp::Rejection> {
    if let Err(e) = store.add_question(new_question).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError(e)));
    } 

    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}
