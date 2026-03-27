# My-First-Stellar-Project


## Project Structure(Soroban Project)

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```
# deploy contract credentials
contract ID:CDXKJD4JPMJ5LVW4YCRLJ3ZWMLJ6SSWFVZTA5RMD4CHWSLIQHUJKIAS6
<img width="1786" height="854" alt="image" src="https://github.com/user-attachments/assets/f46b67e0-b644-4fbd-93ca-457ff9d2e4bc" />


- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.
