use std::time::{Duration, Instant};

fn main() {
    println!("=============================================");
    println!("           SCREENSHOT DEMO");
    println!("=============================================");
    println!(" ");
    println!("██╗    ██╗███████╗██╗      ██████╗ ██████╗ ███╗   ███╗███████╗");
    println!("██║    ██║██╔════╝██║     ██╔════╝██╔═══██╗████╗ ████║██╔════╝");
    println!("██║ █╗ ██║█████╗  ██║     ██║     ██║   ██║██╔████╔██║█████╗  ");
    println!("██║███╗██║██╔══╝  ██║     ██║     ██║   ██║██║╚██╔╝██║██╔══╝  ");
    println!("╚███╔███╔╝███████╗███████╗╚██████╗╚██████╔╝██║ ╚═╝ ██║███████╗");
    println!(" ╚══╝╚══╝ ╚══════╝╚══════╝ ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚══════╝");
    println!(" ");
    println!("This is a sample program output that can be captured with Print Screen.");
    println!(" ");
    println!("█▀▀ █▀█ █▀▀ ▀█▀ █▀▀ █▀█   █▀▄ █▀▀ █▀▀ █▀█ █▀▀ █▀▄");
    println!("█▄▄ █▄█ ██▄  █  ██▄ █▄█   █▄▀ ██▄ █▄▄ █▄█ ██▄ █▄▀");
    println!(" ");
    println!("ASCII Art Generated Successfully!");
    println!(" ");
    println!("Run this program and press Print Screen to capture the output.");
    println!(" ");
    println!("Progress: [████████████████████] 100%");
    println!("Status: Ready for screenshot");
    println!(" ");
    
    // Show some interesting data
    let start_time = Instant::now();
    let elapsed = start_time.elapsed();
    println!("Duration: {:?}", elapsed);
    println!("Process ID: {}", std::process::id());
    println!("Rust Version: 1.70+");
    println!("Target: x86_64-unknown-linux-gnu");
    
    println!(" ");
    println!("██████████████████████████████████████████████████████████████");
    println!("█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░█");
    println!("█░░██░░██░░██░░██░░██░░██░░██░░██░░██░░██░░██░░██░░██░░██░░░░█");
    println!("█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░█");
    println!("█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░█");
    println!("█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░█");
    println!("██████████████████████████████████████████████████████████████");
    println!(" ");
    
    println!("Screenshot ready! Press Print Screen key to capture.");
    println!("This window will remain open for 10 seconds...");
    
    std::thread::sleep(Duration::from_secs(10));
}