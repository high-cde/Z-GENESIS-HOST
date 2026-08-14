pub mod wallet;
pub mod kernel;
pub mod fs;

use std::io::{self, Write};
use wallet::ZChainWallet;
use kernel::DistroHost;
use fs::QuantumFS;

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    println!("==================================================");
    println!("    Z-GENESIS HOST /// BARE-METAL DISTRO BOOT     ");
    println!("==================================================");
    
    let os = DistroHost::boot();
    os.mount_subsystems();
    
    let v_fs = QuantumFS::mount();
    let mut dsn_wallet = ZChainWallet::boot_ledger(); // Ora è mutabile per aggiornare il saldo
    
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
                println!("  transfer   - Firma transazione (es: transfer 0x123... 50)");
                println!("  agi        - Interroga l'entità Z-SYNAPSE");
                println!("  write      - Salva un appunto nel Z-VAULT (es: write test)");
                println!("  read       - Legge l'ultimo file dal Z-VAULT");
                println!("  clear      - Pulisce lo schermo");
                println!("  exit       - Spegne l'Host");
            }
            "sys" => println!("> OS: {} | Versione: {}", os.os_name, os.version),
            "balance" => dsn_wallet.display_status(),
            "agi" => println!("[Z-SYNAPSE] 🧠 Sentience Score: 0.85 (SENTIENT)."),
            "write" => {
                if parts.len() > 1 {
                    let text = parts[1..].join(" ");
                    v_fs.write_memory("mem_block_01.zdb", &text);
                } else {
                    println!("> Errore sintassi. Usa: write <testo>");
                }
            }
            "read" => v_fs.read_memory("mem_block_01.zdb"),
            "transfer" => {
                if parts.len() == 3 {
                    let receiver = parts[1];
                    if let Ok(amount) = parts[2].parse::<f64>() {
                        dsn_wallet.sign_transaction(receiver, amount);
                    } else {
                        println!("> Errore: l'importo deve essere un numero.");
                    }
                } else {
                    println!("> Errore sintassi. Usa: transfer <indirizzo_destinazione> <importo>");
                }
            }
            "clear" => print!("\x1B[2J\x1B[1;1H"),
            "exit" => {
                println!("> Chiusura sicura...");
                break;
            }
            _ => println!("Comando non riconosciuto. Digita 'help'."),
        }
    }
}
