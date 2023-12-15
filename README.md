# Ecdar-GUI-Native

A native implementation of Ecdar-GUI-Web


### Install dependencies

Install `node`, `protobuf`, and `cargo`

#### Linux

###### Debian
```
sudo apt install cargo nodejs protobuf-compiler
```

###### Arch
```
sudo pacman -S cargo nodejs protobuf
```

#### Mac
```
brew install protobuf node
```

Install [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

#### Windows
Install [Protoc](https://www.geeksforgeeks.org/how-to-install-protocol-buffers-on-windows/)

Install [Node](https://nodejs.org/en/download)

Install [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)


### Cargo dependencies
To run tauri applications, some need to install tauri cli
```
cargo install tauri-cli
```

### Git dependencies
To install and initialize all submodules, run
```
git submodule update --init --recursive
```

### Ecdar-GUI-Web dependencies
Open the `Ecdar-GUI-Web` submodule, and follow its installation instructions to ensure that it can compile itself.

### Run
How to run Ecdar-GUI-Native
#### Dev
```
cargo tauri dev
```
#### Release
```
cargo tauri build
```
In release the executable can be found at
###### Mac/Linux
```./target/release/ecdar```
###### Windows
```.\target\release\ecdar.exe```
