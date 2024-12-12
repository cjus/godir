# godir
Goto Directory - Fuzzy CD command

## Installation

To ensure godir can change the current shell directory, add a shell function. Modify your shell’s configuration file (~/.bashrc, ~/.zshrc, etc.):
```
godir() {
    cd "$(command godir "$@")"
}
```

```sh
source ~/.bashrc
```


## Usage

```bash
godir <pattern>
```


