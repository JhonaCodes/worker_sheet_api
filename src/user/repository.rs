// use super::models::{NewUser, Users};
// use crate::schema::users;
// use diesel::prelude::*;
// 
// pub fn create_user(conn: &mut PgConnection, new_user: NewUser) -> QueryResult<Users> {
//     diesel::insert_into(users::table)
//         .values(&new_user)
//         .returning(Users::as_returning())
//         .get_result(conn)
// }
// 
// pub fn get_user_by_id(conn: &mut PgConnection, user_id: &str) -> QueryResult<Users> {
//     users::table.find(user_id).first(conn)
// }
// 
// pub fn get_all_users(conn: &mut PgConnection) -> QueryResult<Vec<Users>> {
//     users::table.load::<Users>(conn)
// }
// 
// pub fn update_user(conn: &mut PgConnection, user_id: &str, user: NewUser) -> QueryResult<Users> {
//     diesel::update(users::table.find(user_id))
//         .set((users::name.eq(user.name), users::email.eq(user.email)))
//         .returning(Users::as_returning())
//         .get_result(conn)
// }
// 
// pub fn delete_user(conn: &mut PgConnection, user_id: &str) -> QueryResult<usize> {
//     diesel::delete(users::table.find(user_id)).execute(conn)
// }
