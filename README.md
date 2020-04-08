# DotDot

Backups My Dot files. (New Edition write with cpp)

## Rules

Resolve rule file like:

`git.yml`

```yaml
root: .gitconfig
```

to `~/.gitconfig`

`gradle.yml`

```yaml
root: .gradle
include:
  - gradle.properties
  - init.d/
```
like `init.d/` is a directory, or else it will be a file.

## Usage

```
Usage:
        dotdot (backup|restore|resolve|help) [-d <Directory>] [-r <Directory>...]

Options:
        -d, --directory <Directory>
                    Directory of dotfiles

        -r, --rules <Directory>
                    Sets addition rules directory

```

## Build

**require cmake and a c++ compiler support c++20 ?**

- clone this repo

- use cmake build

```shell script
mkdir build && cmake -B build
cmake --build build
```
