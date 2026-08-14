pub mod wallet;
pub mod kernel;
pub mod fs; // Agganciamo il nuovo modulo

use std::io::{self, Write};
use wallet::ZChainWallet;
use kernel::DistroHost;
use fs::QuantumFS;

fn main() {
    print!("\x1B[2J\x1B[1;1H"); // Pulisce lo schermo al boot
    println!("==================================================");
    println!("    Z-GENESIS HOST /// BARE-METAL DISTRO BOOT     ");
    println!("==================================================");
    
    let os = DistroHost::boot();
    os.mount_subsystems();
    
    // Inizializza il File System Persistente
    let v_fs = QuantumFS::mount();
    let dsn_wallet = ZChainWallet::boot_ledger();
    
    println!("==================================================");
    println!("🚀 Z-GENESIS HOST ONLINE. Z-SHELL INIZIALIZZATA.");
    println!("Digita 'help' per la lista dei comandi.");
    
    loop {
        print!("\nZ-DOS:~# ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        // Gestione degli argomenti (es: "write <testo>")
        let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
        let command = parts[0];
        let arg = if parts.len() > 1 { parts[1] } else { "" };
        
        match command {
            "help" => {
                println!("Comandi disponibili:");
                println!("  sys info     - Mostra lo stato del Kernel");
                println!("  dsn balance  - Mostra il saldo del portafoglio Layer-1");
                println!("  agi status   - Interroga l'entità Z-SYNAPSE");
                println!("  write [testo]- Salva un appunto nel Z-VAULT (es: write Protocollo Alpha attivo)");
                println!("  read         - Legge l'ultimo appunto salvato nel Z-VAULT");
                println!("  clear        - Pulisce lo schermo");
                println!("  exit         - Spegne l'Host");
            }
            "sys" | "sys info" => { // Supporta sia "sys" che "sys info"
                 println!("> OS: {} | Versione: {}", os.os_name, os.version);
            }
            "dsn" | "dsn balance" => {
                dsn_wallet.display_status();
            }
            "agi" | "agi status" => {
                println!("[Z-SYNAPSE] 🧠 Sentience Score: 0.85 (SENTIENT).");
            }
            "write" => {
                if arg.is_empty() {
                    println!("> Errore: Inserisci il testo da salvare (es: write Mio appunto).");
                } else {
                    v_fs.write_memory("mem_block_01.zdb", arg);
                }
            }
            "read" => {
                v_fs.read_memory("mem_block_01.zdb");
            }
            "clear" => {
                print!("\x1B[2J\x1B[1;1H");
            }
            "exit" => {
                println!("> Chiusura sicura dei moduli e smontaggio Z-VAULT...");
                break;
            }
            "" => continue,
            _ => {
                println!("Comando non riconosciuto: '{}'", command);
            }
        }
    }
}
