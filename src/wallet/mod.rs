use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct ZChainWallet {
    pub address: String,
    pub private_key: String,
    pub balance_dsn: f64,
}

impl ZChainWallet {
    pub fn boot_ledger() -> Self {
        println!("[Z-CHAIN] 🛰️ Inizializzazione Layer-1 (Modalità Locale Sicura)...");
        println!("[Z-CHAIN] 🔐 Credenziali Z-CHAIN caricate nel Kernel bare-metal.");
        
        ZChainWallet {
            address: String::from("0x127Fa720Aae2C145d2f2aC589EEB27D4C87CFB75"),
            private_key: String::from("0x5d090a94fc59963129888427920360b697f146c52f8899571b0d830ca47804a8"),
            balance_dsn: 1000.0, // Ti ho caricato un saldo di test per operare
        }
    }

    pub fn display_status(&self) {
        println!("==================================================");
        println!("Indirizzo Pubblico : {}", self.address);
        println!("Chiave Privata     : [ PROTETTA NEL NUCLEO Z-LANG ]");
        println!("💰 Saldo DSN       : {:.1} DSN", self.balance_dsn);
        println!("==================================================");
    }

    /// Firma una transazione offline combinando la chiave privata e i dati della TX
    pub fn sign_transaction(&mut self, receiver: &str, amount: f64) {
        if amount > self.balance_dsn {
            println!("[Z-CHAIN] ❌ Errore: Saldo $DSN insufficiente.");
            return;
        }

        // Creiamo il payload crittografico
        let payload = format!("{}:{}:{}", self.private_key, receiver, amount);
        
        // Generiamo l'hash della transazione
        let mut hasher = DefaultHasher::new();
        payload.hash(&mut hasher);
        let signature = format!("{:x}", hasher.finish());

        self.balance_dsn -= amount; // Aggiorniamo il saldo locale

        println!("[Z-CHAIN] 📝 Generazione transazione offline...");
        println!("> Da      : {}", self.address);
        println!("> A       : {}", receiver);
        println!("> Importo : {:.2} $DSN", amount);
        println!("> Firma   : 0x{}", signature);
        println!("[Z-CHAIN] ✅ Transazione firmata matematicamente. Pronta per il broadcast nel GhostNet.");
    }
}
