# Quick Question (qq)

This is a simple rust binary to query the ChatGPT API from the CLI.

## Installation

```bash
cargo build -- release && cp target/release/qq /usr/local/bin
```

You'll need to set your OpenAI API key in the `OPENAI_API_KEY` environment variable.

```bash
export OPENAI_API_KEY=sk-...
```

## Usage

```bash
qq "What is the meaning of life?"
```
