[![Rust](https://github.com/michaelcolletti/createpw-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/michaelcolletti/createpw-cli/actions/workflows/rust.yml)
## The CREATEPW-CLI Rust Utility

- This small Rust utility will use the SHA2-256 or SHA3-384 algorithms to create a password and hash and allow for parameterized options for amount of characters used and providing the hash and password for copying and pasting into anywhere.

## Reason for Writing This Utility

- Many Password managers have a way to create a random password based on # of characters and complexity rules and characters used. I love this but it is usually GUI bound and I love the simplicity of a cli utility. I wanted to create a simple utility that would allow me to create a password and hash it using a simple command line interface.

- **Note and Caveat: I noticed that metacharacters are consicuously absent from the output so I need to look into this**
 
- I'm learning Rust and I really love code editors and coding assistants. Here I used VSCode and CoPilot to create this utility over a cup of coffee. I learned a bit about creating code but have quite a bit more learning to do regarding the algorithms and the Rust language. I am excited to learn more and create more utilities. 

## How to Get Rockin'

- Create a folder.  I like to create a utilities folder in my home directory. 
```
mkdir utilities;cd utilities/
```
- Clone this repo.  This will create a directory called createpw-cli

```
git clone https://github.com/michaelcolletti/createpw-cli.git
```
- Enter the /createpw-cli directory.
```
cd createpw-cli
```
- Build the utility with Cargo. This will build the utility in release mode, run the tests and check the code.
```
cargo build --release;cargo test;cargo check
```

## Command Usage

```
target/release/createpw-cli <number of password characters> <SHA2-256 or SHA3-384> 
```
## Using Cargo 

- You can use Cargo to run the program as well 

- Example of creating a 12 character password using the SHA2-256 algorithm
```  
cargo run 12 SHA256
target/createpw-cli 12 SHA256 
```
- Example of creating a 24 character password using the SHA3-384 algorithm
```
cargo run 24 SHA3-384
```
