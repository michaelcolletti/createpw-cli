*## CREATEPW-CLI 
This rust program will use the SHA256 or SHA3-384 algos to create a password and hash and allow for parameterized options for amount of characters providing the hash and password for use.


## Reason for Writing This Utility

- Many Password managers have a way to create a random password based on # of characters and complexity rules and characters used. I love this but it is usually GUI bound and I live the simplicity of a cli utility. 

- I'm learning Rust and I really love code editors and coding assistants. Here I used both to create this utility over a cup of coffee.


## How to Get Rockin 

- Create a folder 
```
mkdir utilities;cd utilities/
```
- Clone this repo

```
git clone https://github.com/michaelcolletti/createpw-cli.git
```

- Enter the /createpw-cli directory
```
cd createpw-cli
```
- Build the utility with Cargo
```
cargo build --release;cargo test;cargo check
```

## Command Usage

```
createpw-cli <number of password characters> <SHA2-256 or SHA3-384> 
```