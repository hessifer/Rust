## Installation
1. On your Mac OS or Linux system execute the following:
    ``$ curl https://sh.rustup.rs -sSf | sh
``
    * follow the prompts to complete the installation
1. The installation process will add the Rust binaries to your system PATH at next login. To use Rust now execute the
   following command.
   ```source $HOME/.cargo/env```
1. If you want to update Rust to the latest version execute the following command: ```rustup update```
1. To uninstall Rust execute the following command: ```rustup self uninstall```
1. To get the current version of your Rust compiler execute the following command: ```rustc --version```
1. To get the current version of Rust's build/package manager called Cargo issue the following
   command: ```cargo --version```
1. To create your first Rust program called *getting_started* you can use your favorite IDE or issue the following
   cargo command from your code repository directory: ```cargo new getting_started --bin```
   * This will create a new 'binary' project as opposed to a 'library' in your current directory. Inside the
     getting_started directory you will find a *src* directory and two files *Cargo.toml* and *Cargo.lock*. In
     addition you should notice a git repo initialized and a .gitignore file. (*see documentation for how to change
     the VCS*)
   * *Cargo.toml* has an extension .toml (Tom's Obvious, Minimal Language). This file will be used to manage your
     project's configuration.
   * *Cargo.lock* keeps track of the versions for each dependency your project has. You do not need to edit this
     file.
1. To build your program execute the following command from the root of your project folder
   (**not the src folder**: ```cargo build ```
1. To run your program execute the following command from the same location: ```cargo run``` 
1. To check if your code will compile but not execute the normal build process or run the program issue the following
   command: ```cargo check```
1.  The above build commands will create a debug version of your project.
    * You can find the binary in *target/debug*
1. Once you are ready to compile the code for production/release using the following command will compile your code
   with optimizations: ```cargo build --release```