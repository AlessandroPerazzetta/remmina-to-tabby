use clap::Parser;
// use std::path::Path;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};

mod remmina_parser;
mod remmina_types;
use remmina_types::{RemminaFiles, RemminaProfile};
mod tabby_parser;
use tabby_parser::TabbyConfig;

mod ascii_art;
use ascii_art::show_ascii_art_header;
mod protocols_types;

/// Remmina to Tabby converter
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to Remmina directory
    #[arg(long, default_value_t = default_remmina_dir())]
    remmina_dir: String,

    /// Path to Tabby directory
    #[arg(long, default_value_t = default_tabby_dir())]
    tabby_dir: String,

    /// Protocol to filter (e.g. SSH, RDP, VNC)
    #[arg(long, default_value = "SSH")]
    protocol: String,

    /// Check protocols in Remmina files
    #[arg(long, default_value_t = false)]
    remmina_check: bool,

    /// Really execute export (otherwise dry-run)
    #[arg(long, default_value_t = false)]
    execute: bool,

    /// Skip all confirmations
    #[arg(long, default_value_t = false)]
    yes: bool,
}

fn main() {
    show_ascii_art_header();

    let args = Args::parse();

    let remmina_dir = &args.remmina_dir;
    let tabby_dir = &args.tabby_dir;
    let protocol_arg = &args.protocol;
    
    println!("Remmina dir: {}", remmina_dir);
    println!("Tabby dir: {}", tabby_dir);
    println!("Protocol filter: {}", protocol_arg);

    if !Path::new(remmina_dir).is_dir() {
        eprintln!("\n 🚫 Error: Remmina directory '{}' does not exist or is not a directory.\n", remmina_dir);
        std::process::exit(1);
    }

    if !Path::new(tabby_dir).is_dir() {
        eprintln!("\n 🚫 Error: Tabby directory '{}' does not exist or is not a directory.\n", tabby_dir);
        std::process::exit(1);
    }

    let mut tabby_config = match TabbyConfig::load_from_dir(tabby_dir) {
        Ok(config) => {
            println!("\nLoaded Tabby config from {}\n", tabby_dir);

            config
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    println!("Current number of Tabby profiles: {} and {} groups.", tabby_config.profiles.len(), tabby_config.groups.as_ref().map_or(0, |g| g.len()));
    if !args.yes { confirm_continue(Some("\nDo you want to continue with export from Remmina?")); }
    

    // Split protocol argument by comma and trim whitespace
    let protocols: Vec<String> = protocol_arg
        .split(',')
        .map(|s| s.trim().to_uppercase())
        .collect();

    // Find and print .remmina files filtered by protocols
    // let remmina_files = RemminaFiles::find(remmina_dir).filter_by_protocols(&protocols);

    // Find .remmina files with proper error handling
    let remmina_files = match RemminaFiles::find(remmina_dir) {
        Ok(files) => {
            println!("\nFound {} .remmina files\n", files.files.len());
            let filtered_files = files.filter_by_protocols(&protocols);
            println!("After filtering, {} .remmina files match protocols: {:?}\n", filtered_files.files.len(), protocols);
            filtered_files
        }
        Err(e) => {
            eprintln!("\n🚫 Error reading Remmina directory '{}': {}\n", remmina_dir, e);
            std::process::exit(1);
        }
    };


    if args.remmina_check {
        remmina_files.check_protocols();
    }

    // if args.execute {
    //     println!("Executing export_profiles...");
    //     remmina_files.export_profiles(true);
    // } else {
    //     println!("Dry-run would export the following profiles:");
    //     remmina_files.export_profiles(false);
    // }

    // for remmina_file in remmina_files.files {
    //     println!("Found Remmina file: {}", remmina_file.display());
    // }

    let remmina_profiles: Vec<RemminaProfile>  = remmina_files.export_profiles();
    if remmina_profiles.len() == 0 {
        println!("\n🟡 No Remmina profiles found with protocol(s): {:?}\n", protocols);
        return;
    }
    if remmina_profiles.len() > 0 {
        println!("\n✅ Exported {} profiles from Remmina files.\n", remmina_profiles.len());
    } else {
        println!("\n🛑 No Remmina profiles exorted");
        return;
    }
    
    
    // for profile in &remmina_profiles {
    //     println!(
    //         "Profile: name={:?}, server={:?}, group={:?}, protocol={:?}, path={}",
    //         profile.name,
    //         profile.server,
    //         profile.group,
    //         profile.protocol,
    //         profile.path.display()
    //     );
    // }

    if !args.yes { confirm_continue(Some("\nDo you want to continue with import into Tabby config?")); }

    // Make a backup copy of config.yaml for Tabby
    let config_path = PathBuf::from(tabby_dir).join("config.yaml");
    let backup_path = PathBuf::from(tabby_dir).join("config.yaml.bak");
    
    if args.execute {
        if config_path.exists() {
            match fs::copy(&config_path, &backup_path) {
                Ok(_) => println!("\nBackup of {} created: {}\n", config_path.display(), backup_path.display()),
                Err(e) => eprintln!("Failed to create backup: {}", e),
            }
        } else {
            println!("❗❗❗ No config.yaml found to backup in {}", tabby_dir);
        }
    } else {
        println!("Dry-run would create backup of {} to {}", config_path.display(), backup_path.display());
    }

    // Import Remmina profiles into Tabby config
    let imported_count = tabby_config.import_profiles(remmina_profiles);
    if imported_count == 0 {
        println!("\n🟡 No new profiles were imported into Tabby config (all already exist).\n");
        return;
    } else {
        println!("\n✅ Imported {} new profiles into Tabby config.\n", imported_count);
    }

    if args.execute {
        // Save updated Tabby config back to config.yaml
        tabby_config.save_to_path(&config_path.to_string_lossy())
            .unwrap_or_else(|err| {
                eprintln!("Failed to save Tabby config: {}", err);
                std::process::exit(1);
            });
        println!("Tabby config saved to {}", config_path.display());
    } else {
        println!("Dry-run would save updated Tabby config to {}", config_path.display());
    }


    if let Some(profile) = tabby_config.get_profile("host011") {
        println!("Found profile: {:#?}", profile);
        let group_id = profile.group.as_deref().unwrap_or("No group");
        if let Some(groups) = &tabby_config.groups {
            if let Some(group) = groups.iter().find(|g| g.id == group_id) {
                println!("Profile belongs to group: {}", group.name);  
            } else {
                println!("Group with id '{}' not found", group_id);
            }
        } else {
            println!("No groups defined in the config");
        }
    } else {
        println!("Profile {} not found", "host011");
    }




}

/// Get default Remmina directory based on OS (Currently only Linux supported)
fn default_remmina_dir() -> String {
    #[cfg(target_os = "linux")]
    {
        format!("{}/.local/share/remmina", std::env::var("HOME").unwrap_or_default())
    }
}

/// Get default Tabby directory based on OS
/// # Returns
/// * `String` - Default Tabby configuration directory path
/// # Behavior
/// * On Linux: ~/.config/tabby
/// * On Windows: %APPDATA%\Tabby
/// * On macOS: ~/Library/Application Support/tabby
/// * On other OS: ./tabby
fn default_tabby_dir() -> String {
    #[cfg(target_os = "linux")]
    {
        format!("{}/.config/tabby", std::env::var("HOME").unwrap_or_default())
    }
    #[cfg(target_os = "windows")]
    {
        format!("{}/Tabby", std::env::var("APPDATA").unwrap_or_default())
    }
    #[cfg(target_os = "macos")]
    {
        format!("{}/Library/Application Support/tabby", std::env::var("HOME").unwrap_or_default())
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        String::from("./tabby")
    }
}

fn confirm_continue(message: Option<&str>) {
    if let Some(msg) = message {
        println!("{}", msg);
    }
    print!("⚠️  Press [Enter] to continue or 'q' then [Enter] to quit: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim().eq_ignore_ascii_case("q") {
        println!("🛑 Operation cancelled by user.");
        std::process::exit(0);
    }
}