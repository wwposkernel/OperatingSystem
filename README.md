# Operating-System
Operating System Notes

本项目是使用Rust编写的微内核x64位操作系统，支持多进程调度，鼠标操作，键盘输入等（其余功能还在开发中）

# 博客地址

[使用Rust开发操作系统](https://blog.csdn.net/qq_41698827)


# 编写环境
## 系统
Linux version 5.0.0-27-generic (buildd@lgw01-amd64-031) (gcc version 7.4.0 (Ubuntu 7.4.0-1ubuntu1~18.04.1)) #28~18.04.1-Ubuntu

## 虚拟机
### qemu-system-x86

```
sudo apt install qemu-system-x86
```

## RUST 编译器
rustc 1.41.0-nightly (ded5ee001 2019-11-13)

### 安装

#### rust:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### nightly安装（需要安装rustup 一般安装完rust后自带的）
```
rustup install nightly
```

#### 查看
```
$>> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

#### 切换编译器
```
// 切换到cargo项目目录
$ rustup override set nightly
```

### 安装xbuild

```
cargo install cargo-xbuild
```



# 参考书籍

《一个64位操作系统的设计与实现》

《30天自制操作系统》

《Orange\`s一个操作系统的实现》

《Linux内核完全注释》

《Professional Assembly Language》

《汇编语言（第三版）》

《深入理解Linux内核》

《深入理解BootLoader》

《深入理解计算机系统第三版》

《操作系统概念》

《64-ia-32-architectures-software-developer-vol-3a》

# 网站

http://www.maverick-os.dk/

http://mikeos.sourceforge.net/write-your-own-os.html

http://www.bioscentral.com

# License
Source code  is under the Apache License.
The `src/lib` code is reference [rust-osdev](https://github.com/rust-osdev/x86_64)

# Todo

- [x] println!和print！宏
- [x] 原子锁
- [x] 异常处理
- [x] 双重异常处理
- [x] 初级中断处理（8259A）
- [x] 键盘可输入ASCII可打印字符
- [ ] 初级进程
- [ ] 添加系统调用
- [ ] 进程间通信(IPC)
- [ ] 处理器固件信息处理
- [ ] 鼠标驱动
- [ ] 分页内存以及初级内存管理
- [ ] SLAB内存池以及通用内存管理
- [ ] UEFI启动
- [ ] 文件系统
- [ ] Shell解析器
- [ ] 系统API库