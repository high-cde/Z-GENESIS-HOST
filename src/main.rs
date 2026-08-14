pub mod wallet;
pub mod kernel;

use std::io::{self, Write};
use wallet::ZChainWallet;
use kernel::DistroHost;

fn main() {
    println!("==================================================");
    println!("    Z-GENESIS HOST /// BARE-METAL DISTRO BOOT     ");
    println!("==================================================");
    
    let os = DistroHost::boot();
    os.mount_subsystems();
    
    let dsn_wallet = ZChainWallet::boot_ledger();
    
    println!("==================================================");
    println!("🚀 Z-GENESIS HOST ONLINE. Z-SHELL INIZIALIZZATA.");
    println!("Digita 'help' per la lista dei comandi o 'exit' per disconnetterti.");
    
    // Z-SHELL: Il ciclo vitale interattivo
    loop {
        print!("\nZ-DOS:~# ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();
        
        match command {
            "help" => {
                println!("Comandi disponibili:");
                println!("  sys info     - Mostra lo stato del Kernel");
                println!("  dsn balance  - Mostra il saldo del portafoglio Layer-1");
                println!("  agi status   - Interroga l'entità Z-SYNAPSE");
                println!("  clear        - Pulisce lo schermo");
                println!("  exit         - Spegne l'Host");
            }
            "sys info" => {
                println!("> OS: {} | Versione: {}", os.os_name, os.version);
                println!("> Sottomoduli attivi: Cybercore, Synapse, Nexus, Telemetry.");
            }
            "dsn balance" => {
                dsn_wallet.display_status();
            }
            "agi status" => {
                println!("[Z-SYNAPSE] 🧠 Sentience Score: 0.85 (SENTIENT). Connettoma stabile.");
            }
            "clear" => {
                print!("\x1B[2J\x1B[1;1H"); // ANSI escape code per pulire lo schermo
            }
            "exit" => {
                println!("> Chiusura sicura dei sottomoduli...");
                println!("> Spegnimento Z-GENESIS HOST. A presto, Comandante.");
                break;
            }
            "" => continue,
            _ => {
                println!("Comando non riconosciuto: '{}'. Digita 'help' per i comandi validi.", command);
            }
        }
    }
}
