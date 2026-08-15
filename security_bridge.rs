// Modulo di Automazione Sicura Z-DOS per Nginx & Certbot
use std::process::Command;

pub fn avvia_perimetro_sicurezza(dominio: &str) {
    println!("[Z-DOS SECURITY] Avvio configurazione Nginx e Certbot per: {}", dominio);
    
    // Attivazione del servizio Nginx
    let _ = Command::new("systemctl")
        .args(["start", "nginx"])
        .status();

    // Richiesta certificato SSL crittografato tramite Certbot
    let status = Command::new("certbot")
        .args(["--nginx", "-d", dominio, "--non-interactive", "--agree-tos", "-m", "admin@x-zdos.it"])
        .status();

    if status.is_ok() {
        println!("[Z-DOS SECURITY] Perimetro protetto con successo per {}", dominio);
    } else {
        println!("[Z-DOS SECURITY] Errore critico durante l'handshake SSL.");
    }
}
