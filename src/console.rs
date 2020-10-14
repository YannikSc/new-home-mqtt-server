use std::io::stdin;
use std::time::Duration;

use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};

use crate::services::{WebSettingsCompiledMessage, WebSettingsMessage, WebSettingsService};
use crate::thread_helper::run_in_thread;

pub struct ConsoleApp {
    settings: Addr<WebSettingsService>,
    on_stop: Option<Box<dyn Fn()>>,
}

pub struct ConsoleMessage(String);

impl ConsoleApp {
    pub fn new(settings: Addr<WebSettingsService>) -> Self {
        ConsoleApp {
            settings,
            on_stop: None,
        }
    }
}

impl Actor for ConsoleApp {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();

        let (stop, _) = run_in_thread(
            move |recv| loop {
                let mut line = String::new();

                match stdin().read_line(&mut line) {
                    Err(error) => {
                        eprintln!("[ERROR] [Console]: Could not receive from cli. Canceling");
                        eprintln!("{:?}", error);

                        break;
                    }
                    _ => {}
                }

                addr.do_send(ConsoleMessage(String::from(line.trim())));

                if recv
                    .recv_timeout(Duration::from_millis(10))
                    .unwrap_or(false)
                {
                    break;
                }
            },
            String::from("Console listener"),
        );

        self.on_stop = Some(Box::new(move || {
            stop();
        }));
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        match &self.on_stop {
            Some(fun) => fun(),
            None => {}
        }
    }
}

impl Handler<ConsoleMessage> for ConsoleApp {
    type Result = ();

    fn handle(&mut self, msg: ConsoleMessage, _: &mut Self::Context) -> Self::Result {
        if msg.is("/reload_web_config") {
            self.settings.do_send(WebSettingsMessage::Reload);
            self.settings.do_send(WebSettingsCompiledMessage::Reload);

            println!("Reloading web settings.");

            return;
        }

        if msg.is("/show_web_config") {
            futures::executor::block_on(async {
                match self.settings.send(WebSettingsMessage::Get).await {
                    Ok(settings) => println!("{:?}", settings),
                    _ => eprintln!("Could not get settings."),
                }
            });

            return;
        }

        eprintln!("Command not found.");
    }
}

impl ConsoleMessage {
    pub fn is(&self, cmd: impl ToString) -> bool {
        self.0.starts_with(&cmd.to_string())
    }
}

impl Message for ConsoleMessage {
    type Result = ();
}