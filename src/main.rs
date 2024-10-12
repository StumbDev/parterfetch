use colored::*; // For colorized output
use sysinfo::{System, SystemExt, CpuExt, DiskExt, MemoryExt};
use whoami;
use std::env;
use std::path::PathBuf;

fn parterfetch() {
    // ASCII logo for Partermai
    let logo = r#"
 ____              _                  _       
|  __|  _ __   ___| |_ _ __ __ _ _ __ (_) __ _ 
| |_   | '_ \ / _ \ __| '__/ _` | '_ \| |/ _` |
|  _|  | | | |  __/ |_| | | (_| | | | | | (_| |
|_|    |_| |_|\___|\__|_|  \__,_|_| |_|_|\__,_|
    "#;
    
    // Display the colored ASCII logo
    println!("{}", logo.bright_yellow());
    
    // Create a system object to fetch system information
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Fetch the OS and Kernel version
    let os_name = sys.name().unwrap_or("Unknown OS".to_string());
    let kernel_version = sys.kernel_version().unwrap_or("Unknown Kernel".to_string());
    
    // Fetch CPU information
    let cpu_brand = sys.global_cpu_info().brand();
    
    // Fetch Memory information
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    
    // Fetch Disk usage
    let total_disk = sys.disks().iter().map(|d| d.total_space()).sum::<u64>();
    let used_disk = sys.disks().iter().map(|d| d.available_space()).sum::<u64>();
    
    // Get user information
    let username = whoami::username();
    let hostname = whoami::hostname();

    // Get installation path of Partermai
    let install_path: PathBuf = match env::current_exe() {
        Ok(path) => path,
        Err(_) => PathBuf::from("Unknown Path"),
    };

    // Display the system information with colors
    println!("{}", "Parterfetch:".gradient(Gradient::Rainbow));
    println!("{}: {}", "User".bright_yellow(), username.bright_green());
    println!("{}: {}", "Host".bright_yellow(), hostname.bright_green());
    println!("{}: {}", "Operating System".bright_yellow(), os_name.bright_green());
    println!("{}: {}", "Kernel Version".bright_yellow(), kernel_version.bright_green());
    println!("{}: {}", "CPU".bright_yellow(), cpu_brand.bright_green());
    println!(
        "{}: {} / {} MB",
        "Memory Usage".bright_yellow(),
        used_memory.bright_green(),
        total_memory.bright_green()
    );
    println!(
        "{}: {} / {} GB",
        "Disk Usage".bright_yellow(),
        (total_disk - used_disk) / 1_000_000_000,
        total_disk / 1_000_000_000
    );
    println!("{}: {}", "Installation Path".bright_yellow(), install_path.display().to_string().bright_green());
}
