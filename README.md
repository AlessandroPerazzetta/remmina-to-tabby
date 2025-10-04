██▀███  ▓█████  ███▄ ▄███▓ ███▄ ▄███▓ ██▓ ███▄    █  ▄▄▄         ▄▄▄█████▓ ▒█████     ▄▄▄█████▓ ▄▄▄       ▄▄▄▄    ▄▄▄▄ ▓██   ██▓
▓██ ▒ ██▒▓█   ▀ ▓██▒▀█▀ ██▒▓██▒▀█▀ ██▒▓██▒ ██ ▀█   █ ▒████▄       ▓  ██▒ ▓▒▒██▒  ██▒   ▓  ██▒ ▓▒▒████▄    ▓█████▄ ▓█████▄▒██  ██▒
▓██ ░▄█ ▒▒███   ▓██    ▓██░▓██    ▓██░▒██▒▓██  ▀█ ██▒▒██  ▀█▄     ▒ ▓██░ ▒░▒██░  ██▒   ▒ ▓██░ ▒░▒██  ▀█▄  ▒██▒ ▄██▒██▒ ▄██▒██ ██░
▒██▀▀█▄  ▒▓█  ▄ ▒██    ▒██ ▒██    ▒██ ░██░▓██▒  ▐▌██▒░██▄▄▄▄██    ░ ▓██▓ ░ ▒██   ██░   ░ ▓██▓ ░ ░██▄▄▄▄██ ▒██░█▀  ▒██░█▀  ░ ▐██▓░
░██▓ ▒██▒░▒████▒▒██▒   ░██▒▒██▒   ░██▒░██░▒██░   ▓██░ ▓█   ▓██▒     ▒██▒ ░ ░ ████▓▒░     ▒██▒ ░  ▓█   ▓██▒░▓█  ▀█▓░▓█  ▀█▓░ ██▒▓░
░ ▒▓ ░▒▓░░░ ▒░ ░░ ▒░   ░  ░░ ▒░   ░  ░░▓  ░ ▒░   ▒ ▒  ▒▒   ▓▒█░     ▒ ░░   ░ ▒░▒░▒░      ▒ ░░    ▒▒   ▓▒█░░▒▓███▀▒░▒▓███▀▒ ██▒▒▒ 
  ░▒ ░ ▒░ ░ ░  ░░  ░      ░░  ░      ░ ▒ ░░ ░░   ░ ▒░  ▒   ▒▒ ░       ░      ░ ▒ ▒░        ░      ▒   ▒▒ ░▒░▒   ░ ▒░▒   ░▓██ ░▒░ 
  ░░   ░    ░   ░      ░   ░      ░    ▒ ░   ░   ░ ░   ░   ▒        ░      ░ ░ ░ ▒       ░        ░   ▒    ░    ░  ░    ░▒ ▒ ░░  
   ░        ░  ░       ░          ░    ░           ░       ░  ░                ░ ░                    ░  ░ ░       ░     ░ ░     
                                                                                                                ░       ░░ ░     
# REMMINA TO TABBY

**Remmina To Tabby** is a Rust CLI tool to migrate your remote connection profiles from [Remmina](https://remmina.org/) to [Tabby](https://tabby.sh/).

An over engineered remmina profiles parser written in Rust because why not, it simply take one or many remmina profiles extract into a tabby parser that handle yaml config internally with serde-yaml-ng.

> **Note:**  
> This project uses `serde-yaml-ng` due to [serde-yaml deprecation](https://github.com/dtolnay/serde-yaml/releases/tag/0.9.34).

---

## Features

- **Export Remmina Profiles:**  
  Scans your Remmina directory for `.remmina` files and extracts SSH, RDP, and VNC profiles.

- **Import to Tabby:**  
  Converts and imports supported profiles into your Tabby `config.yaml` file, preserving names, hosts, users, and groups.

- **Dry-run and Safe Execution:**  
  Preview what will be imported before making changes. Automatically creates a backup of your Tabby config before writing.

- **Protocol Filtering:**  
  Filter which protocols to export/import (e.g., only SSH).

- **Cross-platform Defaults:**  
  Automatically detects default Remmina and Tabby config locations for Linux, Windows, and macOS.

---

## Usage

```sh
cargo run -- --remmina-dir <remmina_dir> --tabby-dir <tabby_dir> [--protocol SSH,RDP,VNC] [--remmina-check] [--execute] [--yes]
```

```sh
remmina-to-tabby --remmina-dir <remmina_dir> --tabby-dir <tabby_dir> [--protocol SSH,RDP,VNC] [--remmina-check] [--execute] [--yes]
```

- `--remmina-dir`     : Path to your Remmina profiles directory.
- `--tabby-dir`       : Path to your Tabby config directory.
- `--protocol`        : Comma-separated list of protocols to migrate (default: SSH).
- `--remmina-check`   : Check and show protocols found in Remmina files.
- `--execute`         : Actually perform the import (otherwise, dry-run).
- `--yes`             : Proceed without confirmation

---

## Limitations & Enhancement Notes

- **Tabby SSH Only:**  
  Currently, Tabby only supports SSH profiles. RDP and VNC profiles are detected but **not imported**.  
  _Enhancement: Tabby support for RDP/VNC would allow full migration._

- **No Subgroup Support:**  
  Remmina supports nested groups (subgroups), but Tabby does not.  
  _Enhancement: Tabby subgroup support would improve group mapping._

- **Profile Options:**  
  Some advanced Remmina options may not be mapped if not supported by Tabby.

---

## Roadmap / Ideas

- [ ] Full support for RDP and VNC when Tabby adds these protocols.
- [ ] Better mapping of Remmina subgroups to Tabby groups.
- [ ] More robust handling of custom/advanced Remmina profile fields.

---

## Contributing

PRs and issues are welcome!  
If you find a bug or want to help with enhancements, open an issue or submit a pull request.

---

## License

MIT

---