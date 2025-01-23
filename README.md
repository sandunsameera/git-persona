# git-persona

git-persona is a CLI tool for managing multiple Git profiles and credentials. It simplifies switching between different identities (e.g., work, personal) for Git repositories. With git-persona, you can easily add, remove, and use profiles without manually editing your `.gitconfig` file.

---

## Features

- **Add Credentials**: Save multiple Git profiles with username and email.
- **Switch Profiles**: Quickly switch between profiles globally or per repository.
- **List Profiles**: View all stored profiles.
- **Remove Profiles**: Delete unused profiles.

---

## Installation

### Prerequisites

- Rust and Cargo installed ([Install Rust](https://www.rust-lang.org/tools/install)).

### Build from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo/git-persona.git
   cd git-persona
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Install the binary:
   ```bash
   cp target/release/git-persona /usr/local/bin/git-persona
   ```
4. Verify the installation:
   ```bash
   git-persona --help
   ```

---

## Usage

### Add a Profile

To add a new Git profile:
```bash
git-persona add --name <profile_name> --username <username> --email <email>
```
Example:
```bash
git-persona add --name work --username "WorkUser" --email "work@example.com"
```

### List Profiles

To list all saved profiles:
```bash
git-persona list
```

### Use a Profile

To switch to a saved profile globally:
```bash
git-persona use --name <profile_name>
```
Example:
```bash
git-persona use --name work
```

### Remove a Profile

To delete a saved profile:
```bash
git-persona remove --name <profile_name>
```
Example:
```bash
git-persona remove --name work
```

---

## How It Works

git-persona stores your profiles in a secure configuration file located in your system's configuration directory (e.g., `~/.config/git-persona/credentials.json`).

When you use a profile, git-persona updates your global Git configuration using:
- `git config --global user.name`
- `git config --global user.email`

For more advanced setups, you can use git-persona to manage repository-specific credentials by navigating to the repository and running:
```bash
git config user.name <username>
git config user.email <email>
```

---

## Roadmap

- Add support for managing SSH keys.
- Enhance security with encrypted storage.
- Add cross-platform installers (Homebrew, Scoop).
- Support repository-specific credential management directly.

---

## Contributing

Contributions are welcome! To contribute:
1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Submit a pull request with a clear description of your changes.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Support

If you encounter any issues or have questions, please open an issue on [GitHub](https://github.com/your-repo/git-persona/issues).
