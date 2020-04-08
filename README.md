DotDot

Backups My Dot files. (New Edition write with cpp)

Resolve rule file like:

git.yml

```yaml
root: .gitconfig
```

to ~/.gitconfig

gradle.yml

```yaml
root: .gradle
include:
  - gradle.properties
  - init.d/
```
like `init.d/` is a directory, or else it will be a file.

