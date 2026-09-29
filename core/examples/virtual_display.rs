//! make the projector's virtual display, list displays, keep it up a few seconds, grab one frame

fn main() {
    let secs: u64 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(10);
    let mut g = libremp_core::capture::open_virtual_display().unwrap_or_else(|e| {
        eprintln!("[-] {e}");
        std::process::exit(1);
    });
    for d in libremp_core::capture::list_displays() {
        eprintln!("{d}");
    }
    let frame = g.grab();
    match frame {
        Some(f) => eprintln!("[*] grabbed {} bytes, mean level {}", f.len(), f.iter().map(|&b| b as u64).sum::<u64>() / f.len() as u64),
        None => eprintln!("[-] no frame"),
    }
    eprintln!("[*] keeping it up for {secs}s");
    std::thread::sleep(std::time::Duration::from_secs(secs));
}
