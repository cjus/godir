# godir
Goto Directory - A fuzzy directory navigation tool

Godir is CLI tool that allows you to quickly navigate to a directory based on a pattern.

Very early in my career, I built a command line utility in C/Assembly called `gd` - "go directory" that allowed me to navigate to directories based on patterns. I've been missing that little tool ever since. So, I rebuilt it in Rust.

## Installation

To ensure `godir` can change the current shell directory, add a shell function to your shell's configuration file (~/.bashrc, ~/.zshrc, etc.):

```sh
godir() {
    local output
    output="$(command ~/dev/godir/target/release/godir "$@")"
    if [ $? -eq 0 ]; then
        if [[ "$1" == "--help" ]] || [[ "$1" == "-h" ]] || [[ "$1" == "--version" ]] || [[ "$1" == "-V" ]] || [[ "$1" == "--list" ]] || [[ "$1" == "-l" ]]; then
            # Handle help, version, and list flags
            echo "$output"
        elif [ -n "$output" ]; then
            # Handle directory change
            cd "$output"
        fi
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

Pro Tip: to quickly add the current directory to the configuration file, use the `.` pattern.

```sh
godir .
```

---

## Usage tips

The `godir` command maintains an editable configuration file. You can find the `directories.json` file in the `.godir` directory under the user's home directory. The configuration file is used to store the directories that `godir` knows about.

Here's an example of what the `directories.json` file looks like:

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

To exclude directories, add related patterns to the `excludes` array.

Notes:
* If you use a pattern that `godir` doesn't reconize then it will give you the option to add a directory path on the spot or ask if you'd like it to perform a full directory scan.  It will then add the directory path(s) it matches to the configuration file.
* After a full directory scan, you should edit the `~/.godir\directories.json` file to cleanup any entries that you don't care about.  You can also add patterns to the `excludes` array to exclude directories from future scans.

Remember, to quickly add the current directory to the configuration file, use the `.` pattern.

```sh
godir .
```

### Path Handling

Godir supports direct navigation using relative or absolute paths:

```sh
godir ../projects     # Navigate to relative path
godir /Users/name/dev # Navigate to absolute path
godir ~/dev/project   # Navigate using shell expansion
```

When using a path (instead of a pattern), godir will:
1. Expand the path to its full canonical form
2. Verify it's a valid directory
3. Add it to the configuration file if not already present
4. Navigate to the directory

This makes it easy to add new directories to your configuration while navigating to them.

### Pattern Expressions

Godir supports Regex pattern matching expressions:

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

