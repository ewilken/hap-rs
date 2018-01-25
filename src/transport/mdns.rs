use std::thread;
use std::sync::mpsc::{self, TryRecvError};
use std::time::Duration;
use mdns;

pub struct Responder {
    name: String,
    txt_records: [String; 8],
    stop: Option<mpsc::Sender<()>>,
}

impl Responder {
    pub fn new(name: &String, txt_records: [String; 8]) -> Self {
        Responder {
            name: name.to_owned(),
            txt_records,
            stop: None,
        }
    }

    pub fn start(&mut self) {
        let (tx, rx) = mpsc::channel();
        let name = self.name.to_owned();
        let tr = self.txt_records.to_owned();
        thread::spawn(move || {
            let responder = mdns::Responder::new().unwrap();
            let _svc = responder.register(
                "_hap._tcp".into(),
                name,
                // TODO - maybe randomize port
                55123,
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
        if let Some(stop) = self.stop.to_owned() {
            stop.send(());
        }
    }
}
