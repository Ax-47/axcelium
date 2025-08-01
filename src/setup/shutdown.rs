use tokio::{
    signal::unix::{SignalKind, signal},
    sync::watch,
};

pub fn spawn_signal_handler(shutdown_tx: watch::Sender<bool>) {
    tokio::spawn(async move {
        let mut sigterm = signal(SignalKind::terminate()).expect("Failed to register SIGTERM");
        let mut sigint = signal(SignalKind::interrupt()).expect("Failed to register SIGINT");

        tokio::select! {
            _ = sigterm.recv() => println!("Received SIGTERM"),
            _ = sigint.recv() => println!("Received SIGINT"),
        }

        let _ = shutdown_tx.send(true);
    });
}
