# 项目先导阶段 - 组件化操作系统

基于 [LearningOS 2026s 课程仓库](https://github.com/LearningOS/2026s-oscamp-professional-2026s-arceos-arceos-classroom-2026s-arceos-oscamp)，由 [LearningOS](https://github.com/LearningOS) 统一分配学员仓库、运行真实测试并上传 OpenCamp。

**6 项练习 · 总分：600**

## 领取与克隆

1. 加入 [OpenCamp 秋冬季训练营](https://opencamp.cn/os2edu/camp/2026fall)，并绑定自己的 GitHub 账号。
2. 点击[领取作业仓库](https://github.com/LearningOS/2026a-enroll/issues/new?template=arceos.yml)，点击 **Create** 提交申请；等待机器人回复，接受仓库邀请。
3. 克隆分配的仓库，在 `main` 完成实验。

```sh
git clone https://github.com/LearningOS/2026a-arceos-YOUR_GITHUB_LOGIN.git
cd 2026a-arceos-YOUR_GITHUB_LOGIN
```

将 `YOUR_GITHUB_LOGIN` 替换为自己的 GitHub 登录名，依次下载作业仓库并进入目录。OpenCamp 绑定、领取和推送应使用同一个 GitHub 账号。

## 教材与代码导航

本仓库是 [ArceOS](https://github.com/arceos-org/arceos) 的教学剪裁版本，通过补全组件和练习理解组件化操作系统。阅读 [ArceOS Tutorial Book](https://rcore-os.cn/arceos-tutorial-book/)，配合仓库课件和 [OpenCamp 项目先导阶段](https://opencamp.cn/os2edu/camp/2026fall/stage/6) 的课程资料学习。

| 目录 | 学习内容 |
| --- | --- |
| [arceos/](arceos/) | 教学内核；先了解模块划分，再沿练习调用路径阅读实现 |
| [arceos/exercises/](arceos/exercises/) | 六项训练练习及各自说明 |
| [course/](course/) | 往期课程 PPT，供本期配合学习 |
| [crates/](crates/) | 教学版本调整过的组件，如 `kernel_guard` |
| [scripts/](scripts/) | 单项测试与完整测试脚本 |
| [challenges/](challenges/) | 内存分配器挑战的原理与实验资料 |

`challenges/` 保留了往期挑战题资料。其中的邮件提交、截止日期和独立 `lab1` 工程属于往期活动；本期自动评测的六项练习见下表。可以继续阅读挑战题设计，但当前作业仓库没有 `lab1` 分支或 `verify_lab1.sh`。

## 环境配置

以下本地步骤面向 **x86_64 Linux / Ubuntu**；Windows 可在 WSL2 Ubuntu 中进行。Rust 安装原理可参考[环境配置教程](https://rcore-os.cn/arceos-tutorial-book/ch01-02.html)，编译版本使用本仓库 `arceos/rust-toolchain.toml` 中的 `nightly-2024-09-04`。

先通过 [rustup](https://rustup.rs/) 安装 Rust，再在作业仓库根目录准备系统工具：

```sh
sudo apt-get update
sudo apt-get install -y build-essential libclang-dev qemu-system-misc dosfstools mtools pkg-config libssl-dev zlib1g-dev wget xz-utils
```

依次更新包索引并安装编译、QEMU、磁盘镜像及依赖开发工具。

```sh
rustup toolchain install nightly-2024-09-04 --profile minimal --component rust-src --component llvm-tools-preview --component rustfmt --component clippy --target riscv64gc-unknown-none-elf
```

安装课程使用的 Rust、源码与 LLVM 工具，以及 RISC-V 裸机目标。

部分练习还需要 RISC-V musl C 工具链。首次安装时，使用与课程 CI 相同的预编译来源：

```sh
mkdir -p tmp/musl
wget -O tmp/musl/riscv64-linux-musl-cross.tgz https://github.com/arceos-org/setup-musl/releases/download/prebuilt/riscv64-linux-musl-cross.tgz
sudo mkdir -p /opt/musl
sudo tar -xzf tmp/musl/riscv64-linux-musl-cross.tgz -C /opt/musl
```

依次创建下载目录、下载工具链，并解压到 `/opt/musl`。该预编译工具链用于 x86_64 Linux 主机。

在练习终端中让构建工具可见：

```sh
mkdir -p tmp/tools
course_sysroot=$(rustc +nightly-2024-09-04 --print sysroot)
ln -sf "$course_sysroot/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-objcopy" tmp/tools/rust-objcopy
ln -sf "$course_sysroot/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-objdump" tmp/tools/rust-objdump
export PATH="$PWD/tmp/tools:/opt/musl/riscv64-linux-musl-cross/bin:$PATH"
qemu-system-riscv64 --version
riscv64-linux-musl-gcc --version
```

先在仓库的 `tmp/tools` 中建立课程版本的 LLVM 工具入口，再让当前终端找到它们和 musl 编译器，最后检查 QEMU 与 C 编译器。新开终端后，在仓库根目录重新执行其中的 `export PATH=...` 即可。

本期六项练习使用 RISC-V。学习其他架构时可继续参考 [ArceOS 上游构建说明](https://github.com/arceos-org/arceos) 配置 x86_64 或 AArch64 的工具链。

## 练习与本地测试

按顺序阅读练习目录中的说明、补全代码，再运行对应测试。下面的命令均在作业仓库根目录执行：

| 练习 | 本地测试命令 |
| --- | --- |
| `print_with_color` | `./scripts/test-print.sh` |
| `ramfs_rename` | `./scripts/test-ramfs_rename.sh` |
| `alt_alloc` | `./scripts/test-alt_alloc.sh` |
| `support_hashmap` | `./scripts/test-support_hashmap.sh` |
| `sys_map` | `./scripts/test-sys_map.sh` |
| `simple_hv` | `./scripts/test-simple_hv.sh` |

每条命令运行一项练习的构建与测试。测试会生成并重建用于练习的磁盘镜像；以脚本输出的测试结果检查实现。

```sh
mkdir -p tmp
./scripts/total-test.sh > tmp/local-test.log
cat tmp/local-test.log
cat test.output
```

依次准备日志目录、运行全部六项测试、查看完整日志和各项结果。总测试脚本会在日志末尾打印总分；它本身成功退出并不代表六项全部通过，请同时查看 `test.output` 中的通过情况。

## 提交实验

```sh
git diff
git add 你修改的实验文件
git commit -m "Complete an ArceOS exercise"
git push origin main
```

依次检查改动、暂存指定实验文件、创建提交并推送到 `main`。将占位文字替换为实际修改的文件路径；不要提交测试生成的磁盘镜像和日志。也可以通过已登录的 GitHub Desktop 或编辑器完成相同操作。

## 计分规则


沿用往期各项练习的权重，每项全部通过才获得该项分数。每次提交重新计算全部练习，上传本次实际总分；不是按提交次数累加。部分完成也会上传测得的分数。编译或测试未通过、超时的练习记 0 分；环境准备失败、缺少结果或结果不完整时不上传。

| 练习 | 分值 |
| --- | ---: |
| `print_with_color` | 100 |
| `ramfs_rename` | 100 |
| `alt_alloc` | 100 |
| `support_hashmap` | 100 |
| `sys_map` | 100 |
| `simple_hv` | 100 |

Actions 中测试作业变红表示还有未完成练习；单独的 **Save measured score and upload to OpenCamp** 作业显示成绩同步是否成功。日志出现 `OpenCamp accepted the score (result=1).` 才表示接口接受成绩。

成绩明细同时保存在运行附件和学员仓库的 `gh-pages` 分支的成绩文件，无需启用 GitHub Pages。

## 查看成绩与处理问题

在自己仓库的 **Actions** 查看本次运行的各题日志、实际分数和上传结果，再到 OpenCamp 核对自己的成绩。只有分配的学员账号触发的运行会上传该账号成绩；维护者代推只测试。

若上传失败，先确认已加入秋冬季训练营并绑定领取仓库的 GitHub 账号，再把运行链接和错误信息交给助教。修复后可以重跑工作流。环境安装失败或运行取消时，不会上传不完整结果。
