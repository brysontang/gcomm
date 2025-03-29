# gcomm

**AI-powered Git commit message generator using your local Ollama model.**

This CLI tool analyzes staged Git changes and generates a structured commit message using a locally running LLM (e.g. `gemma3`, `llama3`, etc.).

---

## 🚀 Getting Started

### Install via cargo

```bash
cargo install gcomm
```

### Alternative: Build from source

#### 1. Install Rust (if you haven't yet)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### 2. Clone and build the project

```bash
git clone https://github.com/bryson-tang/gcomm
cd gcomm
cargo build
```

#### 3. (Optional) Install globally

```bash
cargo install --path .
```

Or add a shortcut:

```bash
alias gcomm="./target/debug/gcomm"
```

Now you can use gcomm from anywhere.

## 🧠 Usage

This uses your staged changes only (like git diff --cached), so make sure you've staged what you want:

```bash
git add .
```

Generate a commit message (copies to clipboard):

```bash
gcomm
```

Specify a different Ollama model (default is gemma3:latest):

```bash
gcomm --model codellama
```

Run git add . first:

```bash
gcomm --add
```

Generate, edit in your default editor, and commit:

```bash
gcomm --edit
```

Generate and commit immediately without editing:

```bash
gcomm --yolo
```

## 🧩 Requirements

- [Ollama](https://ollama.com) running locally
- A model pulled (e.g. ollama run gemma3 or ollama run llama3)

## ✨ Output Format

```
Add analytics and fix layout issues

Added

- Event tracking for login

Changed

- Layout of settings page
- Refactored token logic

Removed

- Unused `tracking.js`
```

## 📄 License

MIT

## 💖 Acknowledgements

Built with a lot of love and help from ChatGPT.
