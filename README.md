# DotDot

Backup My Dot files.

Resolve rule file like:

git.yml
```yaml
root: .gitconfig
```

to `~/.gitconfig`

```yaml
root: .gradle
include:
  - gradle.properties
  - init.d/
```

to `~/.gradle/gradle.properties` and `~/.gradle/init.d/*`

Very fast, but 应该有很多 bug.
