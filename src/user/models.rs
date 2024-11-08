use chrono::{NaiveDateTime};
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub position: String,
    pub department: String,
    pub phone: String,
    pub status: String,
    pub email_notification: Option<bool>,
    pub push_notification: Option<bool>,
    pub auto_sync: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive( Deserialize, Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub position: String,
    pub department: String,
    pub phone: String,
    pub status: String,
    pub email_notification:bool,
    pub push_notification:bool,
    pub auto_sync: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub position: String,
    pub department: String,
    pub phone: String,
    pub status: String,
    pub email_notification:bool,
    pub push_notification:bool,
    pub auto_sync: bool,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::activities)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Activities {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub risk_level: String,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub user_id: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_synchronized: bool,
    pub hashtag: Option<String>,
    pub is_deleted: Option<bool>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::activity_photos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PhotoActivity{
    pub id: String,
    pub activity_id: String,
    pub url: String,
}