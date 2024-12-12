# godir
Goto Directory - A fuzzy directory navigation tool

Godir is a fuzzy directory navigation tool that allows you to quickly navigate to directories based on a pattern.

Very early in my career, I built a command line utility in C that required me to navigate to directories based on a pattern. I've been missing that little tool ever since. So I rebuilt the tool in Rust.

## Installation

To ensure godir can change the current shell directory, add a shell function. Modify your shell’s configuration file (~/.bashrc, ~/.zshrc, etc.):

```sh
godir() {
    local dir
    dir="$(command ~/dev/godir/target/release/godir "$@")"
    if [ -n "$dir" ]; then
        cd "$dir"
    fi
}
```

```sh
source ~/.bashrc
```


## Usage

```sh
godir <pattern>
```


---

## Usage tips

The godir command maintains a configuration file in the user's home directory. The configuration file is named `.directories.json` and is located in the `.godir` directory under the user's home directory. The configuration file is used to store the directories that godir has scanned.


```json
{
  "directories": [
    "/Users/cjus/dev/commercial",
    "/Users/cjus/dev/commercial/apps/demo-redis",
    "/Users/cjus/dev/godir",
    "/Users/cjus/dev/moose",
    "/Users/cjus/dev/moose_redis",
    "/Users/cjus/dev/redis-user-create"
  ],
  "excludes": [
    ".git",
    ".next",
    "/Applications",
    "/cores",
    "/Library",
    "/private",
    "/System",
    "/usr",
    "/var",
    "/Volumes",
    "dist",
    "iCloud",
    "node_modules",
    "target"
  ]
}
```

Edit the configuration file to add or remove directories.

To exclude directories, add them to the `excludes` array.





