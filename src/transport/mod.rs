pub mod bonjour;
pub mod ip;

pub trait Transport {
    fn start();
    fn stop();
}
