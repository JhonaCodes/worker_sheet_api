use std::env;
use actix_web::HttpResponse;
use serde::Serialize;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{ MultiPart, SinglePart};
use crate::handler::model::{ApiResponse};

pub fn string(text:&str) -> String{
    String::from(text)
}

pub fn susses_json<T: Serialize>(body: T) -> HttpResponse{
    HttpResponse::Ok().json(body)
}

pub fn un_success_json<T: Serialize>( message: &str, data: Option<T>) -> HttpResponse {
    let response = ApiResponse::new(message, data);
    HttpResponse::BadRequest().json(response)
}


pub fn  send_email(user_email:&String) -> Result<bool, Box<dyn std::error::Error>> {

    let email = env::var("SMTP_EMAIL").expect("HASH_SECRET must be set!");
    let email_password = env::var("SMTP_EMAIL_PASSWORD").expect("HASH_SECRET must be set!");
    let smtp_server = env::var("SMTP_SERVER").expect("HASH_SECRET must be set!");
    let smtp_server_port = env::var("SMTP_SERVER_PORT").expect("HASH_SECRET must be set!");
    
    let smtp_credentials = Credentials::new(
        email.clone(),
        email_password,
    );

    let mailer = SmtpTransport::relay(&smtp_server)
        .unwrap()
        .credentials(smtp_credentials)
        .port(smtp_server_port.parse().unwrap())
        .build();


    let email = Message::builder()
        .from(format!("Worker sheet <{}>", email.clone()).parse()?)
        .to(format!("{}",user_email).parse()?)
        .subject("¡Bienvenido a Worker Sheets!")
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::plain("¡Bienvenido a Worker Sheets! Nos complace darte la bienvenida a tu nueva herramienta para la gestión eficiente de hojas de cálculo y documentos.".to_string())
                )
                .singlepart(
                    SinglePart::html(email_welcome().to_string())
                ),
        )?;

    let mut result_sending_email: bool = false;
    
    match mailer.send(&email) {
        Ok(_) =>{
            result_sending_email = true;
            println!("Email enviado exitosamente!");
        }
        Err(e) => {
            result_sending_email = false;
            println!("No se pudo enviar el email: {:?}", e);
        },
    }


    Ok(result_sending_email)
}


fn email_welcome() -> &'static str {
    r#"
    <!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        body {
            font-family: Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 600px;
            margin: 0 auto;
            padding: 20px;
        }
        .header {
            text-align: center;
            padding: 20px 0;
            border-bottom: 2px solid #f0f0f0;
        }
        .content {
            padding: 20px 0;
        }
        .footer {
            text-align: center;
            padding: 20px 0;
            font-size: 0.9em;
            color: #666;
            border-top: 1px solid #f0f0f0;
        }
        .button {
            display: inline-block;
            padding: 10px 20px;
            background-color: #4CAF50;
            color: white;
            text-decoration: none;
            border-radius: 5px;
            margin: 20px 0;
        }
    </style>
</head>
<body>
<div class="header">
    <h1>¡Bienvenido a Worker Sheets!</h1>
</div>

<div class="content">
    <p>¡Hola!</p>

    <p>Nos complace darte la bienvenida a Worker Sheets, tu nueva herramienta para la gestión eficiente de hojas de cálculo y documentos.</p>

    <p>Con Worker Sheets podrás:</p>
    <ul>
        <li>Gestionar tus documentos de forma eficiente</li>
        <li>Colaborar en tiempo real con tu equipo</li>
        <li>Automatizar tareas repetitivas</li>
        <li>Mantener tus datos seguros y organizados</li>
    </ul>

    <p>Para comenzar, puedes acceder a tu cuenta usando el siguiente botón:</p>

    <center>
        <a href="\#" class="button">Acceder a mi cuenta</a>
    </center>

    <p>Si tienes alguna pregunta o necesitas ayuda, no dudes en contactar con nuestro equipo de soporte.</p>

    <p>¡Gracias por confiar en nosotros!</p>
</div>

<div class="footer">
    <p>Worker Sheets - Gestión inteligente de documentos</p>
    <p>Este es un correo automático, por favor no responder</p>
</div>
</body>
</html>
"#
}