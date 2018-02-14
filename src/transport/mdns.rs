use std::thread;
use std::sync::mpsc::{self, TryRecvError};
use std::time::Duration;
use mdns;

pub struct Responder {
    name: String,
    port: u16,
    txt_records: [String; 8],
    stop: Option<mpsc::Sender<()>>,
}

impl Responder {
    pub fn new(name: &String, port: &u16, txt_records: [String; 8]) -> Self {
        Responder {
            name: name.clone(),
            port: port.clone(),
            txt_records,
            stop: None,
        }
    }

    pub fn start(&mut self) {
        let (tx, rx) = mpsc::channel();
        let name = self.name.clone();
        let port = self.port.clone();
        let tr = self.txt_records.clone();
        thread::spawn(move || {
            let responder = mdns::Responder::new().unwrap();
            let _svc = responder.register(
                "_hap._tcp".into(),
                name,
                port,
                &[&tr[0], &tr[1], &tr[2], &tr[3], &tr[4], &tr[5], &tr[6], &tr[7]],
            );
            loop {
                thread::sleep(Duration::from_secs(10));
                match rx.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        break;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
        });
        self.stop = Some(tx);
    }

    pub fn stop(&self) {
        if let Some(stop) = self.stop.clone() {
            stop.send(()).unwrap();
        }
    }
}
