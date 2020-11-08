use std::collections::HashMap;
use std::net::ToSocketAddrs;

use actix::Addr;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::client::Client;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::Error;
use actix_web::http::{HeaderValue, Method};
use actix_web::http::header::{ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN};
use actix_web::web::{Bytes, Data, Json, Path};
use serde_json::Value;

use crate::mime_type_mapper::MimeTypeMapper;
use crate::services::{DashboardData, DashboardMessage, DashboardService, GroupData, GroupMessage, GroupService, ShortcutData, ShortcutsMessage, ShortcutsService, WebSettingsCompiledMessage, WebSettingsService};
use crate::settings::{AppSettings, ServerType};

pub async fn start_web_server(
    bind_addr: impl ToSocketAddrs,
    web_settings: Addr<WebSettingsService>,
    shortcuts: Addr<ShortcutsService>,
    dashboard: Addr<DashboardService>,
    group: Addr<GroupService>,
    settings: AppSettings,
) -> std::io::Result<()> {
    HttpServer::new(move || App::new()
        .data(web_settings.clone())
        .data(shortcuts.clone())
        .data(settings.clone())
        .data(dashboard.clone())
        .data(group.clone())
        .data(Client::new())
        .data(MimeTypeMapper::default())
        .route("/settings.js", web::get().to(settings_js))
        .route("/api/shortcut", web::get().to(api_shortcuts_list))
        .route("/api/shortcut/{name}", web::get().to(api_shortcut_get))
        .route("/api/shortcut/{name}", web::post().to(api_shortcut_post))
        .route("/api/dashboard", web::get().to(api_dashboard_list))
        .route("/api/dashboard/{name}", web::get().to(api_dashboard_get))
        .route("/api/dashboard/{name}", web::post().to(api_dashboard_post))
        .route("/api/dashboard/{name}", web::delete().to(api_dashboard_delete))
        .route("/api/group/{name}", web::get().to(api_group_get))
        .route("/api/group/{name}", web::post().to(api_group_post))
        .route("/api/group/{name}", web::delete().to(api_group_delete))
        .route(
            "/api/shortcut/{name}",
            web::delete().to(api_shortcut_delete),
        )
        .route("/api/{_:.*}", web::method(Method::OPTIONS).to(|| HttpResponse::Ok()))
        .default_service(web::to(default_service))
        .wrap_fn(|req, srv| {
            let origin = match req.headers().get("Origin") {
                Some(origin) => String::from(origin.to_str().unwrap_or_default()),
                _ => String::new(),
            };
            let fut = srv.call(req);

            async move {
                let mut res: ServiceResponse = fut.await.unwrap();

                res.headers_mut().insert(
                    ACCESS_CONTROL_ALLOW_HEADERS,
                    HeaderValue::from_str("Authorization, Content-Type, Cookie").unwrap(),
                );
                res.headers_mut().insert(
                    ACCESS_CONTROL_ALLOW_METHODS,
                    HeaderValue::from_str("GET, POST, PUT, PATCH, DELETE, CALL").unwrap(),
                );
                res.headers_mut().insert(
                    ACCESS_CONTROL_ALLOW_ORIGIN,
                    HeaderValue::from_str(origin.as_str()).unwrap(),
                );
                res.headers_mut().insert(
                    ACCESS_CONTROL_ALLOW_CREDENTIALS,
                    HeaderValue::from_str("true").unwrap(),
                );

                Ok(res)
            }
        }))
        .bind(bind_addr)?
        .run()
        .await
}

async fn settings_js(settings: Data<Addr<WebSettingsService>>) -> impl Responder {
    match settings.send(WebSettingsCompiledMessage::Get).await {
        Ok(value) => HttpResponse::Ok().header("Content-Type", "application/javascript").body(value),
        Err(error) => {
            eprintln!("[ERROR] [Web Server] {:?}", error);

            HttpResponse::InternalServerError().body("Server error occurred. For more information ask the system administrator")
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
    let name = Path(percent_encoding::percent_decode_str(name.0.as_str()).decode_utf8_lossy().to_string());

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
    let name = Path(percent_encoding::percent_decode_str(name.0.as_str()).decode_utf8_lossy().to_string());

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
    let name = Path(percent_encoding::percent_decode_str(name.0.as_str()).decode_utf8_lossy().to_string());

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
    let name = Path(percent_encoding::percent_decode_str(name.0.as_str()).decode_utf8_lossy().to_string());

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
    let name = Path(percent_encoding::percent_decode_str(name.0.as_str()).decode_utf8_lossy().to_string());

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
    let name = Path(percent_encoding::percent_decode_str(name.0.as_str()).decode_utf8_lossy().to_string());

    let dashboard = match dashboard.send(DashboardMessage::Delete(name.0)).await {
        Ok(dashboard) => dashboard,
        Err(error) => {
            eprintln!("[ERROR] [Web Server] {:?}", error);

            Vec::new()
        }
    };

    HttpResponse::Ok().json(dashboard)
}

async fn api_group_get(name: Path<String>, group: Data<Addr<GroupService>>) -> impl Responder {
    let name = Path(percent_encoding::percent_decode_str(name.0.as_str()).decode_utf8_lossy().to_string());

    let group = match group.send(GroupMessage::Get(name.0)).await {
        Ok(group) => group,
        Err(error) => {
            eprintln!("[ERROR] [Web Selver] {:?}", error);

            return HttpResponse::Ok().json(Value::Null);
        }
    };

    HttpResponse::Ok().json(group)
}

async fn api_group_post(name: Path<String>, body: Json<GroupData>, group: Data<Addr<GroupService>>) -> impl Responder {
    let name = Path(percent_encoding::percent_decode_str(name.0.as_str()).decode_utf8_lossy().to_string());

    let group = match group.send(GroupMessage::Set(name.0, body.0)).await {
        Ok(group) => group,
        Err(error) => {
            eprintln!("[ERROR] [Web Server] {:?}", error);

            return HttpResponse::Ok().json(Value::Null);
        }
    };

    HttpResponse::Ok().json(group)
}

async fn api_group_delete(name: Path<String>, group: Data<Addr<GroupService>>) -> impl Responder {
    let name = Path(percent_encoding::percent_decode_str(name.0.as_str()).decode_utf8_lossy().to_string());

    let group = match group.send(GroupMessage::Delete(name.0)).await {
        Ok(group) => group,
        Err(error) => {
            eprintln!("[ERROR] [Web Server] {:?}", error);

            return HttpResponse::Ok().json(Value::Null);
        }
    };

    HttpResponse::Ok().json(group)
}

async fn default_service(
    req: HttpRequest,
    body: Bytes,
    client: Data<Client>,
    mime_type_mapper: Data<MimeTypeMapper>,
    settings: Data<AppSettings>,
) -> Result<HttpResponse, Error> {
    match &settings.server_type {
        ServerType::Proxy(base_url) => default_proxy(req, body, client, base_url.clone()).await,
        ServerType::File(public_path) => default_serve(req, public_path.clone(), mime_type_mapper).await,
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

async fn default_serve(request: HttpRequest, public_path: String, mime_type_mapper: Data<MimeTypeMapper>) -> Result<HttpResponse, Error> {
    let read_path = format!(
        "{}/{}",
        public_path.trim_end_matches("/"),
        String::from(request.uri().path()).trim_start_matches("/")
    );
    let mime_type = mime_type_mapper.match_file(&read_path);

    match std::fs::read(read_path) {
        Ok(content) => Ok(HttpResponse::Ok().header("Content-Type", mime_type).body(content)),
        Err(error) => {
            eprintln!("[ERROR] [WebServer]: Could not open file {:?}", error);

            Ok(HttpResponse::NotFound().body("File not found."))
        }
    }
}
