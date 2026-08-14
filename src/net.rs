use std::net::TcpListener;
use std::thread;

pub struct GhostSocket;

impl GhostSocket {
    pub fn bind(port: &str) {
        let address = format!("127.0.0.1:{}", port);
        println!("[GHOST-SOCKET] 🕸️ Inizializzazione demone di rete su {}...", address);
        
        // Apriamo il socket in un thread separato per non bloccare la Z-SHELL
        thread::spawn(move || {
            if let Ok(listener) = TcpListener::bind(&address) {
                // Il nodo è fisicamente in ascolto
                for _stream in listener.incoming() {
                    // Qui gestiremo i peer in futuro
                }
            }
        });
        println!("[GHOST-SOCKET] 🟢 Socket TCP aperto in background. Nodo Z-DOS in ascolto.");
    }
}
