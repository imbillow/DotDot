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

非常快的速度，但是应该有很多 bug.

使用

需要 rust 和 cargo 环境

```shell script
git clone https://github.com/iovw/DotDot.git
cd DotDot
cargo build
```

备份且自动链接(但是会删除原位的文件，建议先备份(但是这是一个备份软件？ D:))

```shell script
cargo run --backup -vvv
```

还原

```shell script
cargo run --restore -vvv
```

如果不再这个目录里运行会找不到配置文件，虽然有点糟糕，但我不打算修了， rust 语言用得不是很开心。 
