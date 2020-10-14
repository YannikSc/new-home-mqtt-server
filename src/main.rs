extern crate serde;

use actix::Actor;
use actix_web::rt::{Arbiter, System};

use crate::console::ConsoleApp;
use crate::services::{WebSettingsCompiledMessage, WebSettingsService};
use crate::web_handler::start_web_server;

mod console;
mod services;
mod settings;
mod thread_helper;
mod web_handler;

fn main() {
    let mut server_system = System::new("sys_webserver");
    let mut web_settings_arbiter = Arbiter::new();
    let web_settings_addr =
        WebSettingsService::start_in_arbiter(&web_settings_arbiter, |_| WebSettingsService::new());
    web_settings_addr.do_send(WebSettingsCompiledMessage::Reload);

    let console_arbiter = Arbiter::new();
    let console_settings = Clone::clone(&web_settings_addr);
    ConsoleApp::start_in_arbiter(&console_arbiter, move |_| ConsoleApp::new(console_settings));

    let server = start_web_server("localhost:9002", web_settings_addr);

    server_system.block_on(server).unwrap();
    server_system.run().unwrap();
    web_settings_arbiter.join().unwrap();
}
