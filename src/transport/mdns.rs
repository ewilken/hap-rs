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
    pub fn new(name: String, txt_records: [String; 8]) -> Self {
        Responder {
            name,
            txt_records,
            stop: None,
        }
    }

    pub fn start(&self) {
        let (tx, rx) = mpsc::channel();
        let name = self.name;
        let txt_records = self.txt_records;
        thread::spawn(move || {
            let responder = mdns::Responder::new().unwrap();
            let _svc = responder.register(
                "_hap._tcp".into(),
                name,
                55123,
                &["c#=1", "ff=1", "id=4c:32:75:98:44:e9", "md=Device", "s#=1", "sf=1", "ci=7"],
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
        if let Some(stop) = self.stop {
            stop.send(());
        }
    }
}
