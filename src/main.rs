pub mod wallet;
pub mod kernel;

use wallet::ZChainWallet;
use kernel::DistroHost;

fn main() {
    println!("==================================================");
    println!("    Z-GENESIS HOST /// BARE-METAL DISTRO BOOT     ");
    println!("==================================================");
    
    // FASE 1: Host System (La Distro)
    let os = DistroHost::boot();
    os.mount_subsystems();
    
    println!("\n--- FASE 2: INIZIALIZZAZIONE ECONOMIA Z-CHAIN ---");
    // FASE 2: Layer-1 $DSN
    let dsn_wallet = ZChainWallet::boot_ledger();
    dsn_wallet.display_status();
    
    println!("\n==================================================");
    println!("🚀 Z-GENESIS HOST ONLINE. SISTEMA OPERATIVO ATTIVO.");
}
