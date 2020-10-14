use std::net::ToSocketAddrs;

use actix::Addr;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer, Responder};

use crate::services::{WebSettingsCompiledMessage, WebSettingsService};

pub async fn start_web_server(
    bind_addr: impl ToSocketAddrs,
    web_settings: Addr<WebSettingsService>,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(web_settings.clone())
            .route("/settings.js", web::get().to(settings_js))
    })
    .bind(bind_addr)?
    .run()
    .await
}

async fn settings_js(settings: Data<Addr<WebSettingsService>>) -> impl Responder {
    match settings.send(WebSettingsCompiledMessage::Get).await {
        Ok(value) => value,
        Err(error) => {
            eprintln!("[ERROR] [Web Server] {:?}", error);

            String::new()
        }
    }
}
