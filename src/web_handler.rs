use std::collections::HashMap;
use std::net::ToSocketAddrs;

use actix::Addr;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::client::Client;
use actix_web::Error;
use actix_web::web::{Bytes, Data, Json, Path};

use crate::services::{DashboardData, DashboardMessage, DashboardService, ShortcutData, ShortcutsMessage, ShortcutsService, WebSettingsCompiledMessage, WebSettingsService};
use crate::settings::{AppSettings, ServerType};

pub async fn start_web_server(
    bind_addr: impl ToSocketAddrs,
    web_settings: Addr<WebSettingsService>,
    shortcuts: Addr<ShortcutsService>,
    dashboard: Addr<DashboardService>,
    settings: AppSettings,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(web_settings.clone())
            .data(shortcuts.clone())
            .data(settings.clone())
            .data(dashboard.clone())
            .data(Client::new())
            .route("/settings.js", web::get().to(settings_js))
            .route("/api/shortcut", web::get().to(api_shortcuts_list))
            .route("/api/shortcut/{name}", web::get().to(api_shortcut_get))
            .route("/api/shortcut/{name}", web::post().to(api_shortcut_post))
            .route("/api/dashboard", web::get().to(api_dashboard_list))
            .route("/api/dashboard/{name}", web::get().to(api_dashboard_get))
            .route("/api/dashboard/{name}", web::post().to(api_dashboard_post))
            .route("/api/dashboard/{name}", web::delete().to(api_dashboard_delete))
            .route(
                "/api/shortcut/{name}",
                web::delete().to(api_shortcut_delete),
            )
            .default_service(web::to(default_service))
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

async fn api_shortcuts_list(shortcuts: Data<Addr<ShortcutsService>>) -> impl Responder {
    let shortcuts = match shortcuts.send(ShortcutsMessage::List).await {
        Ok(data) => data,
        Err(error) => {
            eprintln!("[ERROR] [Web Server] {:?}", error);

            HashMap::new()
        }
    };

    HttpResponse::Ok().json(&shortcuts)
}

async fn api_shortcut_get(
    name: Path<String>,
    shortcuts: Data<Addr<ShortcutsService>>,
) -> impl Responder {
    let shortcuts = match shortcuts
        .send(ShortcutsMessage::Get(name.to_string()))
        .await
    {
        Ok(data) => data,
        Err(error) => {
            eprintln!("[ERROR] [Web Server] {:?}", error);

            HashMap::new()
        }
    };

    HttpResponse::Ok().json(&shortcuts)
}

async fn api_shortcut_post(
    name: Path<String>,
    body: Json<Vec<ShortcutData>>,
    shortcuts: Data<Addr<ShortcutsService>>,
) -> impl Responder {
    let shortcuts = match shortcuts
        .send(ShortcutsMessage::Add(name.to_string(), body.0))
        .await
    {
        Ok(data) => data,
        Err(error) => {
            eprintln!("[ERROR] [Web Server] {:?}", error);

            HashMap::new()
        }
    };

    HttpResponse::Ok().json(&shortcuts)
}

async fn api_shortcut_delete(
    name: Path<String>,
    shortcuts: Data<Addr<ShortcutsService>>,
) -> impl Responder {
    let shortcuts = match shortcuts
        .send(ShortcutsMessage::Delete(name.to_string()))
        .await
    {
        Ok(data) => data,
        Err(error) => {
            eprintln!("[ERROR] [Web Server] {:?}", error);

            HashMap::new()
        }
    };

    HttpResponse::Ok().json(&shortcuts)
}

async fn api_dashboard_list(dashboard: Data<Addr<DashboardService>>) -> impl Responder {
    let dashboards = match dashboard.send(DashboardMessage::List).await {
        Ok(dashboards) => dashboards,
        Err(error) => {
            eprintln!("[ERROR] [Web Server] {:?}", error);

            Vec::new()
        }
    };

    HttpResponse::Ok().json(dashboards)
}

async fn api_dashboard_get(name: Path<String>, dashboard: Data<Addr<DashboardService>>) -> impl Responder {
    let dashboard = match dashboard.send(DashboardMessage::Get(name.0)).await {
        Ok(dashboard) => dashboard,
        Err(error) => {
            eprintln!("[ERROR] [Web Server] {:?}", error);

            Vec::new()
        }
    };

    HttpResponse::Ok().json(dashboard)
}

async fn api_dashboard_post(name: Path<String>, body: Json<DashboardData>, dashboard: Data<Addr<DashboardService>>) -> impl Responder {
    let dashboards = match dashboard.send(DashboardMessage::Set(name.0, body.0)).await {
        Ok(dashboard) => dashboard,
        Err(error) => {
            eprintln!("[ERROR] [Web Server] {:?}", error);

            Vec::new()
        }
    };

    HttpResponse::Ok().json(dashboards)
}

async fn api_dashboard_delete(name: Path<String>, dashboard: Data<Addr<DashboardService>>) -> impl Responder {
    let dashboard = match dashboard.send(DashboardMessage::Delete(name.0)).await {
        Ok(dashboard) => dashboard,
        Err(error) => {
            eprintln!("[ERROR] [Web Server] {:?}", error);

            Vec::new()
        }
    };

    HttpResponse::Ok().json(dashboard)
}

async fn default_service(
    req: HttpRequest,
    body: Bytes,
    client: Data<Client>,
    settings: Data<AppSettings>,
) -> Result<HttpResponse, Error> {
    match &settings.server_type {
        ServerType::Proxy(base_url) => default_proxy(req, body, client, base_url.clone()).await,
        ServerType::File(public_path) => Ok(default_serve(public_path.clone()).await),
    }
}

async fn default_proxy(
    req: HttpRequest,
    body: web::Bytes,
    client: web::Data<Client>,
    base_url: String,
) -> Result<HttpResponse, Error> {
    let base_url = base_url.trim_end_matches('/').to_string();
    let full_uri = base_url + req.uri().to_string().as_str();
    let request = client.request_from(full_uri, req.head()).no_decompress();
    let mut response = request.send_body(body).await.map_err(Error::from)?;
    let mut client_response = HttpResponse::build(response.status());

    for (name, value) in response
        .headers()
        .iter()
        .filter(|(h, _)| *h != "connection")
    {
        client_response.header(name.clone(), value.clone());
    }

    Ok(client_response.body(response.body().limit(10240000).await?))
}

async fn default_serve(public_path: String) -> HttpResponse {
    HttpResponse::from(public_path)
}
