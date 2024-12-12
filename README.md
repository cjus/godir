# godir
Goto Directory - A fuzzy directory navigation tool

Godir is a fuzzy directory navigation tool that allows you to quickly navigate to directories based on a pattern.

Very early in my career, I built a command line utility in C called `gd` - "go directory" that allowed me to navigate to directories based on a patterns. I've been missing that little tool ever since. So I rebuilt the tool in Rust.

## Installation

To ensure `godir` can change the current shell directory, add a shell function. Modify your shell’s configuration file (~/.bashrc, ~/.zshrc, etc.):

```sh
godir() {
    local dir
    dir="$(command ~/dev/godir/target/release/godir "$@")"
    if [ -n "$dir" ]; then
        cd "$dir"
    fi
}
```

Then source the configuration you used above (i.e. ~/.bashrc, ~/.zshrc, etc.)

```sh
source ~/.bashrc
```


## Usage

```sh
godir <pattern>
```


---

## Usage tips

The godir command maintains a configuration file in the user's home directory. The configuration file is named `directories.json` and is located in the `.godir` directory under the user's home directory. The configuration file is used to store the directories that `godir` has scanned.


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

Manually edit the configuration file to add or remove directories.

To exclude directories, add them to the `excludes` array.

Notes:
* If you use a pattern that `godir` doesn't reconize then it will give you the option to add a directory path on the spot or ask if you'd like it to perform a full directory scan.  It will then add the directory path(s) it matches to the configuration file.
* After a full directory scan, you should edit the `~/.godir\directories.json` file to cleanup any entries that you don't care about.  You can also add patterns to the `excludes` array to exclude directories from future scans.

To quickly add the current directory to the configuration file, use the `.` pattern.

```sh
godir .
```

### Pattern Expressions

godir supports the following pattern matching expressions:

#### Basic Patterns
- `foo` - Matches any directory containing "foo"
- `^foo` - Matches directories that start with "foo"
- `foo$` - Matches directories that end with "foo"
- `foo|bar` - Matches directories containing either "foo" or "bar"

#### Directory Path Patterns
- `dev/foo` - Matches directories containing "dev/foo"
- `/Users/name` - Matches exact path segments
- `^/Users/name` - Matches paths starting from root

#### Special Characters
- `.` - Matches any single character
- `.*` - Matches zero or more of any character
- `\w` - Matches word characters (letters, digits, underscore)
- `\d` - Matches digits
- `\s` - Matches whitespace

#### Examples

```sh
godir . # Matches the current directory and adds it to the configuration file
godir dev # Matches any directory containing "dev"
godir ^/Users # Matches directories starting with "/Users"
godir project$ # Matches directories ending with "project"
godir dev/./src # Matches paths containing "dev/" followed by any characters, then "/src"
godir 'test|prod' # Matches directories containing either "test" or "prod"
godir 'lionheart.*crons' # Matches directories containing "lionheart" followed by any characters, then "crons": ~/dev/lionheart-backend/Lionheart-Boreal/app/crons
```
