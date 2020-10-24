use std::env::current_dir;
use std::fs::OpenOptions;
use std::io::Write;

use actix::{Actor, Context, Handler, Message, MessageResult, Addr};

use crate::services::{DashboardService, DataReadWrite, GroupData, GroupMessage, GroupService, IndexOf};
use crate::services::group::group_dashboard_messages::AnyDashboardUsesGroup;

mod group_dashboard_messages {
    pub struct AnyDashboardUsesGroup(pub String);
}

impl Message for AnyDashboardUsesGroup {
    type Result = bool;
}

impl Handler<AnyDashboardUsesGroup> for DashboardService {
    type Result = MessageResult<AnyDashboardUsesGroup>;

    fn handle(&mut self, msg: AnyDashboardUsesGroup, _: &mut Self::Context) -> Self::Result {
        for dashboard in &self.dashboards {
            for group in &dashboard.groups {
                if group.eq(msg.0.as_str()) { return MessageResult(true); }
            }
        }

        MessageResult(false)
    }
}

impl DataReadWrite for Vec<GroupData> {
    fn load() -> Self {
        let groups_path = current_dir().unwrap().join("group.yaml");
        let file = OpenOptions::new().read(true).open(groups_path);

        match file {
            Ok(file) => match serde_yaml::from_reader(file) {
                Ok(groups) => groups,
                Err(error) => {
                    eprintln!("[ERROR] [Group]: Could not parse file");
                    eprintln!("{}", error);

                    Vec::new()
                }
            },
            Err(error) => {
                eprintln!("[WARN] [Group]: Could not open file");
                eprintln!("{}", error);

                Vec::new()
            }
        }
    }

    fn save(&self) {
        let groups_path = current_dir().unwrap().join("group.yaml");
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(groups_path);

        match file {
            Ok(mut file) => {
                if let Err(error) = serde_yaml::to_writer(&file, &self) {
                    eprintln!("[ERROR] [Group]: Could not write groups");
                    eprintln!("{}", error);

                    return;
                }

                if let Err(error) = file.flush() {
                    eprintln!("[WARN] [Group]: Could not fully write groups");
                    eprintln!("{}", error);
                }
            }
            Err(error) => {
                eprintln!("[ERROR] [Group]: Could not open/create file");
                eprintln!("{}", error);
            }
        }
    }

    fn single(&self, which: String) -> Self {
        for item in self {
            if item.name.eq(which.as_str()) {
                return vec![item.clone()];
            }
        }

        Vec::new()
    }
}

impl IndexOf<usize, String> for Vec<GroupData> {
    fn index_of(&self, search: String) -> Option<usize> {
        for (index, group) in self.iter().enumerate() {
            if group.name.eq(search.as_str()) {
                return Some(index);
            }
        }

        None
    }
}

impl GroupService {
    pub fn new(dashboard: Addr<DashboardService>) -> Self {
        Self {
            groups: Vec::<GroupData>::load(),
            dashboard
        }
    }
}

impl Actor for GroupService {
    type Context = Context<Self>;

    fn stopped(&mut self, _: &mut Self::Context) {
        self.groups.save();
    }
}

impl Handler<GroupMessage> for GroupService {
    type Result = MessageResult<GroupMessage>;

    fn handle(&mut self, msg: GroupMessage, _: &mut Self::Context) -> Self::Result {
        match msg {
            GroupMessage::Reload => {
                self.groups = Vec::<GroupData>::load();

                MessageResult(None)
            }
            GroupMessage::Get(name) => {
                if let Some(index) = self.groups.index_of(name) {
                    return MessageResult(match self.groups.get(index) {
                        None => None,
                        Some(group) => Some(group.clone())
                    });
                }

                MessageResult(None)
            }
            GroupMessage::Set(name, group) => {
                if let Some(index) = self.groups.index_of(name) {
                    self.groups[index] = group.clone();
                    self.groups.save();

                    return MessageResult(Some(group));
                }

                self.groups.push(group.clone());
                self.groups.save();

                MessageResult(Some(group))
            }
            GroupMessage::Delete(name) => {
                let is_in_use = match futures::executor::block_on(self.dashboard.send(AnyDashboardUsesGroup(name.clone()))) {
                    Ok(result) => result,
                    Err(error) => {
                        eprintln!("[ERROR] [Group Service] {:?}", error);

                        false
                    }
                };

                if is_in_use {
                    return MessageResult(None);
                }

                if let Some(index) = self.groups.index_of(name) {
                    self.groups.save();

                    return MessageResult(Some(self.groups.remove(index)));
                }

                MessageResult(None)
            }
        }
    }
}

impl Message for GroupMessage {
    type Result = Option<GroupData>;
}
