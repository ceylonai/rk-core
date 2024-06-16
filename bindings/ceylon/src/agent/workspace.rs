use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use uniffi::deps::log::debug;

use crate::AgentCore;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WorkspaceConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
}

pub struct Workspace {
    id: String,
    port: u16,
    host: String,
    _name: String,
    _agents: RwLock<Vec<Arc<AgentCore>>>,
}


impl Workspace {
    pub fn new(agents: Vec<Arc<AgentCore>>, config: WorkspaceConfig) -> Self {
        let _name = config.name;
        let id = format!("workspace-{}", uuid::Uuid::new_v4());

        // Set agent workspace_id
        for agent in agents.iter() {
            agent.set_workspace_id(id.clone());
        }

        // Validate: agent name,id must be unique
        let mut names = vec![];
        let mut ids = vec![];
        for agent in agents.iter() {
            let name = agent.definition().name.clone();
            if names.contains(&name) {
                panic!("Agent name {} is not unique", name);
            }
            names.push(name.clone());
            ids.push(agent.id());
        }


        Self {
            id,
            port: config.port,
            host: config.host,
            _name,
            _agents: RwLock::new(agents),
        }
    }

    pub async fn run(&self, input: Vec<u8>) {
        debug!("Workspace {} running", self.id);
        let rt = Runtime::new().unwrap();
        let mut tasks = vec![];
        let _input = input.clone();
        for agent in self._agents.read().unwrap().iter() {
            let _inputs = _input.clone();
            let url = format!("{}/{}", self.host, self.port);
            let topic = format!("workspace-{}", agent.workspace_id());

            let agent = agent.clone();
            let task = rt.spawn(async move {
                agent.start(topic, url, _inputs).await;
            });
            tasks.push(task);
        }

        for task in tasks {
            task.await.unwrap();
        }
    }
}

