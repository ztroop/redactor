# Redactor

The String Redactor is a CLI tool written in Rust. It takes a string input and redacts the characters based on the options provided. You can configure the length to redact, the redaction character, characters to ignore, and specify if you want to ignore redaction on the first or last `X` number of characters.

## Installation

### Prerequisites

- Rust and Cargo installed (Get it from [here](https://www.rust-lang.org/tools/install))

### Steps

1. Clone the repository:

   ```bash
   git clone https://github.com/ztroop/redactor.git
   ```

2. Navigate to the project directory:

   ```bash
   cd redactor
   ```

3. Install with cargo:

   ```bash
   cargo install --path .
   ```

## Usage

### Basic Syntax

```bash
redactor [FLAGS] [OPTIONS] <input_string>
```

### Options

- `-l, --length <length>`: Specify the length to redact. If not provided, the full length of the string will be used.
- `-r, --redact-char <char>`: Character to use for redaction. The default is '\*'.
- `-i, --ignore-char <char>`: Character to ignore during redaction.
- `-e, --ignore-last <number>`: Ignore the last X characters from redaction.
- `-f, --ignore-first <number>`: Ignore the first X characters from redaction.

### Examples

- To redact the string "hello" completely:

  ```bash
  redactor hello
  ```

- To redact the string "hello" but only the first 3 characters:

  ```bash
  redactor hello -l 3
  ```

- To redact the string "hello" but use `#` as the redaction character:

  ```bash
  redactor hello -r "#"
  ```

- To redact the string "hello" but ignore `l`:

  ```bash
  redactor hello -i "l"
  ```

- To redact the string "hello" but ignore the last 2 characters:

  ```bash
  redactor hello -e 2
  ```

- To redact the string "hello" but ignore the first 2 characters:

  ```bash
  redactor hello -f 2
  ```

## License

This project is licensed under the MIT License.
