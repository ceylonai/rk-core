use log::{debug, info};
use pyo3::{IntoPy, Python};
use pyo3_asyncio::TaskLocals;

use crate::executor::execute_process_function;
use crate::transport::Transporter;
use crate::types::{Event, EventProcessor, EventType, OriginatorType, TransportStatus};

pub struct Application {
    name: String,
    event_processors: Vec<EventProcessor>,
    task_locals: Option<TaskLocals>,
    msg_server_rx: tokio::sync::watch::Receiver<String>,
}

impl Application {
    pub fn new(name: &str, msg_rx: tokio::sync::watch::Receiver<String>) -> Self {
        Self {
            name: name.to_string(),
            event_processors: Vec::new(),
            task_locals: None,
            msg_server_rx: msg_rx,
        }
    }

    pub async fn start<T: Transporter>(&mut self) {
        debug!("Starting application: {}", self.name);
        let task_locals_copy = self.task_locals.clone().unwrap();
        let message_handlers = self.event_processors.clone();

        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        let mut msg_porter = T::new(tx.clone(), self.name.clone());
        let tx = msg_porter.get_tx();

        let mut msg_server_rx = self.msg_server_rx.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    msg = msg_server_rx.changed() => {
                        match msg {
                    Ok(_msg) => {
                        let data = msg_server_rx.borrow().to_string();
                        debug!("[Server] Sent Dispatch Message to [Application]-2: {}", data.clone());
                        match tx.send(data).await {
                            Ok(_) => {
                                debug!("Sent message");
                            }
                            Err(e) => {
                                debug!("error 33 {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        debug!("error 44 {}", e);
                    }
                }
                    }
                }
            }
        });

        let message_processors = self.event_processors.clone();
        tokio::spawn(async move {
            let _mh = message_handlers.clone();
            while let Some(status) = rx.recv().await {
                let (msg, status) = match status {
                    TransportStatus::Data(data) => (data, EventType::Data),
                    TransportStatus::Error(err) => (err, EventType::Error),
                    TransportStatus::Info(info) => (info, EventType::Message),
                    TransportStatus::PeerDiscovered(peer_id) => (peer_id, EventType::SystemEvent),
                    TransportStatus::PeerConnected(peer_id) => (peer_id, EventType::SystemEvent),
                    TransportStatus::PeerDisconnected(peer_id) => (peer_id, EventType::SystemEvent),
                    TransportStatus::Stopped => ("Stopped".to_string(), EventType::Stop),
                    TransportStatus::Started => ("Ready".to_string(), EventType::Start),
                };

                if status == EventType::Data {
                    debug!(
                        "[Application] Received Income Message from [Transporter]-2: {}",
                        msg.clone()
                    );
                }

                let event = Event::new(msg, status, "SYSTEM".to_string(), OriginatorType::System);

                let input = Python::with_gil(|py| event.clone().into_py(py));

                for mp in message_processors.iter() {
                    let input_copy = input.clone();
                    let tlc = task_locals_copy.clone();
                    let mp2 = mp.clone();
                    tokio::spawn(async move {
                        match execute_process_function(input_copy.clone(), &mp2.function, &tlc)
                            .await
                        {
                            Ok(_) => (info!("Server starting..."),),
                            Err(e) => (debug!("error 55 {}", e),),
                        };
                    });
                }
                debug!("Processing message released");
            }
        });

        match msg_porter.message_processor().await {
            Ok(_) => {
                info!("Message Porter started");
            }
            Err(e) => {
                debug!("Message Porter error 66 {}", e);
            }
        };
    }

    pub fn initialize(&mut self, task_local: TaskLocals) {
        self.task_locals = Some(task_local);
    }

    pub fn shutdown(&mut self) {}

    pub fn add_event_processor(&mut self, mp: EventProcessor) {
        self.event_processors.push(mp);
    }
    pub fn remove_message_handler(&mut self, mp: EventProcessor) {
        for i in 0..self.event_processors.len() {
            if self.event_processors[i].name == mp.name {
                self.event_processors.remove(i);
                break;
            }
        }
    }
}
