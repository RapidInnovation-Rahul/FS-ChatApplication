use actix_web::{web, HttpResponse};
use futures::StreamExt;
use mongodb::Client;
use serde::{Deserialize, Serialize};

/// Database config..
const DB_NAME: &str = "FS-ChatApp";
const COLL_NAME: &str = "Messages";

/// Message Schema..
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub sender: String,
    pub msg_body: String,
}

// send message

pub async fn store(client: web::Data<Client>, message: web::Json<Message>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection::<Message>(COLL_NAME);
    let new_msg = Message {
        sender: message.sender.clone(),
        msg_body: message.msg_body.clone(),
    };
    let _result = collection.insert_one(&new_msg, None).await.unwrap();
    HttpResponse::Ok().json(new_msg)
}

// get messages()
pub async fn get_messages(client: web::Data<Client>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection::<Message>(COLL_NAME);

    // create a cursor to iterate over the database collection
    let mut cur = collection.find(None, None).await.unwrap();

    // create a vec to store the messages and return
    let mut res = Vec::new();

    // push the messages to the vec
    while let Some(item) = cur.next().await {
        res.push(item.unwrap())
    }
    HttpResponse::Ok().json(res)
}
