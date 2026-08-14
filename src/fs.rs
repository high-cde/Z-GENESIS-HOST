use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::Path;

pub struct QuantumFS {
    pub root_dir: String,
}

impl QuantumFS {
    pub fn mount() -> Self {
        let root = "./z_vault";
        if !Path::new(root).exists() {
            fs::create_dir(root).expect("[FS ERROR] Impossibile creare z_vault");
            println!("[FILE SYSTEM] 📂 Z-VAULT partizione creata con successo.");
        } else {
            println!("[FILE SYSTEM] 📂 Z-VAULT montato. Memoria persistente attiva.");
        }

        QuantumFS {
            root_dir: String::from(root),
        }
    }

    /// Simula la compressione AAAK e scrive su disco
    pub fn write_memory(&self, filename: &str, data: &str) {
        let filepath = format!("{}/{}", self.root_dir, filename);
        let mut file = File::create(&filepath).expect("[FS ERROR] Creazione file fallita");
        
        // Simbolizziamo la compressione AAAK avvolgendo i dati
        let compressed_data = format!("<<[AAAK_COMPRESSED_BLOCK::]>>\n{}", data);
        file.write_all(compressed_data.as_bytes()).expect("[FS ERROR] Scrittura fallita");
        println!("> 💾 Memoria archiviata nel vault: {}", filepath);
    }

    /// Legge il file e ne rimuove il wrapper AAAK
    pub fn read_memory(&self, filename: &str) {
        let filepath = format!("{}/{}", self.root_dir, filename);
        if Path::new(&filepath).exists() {
            let mut file = File::open(&filepath).expect("[FS ERROR] Lettura fallita");
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            
            // Rimuoviamo il tag AAAK per la visualizzazione
            let decoded = contents.replace("<<[AAAK_COMPRESSED_BLOCK::]>>\n", "");
            println!("--- CONTENUTO DI {} ---", filename);
            println!("{}", decoded);
            println!("-----------------------------");
        } else {
            println!("> ⚠️ Memoria '{}' non trovata nel vault.", filename);
        }
    }
}
