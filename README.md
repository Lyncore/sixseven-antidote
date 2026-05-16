# Sixseven Antidote

> Created for everyone who is tired of seeing 67 everywhere.

Sixseven Antidote is a desktop utility that automatically finds and replaces all occurrences of `67` with `69` in supported text documents.


## Built With

- ⚡ Tauri
- 🦀 Rust
- 📦 TypeScript
- 💚 Vue.js


## Features

- 🔍 Search for files inside a selected folder or across the entire PC
- 📄 Display a list of all found files
- ✅ Apply changes to selected files or all found files at once
- 👀 Open files for preview before editing
- 💾 Optional backup creation before modification
- 🧩 Support for `.txt` and `.docx` files
- ⚙️ Extensible architecture for adding support for more formats


## Supported File Types

| Format | Supported |
|--------|------------|
| `.txt` | ✅ |
| `.docx` | ✅ |
| `.pdf` | ❌ |

Additional formats can be added through custom handlers.


## How It Works

1. Select a folder or scan the entire computer
2. The application searches for supported files
3. Choose which files should be processed
4. Replace all `67` values with `69`
5. Optionally create backup copies before applying changes

## Example

Before:

```txt
The quick brown fox jumps over 67 lazy dogs.
```

After:

```txt
The quick brown fox jumps over 69 lazy dogs.
```

## Development

### Requirements

- Bun
- Rust
- Tauri CLI (installs automatically via `bun install`)

### Install dependencies

```bash
bun install
```

### Run in development mode

```bash
bun run tauri dev
```

### Build the application

```bash
bun run tauri build
```

## Disclaimer

This project is made for fun and entertainment purposes.

Always create backups before modifying important files.

## Why?

Because enough is enough.

## Roadmap

- [ ] PDF support (and maybe other formats)
- [ ] Custom replacement values
- [ ] IDK

## License

MIT License