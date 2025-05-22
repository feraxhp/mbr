# mbr (Monitor brightness)

Is a siple command line tool to change the brightness of
a DDC/CI monitor.

This tool is a cli wrap for [ddc-hi](https://crates.io/crates/ddc-hi)

---
## Installation

- Windows: Download the latest .exe from the releases page. and add it to your PATH.
- Ubuntu: See releases page for the latest deb package.
```bash
wget -O paquete.deb <URL_DEL_PAQUETE> 
dpkg sudo dpkg -i paquete.deb 
sudo apt-get install -f
```
- fedora: See releases page for the latest rpm package.
```bash
sudo dnf install <URL_DEL_PAQUETE>
```
- Arch Linux: See build instructions below.
- Other Linux distributions: See build instructions below.
- MacOS: See build instructions below.

---
## Build and Run

### Build
```bash

# clone the repository
git clone https://github.com/feraxhp/mbr.git
cd mbr

# if you want to install it on your system
cargo install --path .
```
---
## Need more Functionality?

If you need more functionality, feel free to open an issue or a pull request.