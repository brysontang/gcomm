# gcomm

**AI-powered Git commit message generator using your local Ollama model.**

This CLI tool analyzes staged Git changes and generates a structured commit message using a locally running LLM (e.g. `gemma3`, `llama3`, etc.).

---

## 🚀 Getting Started

### 1. Install Rust (if you haven't yet)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone and build the project

```bash
git clone https://github.com/bryson-tang/gcomm
cd gcomm
cargo build
```

### 3. (Optional) Install globally

```bash
cargo install --path .
```

Now you can use gcomm from anywhere.

## 🧠 Usage

Generate a commit message (copies to clipboard):

```bash
gcomm
```

Generate, edit in your default editor, and commit:

```bash
gcomm --edit
```

Generate and commit immediately without editing:

```bash
gcomm --commit
```

Specify a different Ollama model (default is gemma3:latest):

```bash
gcomm --model codellama
```

This uses your staged changes only (like git diff --cached), so make sure you've staged what you want:

```bash
git add .
```

## 🧩 Requirements

- Ollama running locally
- A model pulled (e.g. ollama run gemma3 or ollama run llama3)

## ✨ Output Format

Add analytics and fix layout issues

Added

- Event tracking for login

Changed

- Layout of settings page
- Refactored token logic

Removed

- Unused `tracking.js`
  🛠 Dev Tip

While developing, you can run it without installing globally:

```bash
cargo run
```

Or add a shortcut:

```bash
alias gcomm="./target/debug/gcomm"
```

You can also use the Makefile to build and install the project.

```bash
make dev
```

## 📄 License

MIT

## 💖 Acknowledgements

Built with a lot of love and help from ChatGPT.
