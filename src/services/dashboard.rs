use std::env::current_dir;
use std::fs::OpenOptions;
use std::io::Write;

use actix::{Actor, Context, Handler, Message, MessageResult};

use crate::services::{DashboardData, DashboardMessage, DashboardService, DataReadWrite};

impl DataReadWrite for Vec<DashboardData> {
    fn load() -> Self {
        let dashboards_path = current_dir().unwrap().join("dashboard.yaml");
        let file = OpenOptions::new().read(true).open(dashboards_path);

        match file {
            Ok(file) => match serde_yaml::from_reader(file) {
                Ok(dashboards) => dashboards,
                Err(error) => {
                    eprintln!("[ERROR] [Dashboard]: Could not parse file");
                    eprintln!("{}", error);

                    Vec::new()
                }
            },
            Err(error) => {
                eprintln!("[WARN] [Dashboard]: Could not open file");
                eprintln!("{}", error);

                Vec::new()
            }
        }
    }

    fn save(&self) {
        let dashboards_path = current_dir().unwrap().join("dashboard.yaml");
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(dashboards_path);

        match file {
            Ok(mut file) => {
                if let Err(error) = serde_yaml::to_writer(&file, &self) {
                    eprintln!("[ERROR] [Dashboard]: Could not write dashboards");
                    eprintln!("{}", error);

                    return;
                }

                if let Err(error) = file.flush() {
                    eprintln!("[WARN] [Dashboard]: Could not fully write dashboards");
                    eprintln!("{}", error);
                }
            }
            Err(error) => {
                eprintln!("[ERROR] [Dashboard]: Could not open/create file");
                eprintln!("{}", error);
            }
        }
    }

    fn single(&self, which: String) -> Self {
        if let Some(index) = self.index_of(which) {
            if let Some(item) = self.get(index) {
                return vec![item.clone()];
            }
        }

        Vec::new()
    }
}

trait IndexOf<I, T> {
    fn index_of(&self, search: T) -> Option<I>;
}

impl IndexOf<usize, String> for Vec<DashboardData> {
    fn index_of(&self, search: String) -> Option<usize> {
        for (index, item) in self.iter().enumerate() {
            if item.name.eq(search.as_str()) {
                return Some(index);
            }
        }

        return None;
    }
}

impl DashboardService {
    pub fn new() -> Self {
        Self {
            dashboards: Default::default()
        }
    }
}

impl Actor for DashboardService {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("Started dashboard");

        self.dashboards = Vec::<DashboardData>::load();
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        println!("Stopped Dashboard");

        self.dashboards.save();
    }
}

impl Handler<DashboardMessage> for DashboardService {
    type Result = MessageResult<DashboardMessage>;

    fn handle(&mut self, msg: DashboardMessage, _: &mut Self::Context) -> Self::Result {
        match msg {
            DashboardMessage::List => MessageResult(self.dashboards.clone()),
            DashboardMessage::Reload => {
                self.dashboards = Vec::<DashboardData>::load();

                MessageResult(self.dashboards.clone())
            }
            DashboardMessage::Get(name) => MessageResult(self.dashboards.single(name)),
            DashboardMessage::Set(name, data) => {
                if let Some(index) = self.dashboards.index_of(name) {
                    self.dashboards[index] = data.clone();
                    self.dashboards.save();

                    return MessageResult(self.dashboards.clone());
                }

                self.dashboards.push(data.clone());
                self.dashboards.save();

                MessageResult(self.dashboards.clone())
            }
            DashboardMessage::Delete(name) => {
                if let Some(index) = self.dashboards.index_of(name) {
                    self.dashboards.remove(index);
                }

                self.dashboards.save();

                MessageResult(self.dashboards.clone())
            }
        }
    }
}

impl Message for DashboardMessage {
    type Result = Vec<DashboardData>;
}
