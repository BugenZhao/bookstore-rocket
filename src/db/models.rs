// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use crate::db::schema::*;
use serde_derive::*;

#[derive(Queryable, Debug, Serialize, Identifiable)]
pub struct Book {
    pub id: i32,
    pub isbn: String,
    pub name: String,
    pub r#type: String,
    pub author: String,
    pub price: f64,
    pub description: String,
    pub inventory: i32,
    pub image: String,
}

#[derive(Queryable, Debug, Serialize, Identifiable)]
#[primary_key(user_id)]
pub struct UserAuth {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub user_type: i32,
}

#[derive(Queryable, Debug, Serialize, Identifiable)]
#[primary_key(user_id)]
pub struct User {
    pub user_id: i32,
    pub nickname: String,
    pub name: Option<String>,
    pub tel: Option<String>,
    pub address: Option<String>,
}
