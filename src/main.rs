//! # New Home MQTT Server
//!
//! This is the server side application for the [New Home MQTT](https://github.com/YannikSc/new-home-mqtt) frontend application.
//! It provides the option to proxy the frontend to a running webserver or serving it from a "public"
//! Folder and handling the settings for the frontend.
//!

extern crate serde;

use std::env::args;

use actix::Actor;
use actix_web::rt::{Arbiter, System};

use crate::console::ConsoleApp;
use crate::services::{DashboardService, GroupService, ShortcutsService, WebSettingsCompiledMessage, WebSettingsService};
use crate::settings::AppSettings;
use crate::web_handler::start_web_server;

mod console;
mod services;
mod settings;
mod thread_helper;
mod web_handler;
mod mime_type_mapper;

fn main() {
    let mut console_enabled = true;

    for arg in args() {
        if arg.eq("--no-console") {
            console_enabled = false;
        }
    }

    let app_settings = AppSettings::load();
    app_settings.save();

    let mut server_system = System::new("sys_webserver");
    let mut web_settings_arbiter = Arbiter::new();
    let web_settings_addr =
        WebSettingsService::start_in_arbiter(&web_settings_arbiter, |_| WebSettingsService::new());
    web_settings_addr.do_send(WebSettingsCompiledMessage::Reload);

    let mut shortcuts_arbiter = Arbiter::new();
    let shortcuts_addr =
        ShortcutsService::start_in_arbiter(&shortcuts_arbiter, |_| ShortcutsService::new());

    let mut dashboard_arbiter = Arbiter::new();
    let dashboard_addr =
        DashboardService::start_in_arbiter(&dashboard_arbiter, |_| DashboardService::new());

    let group_dashboard_addr = Clone::clone(&dashboard_addr);
    let mut group_arbiter = Arbiter::new();
    let group_addr =
        GroupService::start_in_arbiter(&group_arbiter, |_| GroupService::new(group_dashboard_addr));

    let mut console_arbiter = Arbiter::new();
    if console_enabled {
        let console_settings = Clone::clone(&web_settings_addr);
        let console_shortcuts = Clone::clone(&shortcuts_addr);
        let console_dashboard = Clone::clone(&dashboard_addr);
        let console_group = Clone::clone(&group_addr);
        ConsoleApp::start_in_arbiter(&console_arbiter, move |_| ConsoleApp::new(
            console_settings,
            console_shortcuts,
            console_dashboard,
            console_group,
        ));
    }

    let server = start_web_server(
        format!("{}:{}", &app_settings.host, &app_settings.port),
        web_settings_addr,
        shortcuts_addr,
        dashboard_addr,
        group_addr,
        app_settings.clone(),
    );

    server_system.block_on(server).unwrap();
    server_system.run().unwrap();
    web_settings_arbiter.join().unwrap();
    console_arbiter.join().unwrap();
    shortcuts_arbiter.join().unwrap();
    dashboard_arbiter.join().unwrap();
    group_arbiter.join().unwrap();
}
