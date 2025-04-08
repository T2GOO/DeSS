use std::process::{Command, Child, Stdio};
use std::io;
use std::time::Duration;
use std::thread;

use home;
use crate::utils::constants::IPFS_CONFIG_PATH_SWARM_SUFF;

/// Lance le daemon IPFS en arrière-plan.
/// Retourne un `Child` process handle qu'on peut utiliser pour le stopper.
pub fn start_ipfs_daemon() -> io::Result<Child> {
    let ipfs_path = home::home_dir().unwrap()
        .join(IPFS_CONFIG_PATH_SWARM_SUFF);

    let child = Command::new("ipfs")
        .arg("daemon")
        .env("IPFS_PATH", ipfs_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    println!("🚀 IPFS daemon lancé (PID: {})", child.id());

    // Optionnel : attendre un petit moment pour laisser le daemon démarrer
    thread::sleep(Duration::from_secs(2));

    Ok(child)
}

/// Arrête proprement le daemon IPFS lancé avec `start_ipfs_daemon`
pub fn stop_ipfs_daemon(child: &mut Child) -> io::Result<()> {
    // Envoie un SIGINT (Ctrl+C)
    #[cfg(unix)]
    {
        use nix::sys::signal;
        use nix::sys::signal::Signal;
        use nix::unistd::Pid;

        let pid = Pid::from_raw(child.id() as i32);
        signal::kill(pid, Signal::SIGINT)?;
        println!("🛑 Signal SIGINT envoyé au daemon IPFS");
    }

    #[cfg(windows)]
    {
        child.kill()?; // Pas de SIGINT sur Windows, on kill
        println!("🛑 IPFS daemon killed (Windows)");
    }

    let status = child.wait()?;
    println!("💤 IPFS daemon terminé avec code: {}", status);
    Ok(())
}
