# Ecdar-GUI-Native

A native implementation of Ecdar-GUI-Web

## Compile
Be shure to be in the root folder of the project and follow these steps

### Install Dependencies


```
$ git clone https://github.com/ECDAR-AAU-SW-P5/Ecdar-GUI-Web.git
```

#### Linux

Install `node`, `npm` and `cargo`
###### Arch
```
$ sudo pacman -S cargo nodejs npm
```

###### Debian
```
$ sudo apt install cargo node npm 
```

#### Mac

#### Windows

### Npm Dependencies
```
$ npm i -g vite yarn

``` 

### Yarn Dependencies
```
$ cd Ecdar-GUI-WEB
$ yarn install
$ cd ..

```

### Install cargo
```
https://doc.rust-lang.org/cargo/getting-started/installation.html
```
### Cargo dependencies
```
$ cargo install tauri-cli
```
### Run
#### Dev
```
$ cargo tauri dev
```
#### Release
```
$ cargo tauri build
```

###### Mac/Linux
The executable can be found at `./src-tauri/target/release/ecdar`
###### Windows
The executable can be found at `.\src-tauri\target\release\ecdar.exe`
