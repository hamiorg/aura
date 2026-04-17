# Installation

## macOS — Homebrew

```sh
brew install hamiorg/aura/aura
```

The Homebrew formula is updated automatically on each release. Run
`brew upgrade aura` to get a newer version.

## Linux — one-line installer

```sh
curl -fsSL https://hami.aduki.org/install.sh | bash
```

Detects your architecture (`x86_64` or `arm64`), downloads the appropriate
static binary from the latest GitHub release, and installs it to
`/usr/local/bin/aura`. The binary has no system library dependencies.

To install a specific version:

```sh
AURA_VERSION=v0.3.2-beta.2 curl -fsSL https://hami.aduki.org/install.sh | bash
```

## Linux — Package Managers (APT / YUM / DNF)

The recommended way to install and stay updated on Linux is via our official
Cloudsmith repository.

### Debian, Ubuntu (APT)

```sh
curl -1sLf 'https://dl.cloudsmith.io/public/aduki-inc/aura/setup.deb.sh' | sudo bash
sudo apt install aura
```

### Fedora, RHEL, AlmaLinux (DNF/YUM)

```sh
curl -1sLf 'https://dl.cloudsmith.io/public/aduki-inc/aura/setup.rpm.sh' | sudo bash
sudo dnf install aura
```

### Manual installation

If you prefer to install the packages manually without adding a repository:

**Debian, Ubuntu:**

```sh
sudo dpkg -i aura-<version>-linux-x86_64.deb
```

**Fedora, RHEL, AlmaLinux:**

```sh
sudo rpm -i aura-<version>-linux-x86_64.rpm
```

## Build from source

Requires Rust 1.75 or later.

```sh
git clone https://github.com/hamiorg/aura.git
cd aura
cargo build --release -p compiler
# binary at: target/release/aura
```

## Supported platforms

| Platform | Architecture | Formats              |
| -------- | ------------ | -------------------- |
| Linux    | x86\_64      | `.tar.gz` `.deb` `.rpm` |
| Linux    | arm64        | `.tar.gz` `.deb` `.rpm` |
| macOS    | arm64        | `.tar.gz` (Homebrew) |
| macOS    | x86\_64      | `.tar.gz`            |

Linux binaries are statically linked against musl libc. They run on any
Linux distribution without installing additional libraries.
