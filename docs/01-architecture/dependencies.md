# 外部依赖与可复用组件 / Dependencies & Reusable Components

## 原则

AOS 不重复造轮子。Rust OS 开发生态已经提供了大量成熟的底层组件，AOS 应当直接使用这些组件，将精力集中在 Agent 原语、IPC、Capability 等 AOS 独有的设计上。

## 可直接使用的 Rust Crate

### 引导与硬件抽象（hal/ 使用）

| crate | 用途 | 对应模块 |
|-------|------|---------|
| [bootloader](https://github.com/rust-osdev/bootloader) | x86_64 引导加载器（BIOS + UEFI） | 省掉自写 bootloader |
| [uefi-rs](https://github.com/rust-osdev/uefi-rs) | UEFI 固件接口封装 | `hal/arch/x86_64/boot.rs` |
| [x86_64](https://crates.io/crates/x86_64) | 页表、GDT、IDT、CPU 指令封装 | `hal/arch/x86_64/` |
| [aarch64-cpu](https://crates.io/crates/aarch64-cpu) | ARM64 CPU 寄存器和指令封装 | `hal/arch/aarch64/` |
| [acpi](https://crates.io/crates/acpi) | ACPI 表解析（硬件发现） | `hal/` 硬件检测 |
| [uart_16550](https://crates.io/crates/uart_16550) | 串口驱动 | `hal/console/` |
| [pic8259](https://crates.io/crates/pic8259) | 8259 中断控制器 | `hal/arch/x86_64/` |

### 内存管理（hal/ 使用）

| crate | 用途 | 对应模块 |
|-------|------|---------|
| [linked-list-allocator](https://github.com/rust-osdev/linked-list-allocator) | no_std 堆分配器 | `hal/mm/` 内核堆初始实现 |

### 同步原语（hal/ 使用）

| crate | 用途 | 对应模块 |
|-------|------|---------|
| [spinning_top](https://crates.io/crates/spinning_top) | 自旋锁 | `hal/sync/` |

### 虚拟化与设备（M5 阶段使用）

| crate | 用途 | 对应模块 |
|-------|------|---------|
| [virtio-spec](https://crates.io/crates/virtio-spec) | VirtIO 设备规范 | QEMU 虚拟设备驱动 |

### 通用工具（全局使用）

| crate | 用途 | 对应模块 |
|-------|------|---------|
| [log](https://crates.io/crates/log) | 统一日志 trait | 全局日志框架 |
| [bitflags](https://crates.io/crates/bitflags) | 位标志宏 | `CapabilitySet` 等 |

## 可参考设计的项目

以下项目不能直接复用代码（语言不同、许可证不同、架构不同），但其设计思路可以借鉴：

| 来源 | 可借鉴内容 | AOS 对应模块 |
|------|-----------|-------------|
| [Redox OS 内核](https://gitlab.redox-os.org/redox-os/kernel) | 微内核 IPC 设计、scheme 机制、上下文切换 | `kernel/ipc/`, `kernel/agent/scheduler/` |
| [Redox rmm](https://gitlab.redox-os.org/redox-os/rmm) | Rust 内存管理实现 | `kernel/memory/` |
| [seL4](https://sel4.systems/) | Capability 安全模型的形式化定义 | `kernel/capability/` |
| Linux VFS | 虚拟文件系统抽象层接口设计 | `kernel/fs/vfs.rs` |
| Linux ELF loader | ELF 可执行文件加载逻辑 | 用户态 Agent 加载 |
| [OpenClaw](https://github.com/openclaw/openclaw) | Agent 间协作模式、技能注册、会话管理 | `kernel/ipc/`, AgentPack 设计 |

## 关键学习资源

| 资源 | 内容 | AOS 中的作用 |
|------|------|-------------|
| [Writing an OS in Rust (Phil Opp)](https://os.phil-opp.com/) | 从零构建 Rust OS 内核的完整教程 | M0 阶段的实现指南：引导、中断、页表、堆分配 |
| [RWTH Aachen LKP 课程](https://teaching.os.rwth-aachen.de/LKP/) | Linux 内核编程：模块、syscall、内存管理、VFS | 路线图 M0-M6 各阶段对应的理论基础 |

## 必须自行设计与实现的（AOS 独有）

以下是 AOS 的核心创新部分，没有现成的可复用组件：

| 模块 | 原因 |
|------|------|
| Agent 原语与生命周期 | AOS 独创概念 |
| Agent 调度器 | 调度目标不同：推理算力 + Token 预算 |
| IPC 消息协议 | AOS 特有的异步语义化消息格式 |
| Capability 的委托链适配 | seL4 可参考，但委托链是新的 |
| Human Interface | 人机接口是 AOS 独有的内核组件 |
| 记忆文件系统 | `/memory/` 是全新概念 |
| 委托链与审计 | 传统 OS 没有对应机制 |

## M0 的实际工作量

借助现有 crate，M0 的实现难度大幅降低：

```
可用现成 crate 的部分：           需要自己实现的部分：

UEFI 引导 ──► bootloader         物理内存管理的具体策略
页表操作  ──► x86_64              （伙伴分配器，参考 Phil Opp）
GDT/IDT  ──► x86_64
中断处理  ──► x86_64 + pic8259    将这些 crate 按 AOS 的 port trait
堆分配器  ──► linked-list-alloc   组装到 hal/ 中
自旋锁   ──► spinning_top
串口输出  ──► uart_16550
```

## 建议的 M0 实现路径

```
第 1 步：参照 Phil Opp 教程，用现成 crate 搭建最小内核骨架
         ├── bootloader crate 引导
         ├── uart_16550 串口输出
         ├── x86_64 + pic8259 中断处理
         ├── x86_64 页表管理
         └── linked-list-allocator 堆分配
         → 此时已有一个能跑在 QEMU 上的最小 Rust 内核

第 2 步：按 AOS 项目结构重组为 api/ + hal/ + kernel/ + init/
         ├── 将 crate 封装为 port trait 实现（放入 hal/）
         ├── 验证 kernel/ 可以在宿主机上 cargo test
         └── 验证 init/ 可以在 QEMU 上启动

第 3 步：进入 M1，开始构建 AOS 独有的 Agent 原语
```
