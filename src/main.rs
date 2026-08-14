pub mod wallet;
pub mod kernel;
pub mod fs;
pub mod pkg;
pub mod net;

use std::io::{self, Write};
use wallet::ZChainWallet;
use kernel::DistroHost;
use fs::QuantumFS;
use pkg::ZGet;
use net::GhostSocket;

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    println!("==================================================");
    println!("    Z-GENESIS HOST /// BARE-METAL DISTRO BOOT     ");
    println!("==================================================");
    
    let os = DistroHost::boot();
    os.mount_subsystems();
    
    let v_fs = QuantumFS::mount();
    let mut dsn_wallet = ZChainWallet::boot_ledger();
    
    println!("==================================================");
    println!("🚀 Z-GENESIS HOST ONLINE. Z-SHELL INIZIALIZZATA.");
    println!("Digita 'help' per la lista dei comandi.");
    
    loop {
        print!("\nZ-DOS:~# ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() { continue; }
        
        let command = parts[0];
        
        match command {
            "help" => {
                println!("Comandi disponibili:");
                println!("  sys        - Mostra lo stato del Kernel");
                println!("  balance    - Mostra il saldo del portafoglio Layer-1");
                println!("  transfer   - Firma transazione $DSN (es: transfer 0x123 50)");
                println!("  zget       - Installa un sottomodulo (es: zget z-ai)");
                println!("  net        - Apre il socket di rete (es: net start 8080)");
                println!("  write      - Salva nel Z-VAULT (es: write test)");
                println!("  read       - Legge dal Z-VAULT");
                println!("  clear      - Pulisce lo schermo");
                println!("  standby    - Entra in Standby Tattico e spegne l'Host");
            }
            "sys" => println!("> OS: {} | Versione: {}", os.os_name, os.version),
            "balance" => dsn_wallet.display_status(),
            "write" => {
                if parts.len() > 1 {
                    let text = parts[1..].join(" ");
                    v_fs.write_memory("mem_block_01.zdb", &text);
                }
            }
            "read" => v_fs.read_memory("mem_block_01.zdb"),
            "transfer" => {
                if parts.len() == 3 {
                    if let Ok(amount) = parts[2].parse::<f64>() {
                        dsn_wallet.sign_transaction(parts[1], amount);
                    }
                }
            }
            "zget" => {
                if parts.len() == 2 {
                    ZGet::install(parts[1]);
                } else {
                    println!("> Errore sintassi. Usa: zget <nome_modulo>");
                }
            }
            "net" => {
                if parts.len() == 3 && parts[1] == "start" {
                    GhostSocket::bind(parts[2]);
                } else {
                    println!("> Errore sintassi. Usa: net start <porta>");
                }
            }
            "clear" => print!("\x1B[2J\x1B[1;1H"),
            "standby" | "exit" => {
                println!("==================================================");
                println!("> ⚠️ INIZIALIZZAZIONE STANDBY TATTICO...");
                println!("> Smontaggio Z-VAULT... OK");
                println!("> Chiusura Ghost-Socket... OK");
                println!("> Ibernazione Z-SYNAPSE AGI... OK");
                println!("> 🌙 Z-GENESIS HOST OFFLINE. BUONANOTTE, COMANDANTE.");
                println!("==================================================");
                break;
            }
            _ => println!("Comando non riconosciuto. Digita 'help'."),
        }
    }
}
