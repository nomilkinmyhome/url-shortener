use std::time::SystemTime;

use diesel::prelude::*;

#[derive(Queryable)]
pub struct URL {
    pub id: i32,
    pub original_url: String,
    pub shorted_url: String,
    pub created_at: SystemTime,
}