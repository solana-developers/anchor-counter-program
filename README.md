# Anchor Counter

This project demonstrates a counter program on Solana using the Anchor framework. It includes an example code from the "Intro to Anchor" lesson in the Solana Onchain Development course.

## Prerequisites

- Rust and Cargo
- Solana CLI tools
- Anchor CLI
- Node.js and Yarn

## Setup

1. Clone the repository

```bash
   git clone <repository-url>
   cd <project-directory>
```

2. Install dependencies:

```bash
   yarn install
```

3. Build the Anchor project

```bash
   anchor build
```

4. Sync the program ID:

```bash
  anchor keys sync
```

## Running Tests

```bash
anchor test
```

## Notes

- Ensure your Solana validator is running locally before running tests.
- If you encounter any issues, make sure your Anchor.toml and Cargo.toml files are correctly configured for your project.
