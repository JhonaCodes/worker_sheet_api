use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Json};

use uuid::Uuid;
use crate::helper::response::{susses_json, un_susses_json};
use crate::model::AppState;
use crate::participants::models::ParticipantsModel;


pub struct ParticipantsRepository {}

impl ParticipantsRepository {
    pub async fn assign_participant(conn: Data<AppState>, body: Json<ParticipantsModel>) -> impl Responder {
        let participants: ParticipantsModel = body.into_inner();

        match sqlx::query(r#"insert into participants (activity_id, user_id) values ($1, $2)"#)
            .bind(participants.activity_id)
            .bind(participants.user_id)
            .execute(&conn.db)
            .await
        {
            Ok(_) => {

                HttpResponse::Created().json("User created successfully")
            },
            Err(e) => {
                println!("Error {}", e);
                HttpResponse::InternalServerError().json("Error al asignar participante.")
            }
        }
    }

    pub async fn get_participants(conn: Data<AppState>, id_activity: Uuid) -> impl Responder {
        match sqlx::query_as::<_,ParticipantsModel>(r#"select * from participants where activity_id = $1"#)
            .bind(id_activity)
            .fetch_all(&conn.db)
            .await
        {
            Ok(participants)=> susses_json(&participants) ,
            Err(err)=>{
                println!("Error {}", err);

                return  un_susses_json("Error al llamar participanes.");
            }
        }
    }

    pub async fn get_activities_by_participant_id(conn: Data<AppState>, user_id: Uuid) -> impl Responder {
        match sqlx::query_as::<_, ParticipantsModel> (r#"select * from participants where user_id = $1"#)
            .bind(user_id)
            .fetch_all(&conn.db)
            .await
        {
            Ok(participant_list) => susses_json(participant_list),
            Err(err) => {
                println!("Error {}", err);
                return un_susses_json("Error al llamar las actividades");
            }
        }
    }

}