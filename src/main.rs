extern crate serde;

use actix::Actor;
use actix_web::rt::{Arbiter, System};

use crate::console::ConsoleApp;
use crate::services::{ShortcutsService, WebSettingsCompiledMessage, WebSettingsService};
use crate::settings::AppSettings;
use crate::web_handler::start_web_server;

mod console;
mod services;
mod settings;
mod thread_helper;
mod web_handler;

fn main() {
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

    let mut console_arbiter = Arbiter::new();
    let console_settings = Clone::clone(&web_settings_addr);
    ConsoleApp::start_in_arbiter(&console_arbiter, move |_| ConsoleApp::new(console_settings));

    let server = start_web_server(
        format!("{}:{}", &app_settings.host, &app_settings.port),
        web_settings_addr,
        shortcuts_addr,
        app_settings.clone(),
    );

    server_system.block_on(server).unwrap();
    server_system.run().unwrap();
    web_settings_arbiter.join().unwrap();
    console_arbiter.join().unwrap();
    shortcuts_arbiter.join().unwrap();
}
