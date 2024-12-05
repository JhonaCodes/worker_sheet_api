use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Json};

use uuid::Uuid;
use crate::helper::email_service_helper::{susses_json, un_success_json};
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
            Ok(_) => susses_json("User created successfully"),
            Err(_) => un_success_json(
                "Participant assignment error",
                Some("Ha ocurrido un error al intentar asignar el participante. Por favor, inténtalo nuevamente más tarde")
            )
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

                return un_success_json(
                    "Participant fetch error",
                    Some("Ocurrió un error al obtener la lista de participantes. Por favor, inténtalo nuevamente")
                );
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
                return un_success_json(
                    "Activity fetch error",
                    Some("Ocurrió un error al obtener la lista de actividades. Por favor, inténtalo nuevamente")
                );
            }
        }
    }

}