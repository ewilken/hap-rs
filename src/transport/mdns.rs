use std::{thread, sync::mpsc::{self, TryRecvError}, time::Duration, rc::Rc, cell::RefCell};

use libmdns;

use Error;

/// An mDNS Responder. Used to announce the Accessory's name and HAP TXT records to potential
/// controllers.
pub struct Responder {
    name: String,
    port: u16,
    txt_records: [String; 8],
    stop: Option<mpsc::Sender<()>>,
}

impl Responder {
    /// Creates a new mDNS Responder.
    pub fn new(name: &str, port: u16, txt_records: [String; 8]) -> Self {
        Responder {
            name: name.to_string(),
            port,
            txt_records,
            stop: None,
        }
    }

    /// Starts mDNS announcement in a separate thread.
    pub fn start(&mut self) {
        let (tx, rx) = mpsc::channel();
        let name = self.name.clone();
        let port = self.port;
        let tr = self.txt_records.clone();
        thread::spawn(move || {
            let responder = libmdns::Responder::new().expect("couldn't create mDNS responder");
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

    /// Stops mDNS announcement.
    pub fn stop(&self) -> Result<(), Error> {
        if let Some(stop) = self.stop.clone() {
            stop.send(())?;
        }
        Ok(())
    }

    /// Stops mDNS announcement and restarts it with updated TXT records.
    pub fn update_txt_records(&mut self, txt_records: [String; 8]) -> Result<(), Error> {
        self.stop()?;
        self.txt_records = txt_records;
        self.start();
        Ok(())
    }
}

/// Reference counting pointer to a `Responder`.
pub type ResponderPtr = Rc<RefCell<Responder>>;
