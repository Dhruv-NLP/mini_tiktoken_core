# Mini Tiktoken Project

A lightweight Python project for working with tokenization in the style of OpenAI's `tiktoken` library. This project is designed to help you understand how text is converted into tokens, how token counts are computed, and how byte-pair encoding (BPE) can be used for large language model workflows.

Whether you are building prompt budget tools, experimenting with model limits, or learning how tokenization works under the hood, this project provides a compact and easy-to-follow implementation.

## Features

- Minimal tokenization library inspired by OpenAI's `tiktoken`
- Encode text into token IDs and decode back to strings
- Count tokens for prompts and chat messages
- Lightweight and beginner-friendly implementation
- Easy to extend with new encodings or custom logic
- Suitable for learning, prototyping, and experimentation

## Why This Project?

Large language models often require you to track token usage carefully. Token counting can affect:

- Prompt length limits
- API cost estimation
- Model context window management
- Output budgeting

This project provides a stripped-down version of these ideas so you can inspect the logic without needing a large external dependency stack.

## Project Goals

- Make tokenization simple and approachable
- Provide a small reference implementation
- Help users understand GPT-style encoding and token limits
- Support educational and prototyping use cases

## Installation

Clone the repository:

```bash
git clone https://github.com/your-username/mini_tiktoken_project.git
cd mini_tiktoken_project
```

Create and activate a virtual environment:

```bash
python -m venv .venv
```

On macOS/Linux:

```bash
source .venv/bin/activate
```

On Windows:

```powershell
.venv\Scripts\activate
```

Install dependencies:

```bash
pip install -r requirements.txt
```

If the project is using `tiktoken` directly, you can also install it manually:

```bash
pip install tiktoken
```

## Usage

A typical usage pattern looks like this:

```python
from mini_tiktoken import get_encoding

text = "Hello, world! This is a mini tiktoken project."
encoding = get_encoding("cl100k_base")

tokens = encoding.encode(text)
print(tokens)
print(len(tokens))
print(encoding.decode(tokens))
```

You can also count tokens without converting to text manually:

```python
from mini_tiktoken import count_tokens

prompt = "How many tokens are in this sentence?"
print(count_tokens(prompt, encoding_name="cl100k_base"))
```

## Example Workflow

```python
from mini_tiktoken import get_encoding

system_message = "You are a helpful assistant."
user_message = "Summarize the benefits of token counting in simple terms."

enc = get_encoding("cl100k_base")

messages = [
    {"role": "system", "content": system_message},
    {"role": "user", "content": user_message},
]

prompt = "\n".join(
    f"{m['role']}: {m['content']}" for m in messages
)

token_count = len(enc.encode(prompt))
print(f"Estimated prompt tokens: {token_count}")
```

## Common Encoding Names

Many GPT-style encoding names are commonly used, including:

- `gpt2`
- `r50k_base`
- `p50k_base`
- `cl100k_base`

The appropriate encoding depends on the model or API provider you are targeting.

## Project Structure

```text
mini_tiktoken_project/
├── README.md
├── requirements.txt
├── mini_tiktoken/
│   ├── __init__.py
│   ├── encoder.py
│   ├── tokenizer.py
│   └── utils.py
├── tests/
│   └── test_tokenizer.py
├── setup.py
└── pyproject.toml
```

The structure may vary depending on how the project is organized, but the core idea remains the same: a small tokenizer implementation and test coverage for expected behavior.

## Running Tests

If the repository includes unit tests, run them with:

```bash
pytest
```

Or, if using Python's built-in test runner:

```bash
python -m unittest discover
```

## Notes

This project is intentionally compact. It is useful for:

- learning how tokenizer logic works
- benchmarking simple text processing tasks
- understanding token budget limits before calling APIs
- experimenting with custom text preprocessing

## Contributing

Contributions are welcome. You can help by:

- improving the tokenizer logic
- adding more encoding support
- increasing test coverage
- documenting edge cases and examples

To contribute:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run the relevant tests
5. Submit a pull request

## License

This project is licensed under the MIT License. See the LICENSE file for details if included in the repository.

## Acknowledgements

This project is inspired by the concepts and tooling used in modern LLM systems, especially tokenization patterns developed by OpenAI and related inference tooling.

## Summary

Mini Tiktoken Project is a small, practical example of how tokenization works in generative AI systems. It is ideal for developers, students, and researchers who want a lightweight, understandable introduction to token budgeting and GPT-style encoding.
