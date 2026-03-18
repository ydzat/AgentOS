# 项目结构 / Project Structure

## 设计原则

1. **依赖倒置 (DIP)** — 高层模块（kernel）定义接口，低层模块（hal）实现接口，通过 `api` crate 解耦
2. **unsafe 隔离** — 所有 unsafe 代码集中在 `hal/`，kernel 使用 `#![forbid(unsafe_code)]`
3. **可测试性** — kernel 不依赖 hal，可在宿主机 `cargo test`
4. **编译器强制依赖方向** — 通过 Rust crate 边界，禁止反向依赖

## 依赖关系

```
init ──► kernel ──► api ◄── hal
  │                         │
  └────────────────────────►┘
  （init 组装 hal 实现注入 kernel 泛型）
```

- `api`: 零依赖，定义 uabi 类型和 port trait
- `hal`: 依赖 `api`，实现具体硬件操作
- `kernel`: 依赖 `api`（不依赖 `hal`），纯 safe Rust
- `init`: 依赖三者，做依赖注入和引导

## 目录结构

```
AgentOS/
│
├── docs/                          # 设计文档
│
├── api/                           # 共享接口层 [crate: aos-api]
│   ├── src/                       # no_std, 零依赖
│   │   ├── lib.rs
│   │   ├── uabi/                  # 用户态 ABI 类型（syscall 接口契约）
│   │   │   ├── mod.rs
│   │   │   ├── agent.rs           # AgentId, AgentConfig, AgentInfo
│   │   │   ├── message.rs         # Message, MessageType
│   │   │   ├── capability.rs      # Capability 枚举
│   │   │   ├── budget.rs          # ResourceBudget
│   │   │   ├── identity.rs        # UserId, DelegationChain
│   │   │   └── error.rs           # AosError
│   │   └── port/                  # 内核 port trait 定义（DIP 接口）
│   │       ├── mod.rs
│   │       ├── memory.rs          # FrameAllocator, PageMapper trait
│   │       ├── interrupt.rs       # InterruptController trait
│   │       ├── timer.rs           # Timer trait
│   │       └── console.rs         # Console trait
│   └── Cargo.toml
│
├── hal/                           # 硬件抽象层 [crate: aos-hal]
│   ├── src/                       # ⚠️ unsafe 集中于此
│   │   ├── lib.rs
│   │   ├── arch/
│   │   │   ├── x86_64/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── boot.rs        # UEFI 引导
│   │   │   │   ├── gdt.rs         # 全局描述符表
│   │   │   │   ├── idt.rs         # 中断描述符表
│   │   │   │   ├── paging.rs      # 页表操作
│   │   │   │   └── apic.rs        # 中断控制器
│   │   │   └── aarch64/
│   │   │       └── ...
│   │   ├── mm/
│   │   │   ├── mod.rs
│   │   │   ├── frame.rs           # 物理页帧管理
│   │   │   └── buddy.rs           # 伙伴分配器 (impl FrameAllocator)
│   │   ├── sync/                  # 自旋锁、屏障等底层同步原语
│   │   └── console/               # 串口驱动 (impl Console)
│   └── Cargo.toml                 # 依赖 api
│
├── kernel/                        # 内核核心 [crate: aos-kernel]
│   ├── src/                       # #![forbid(unsafe_code)]
│   │   ├── lib.rs
│   │   ├── agent/                 # Agent 子系统
│   │   │   ├── mod.rs
│   │   │   ├── primitive.rs       # AgentKernel struct
│   │   │   ├── state.rs           # 状态机 (State Pattern)
│   │   │   ├── registry.rs        # Agent 注册表
│   │   │   └── scheduler/         # 调度器 (Strategy Pattern)
│   │   │       ├── mod.rs
│   │   │       ├── r#trait.rs     # Scheduler trait
│   │   │       ├── round_robin.rs # 轮询调度
│   │   │       └── priority.rs    # 优先级调度
│   │   ├── ipc/                   # 进程间通信
│   │   │   ├── mod.rs
│   │   │   ├── queue.rs           # 异步消息队列
│   │   │   ├── endpoint.rs        # 端点管理
│   │   │   └── router.rs          # 消息路由 (Mediator Pattern)
│   │   ├── capability/            # 安全模型
│   │   │   ├── mod.rs
│   │   │   ├── set.rs             # CapabilitySet
│   │   │   └── safety.rs          # Safety Agent 策略
│   │   ├── budget/                # 资源预算
│   │   │   ├── mod.rs
│   │   │   └── accounting.rs      # 计量与强制执行
│   │   ├── memory/                # 内核内存管理 (safe 封装)
│   │   │   ├── mod.rs
│   │   │   ├── address_space.rs   # 地址空间
│   │   │   ├── heap.rs            # 内核堆
│   │   │   └── slab.rs            # 对象分配器
│   │   ├── human/                 # 人机接口（内核组件，不是 Agent）
│   │   │   ├── mod.rs
│   │   │   ├── interface.rs       # 人类控制通道（不可被 Agent 阻断）
│   │   │   ├── delegation.rs      # 委托链管理
│   │   │   ├── audit.rs           # 行为审计（人类可随时查看）
│   │   │   └── killswitch.rs      # 紧急终止
│   │   ├── fs/                    # 文件系统
│   │   │   ├── mod.rs
│   │   │   ├── vfs.rs             # 虚拟文件系统层
│   │   │   ├── namespace.rs       # 挂载点 (/home, /sys, /apps, /memory, ...)
│   │   │   └── permission.rs      # 基于委托链的访问控制
│   │   ├── event/                 # 内核事件系统 (Observer Pattern)
│   │   │   ├── mod.rs
│   │   │   └── bus.rs             # 事件总线
│   │   └── syscall/               # 系统调用分发
│   │       ├── mod.rs
│   │       ├── dispatch.rs        # 调用号分发
│   │       ├── agent.rs           # Agent 相关
│   │       ├── ipc.rs             # IPC 相关
│   │       └── mem.rs             # 内存相关
│   └── Cargo.toml                 # 依赖 api（不依赖 hal）
│
├── init/                          # 引导入口 [crate: aos-init, 二进制]
│   ├── src/
│   │   └── main.rs                # 引导 → 组装依赖 → 启动首个 Agent
│   ├── linker/
│   │   ├── x86_64.ld              # x86-64 链接脚本
│   │   └── aarch64.ld             # ARM64 链接脚本
│   └── Cargo.toml                 # 依赖 api + hal + kernel
│
├── userspace/                     # 用户态服务 (M5+ 阶段)
│   ├── init-agent/                # 首个用户态 Agent (类似 Linux init)
│   ├── inference-server/          # 推理引擎服务
│   ├── memory-fs/                 # 记忆文件系统
│   └── device-manager/            # 设备管理服务
│
├── examples/                      # 示例 Agent
│   ├── hello-agent/               # 最简 Agent (M1 验证)
│   ├── echo-agent/                # IPC 回显 (M2 验证)
│   └── collab-agents/             # 多 Agent 协作 (M8 验证)
│
├── tests/                         # 测试基础设施
│   ├── integration/               # QEMU 集成测试
│   └── e2e/                       # 端到端场景测试
│
├── tools/                         # 开发工具
│   ├── qemu-runner/               # QEMU 启动与调试
│   └── image-builder/             # 磁盘镜像构建
│
├── Cargo.toml                     # workspace root
└── rust-toolchain.toml
```

## Cargo Workspace 配置

```toml
# 根 Cargo.toml
[workspace]
resolver = "2"
members = [
    "api",
    "hal",
    "kernel",
    "init",
]

# crate 发布名使用 aos- 前缀（避免 crates.io 冲突）
# 目录名使用简短形式
```

各 crate 的 Cargo.toml 中 `name` 字段使用 `aos-` 前缀：

```toml
# api/Cargo.toml
[package]
name = "aos-api"

# hal/Cargo.toml
[package]
name = "aos-hal"
[dependencies]
aos-api = { path = "../api" }

# kernel/Cargo.toml
[package]
name = "aos-kernel"
[dependencies]
aos-api = { path = "../api" }
# 注意: 不依赖 aos-hal

# init/Cargo.toml
[package]
name = "aos-init"
[dependencies]
aos-api = { path = "../api" }
aos-hal = { path = "../hal" }
aos-kernel = { path = "../kernel" }
```

## 设计模式应用

| 模式 | 位置 | 用途 |
|------|------|------|
| State Pattern | `kernel/src/agent/state.rs` | Agent 生命周期状态机，编译期保证状态转换合法性 |
| Strategy Pattern | `kernel/src/agent/scheduler/` | 调度算法可插拔（轮询 / 优先级 / 未来自定义） |
| Builder Pattern | `api/src/uabi/agent.rs` | AgentConfig 构建，复杂参数分步配置 |
| Mediator Pattern | `kernel/src/ipc/router.rs` | IPC 消息路由集中管理 |
| Observer Pattern | `kernel/src/event/` | 内核事件通知（预算耗尽、Agent 死亡等） |

## unsafe 隔离策略

```
hal/     ⚠️ unsafe 允许（硬件交互不可避免）
kernel/  ✅ #![forbid(unsafe_code)]
api/     ✅ #![forbid(unsafe_code)]
init/    ⚠️ unsafe 允许（引导阶段需要少量 unsafe 组装）
```

审计安全性时，只需重点审查 `hal/` 和 `init/`。

## 测试策略

| 层级 | 位置 | 运行环境 | 说明 |
|------|------|---------|------|
| 单元测试 | 各 crate 内 `#[cfg(test)]` | 宿主机 `cargo test` | mock 硬件，验证逻辑正确性 |
| 集成测试 | `tests/integration/` | QEMU | 验证 syscall 接口、IPC 通信 |
| 端到端测试 | `tests/e2e/` | QEMU | 完整的 Agent 协作场景 |

kernel 可以在宿主机上跑单元测试，因为它不依赖 hal，所有硬件交互通过 trait mock。

## Cargo Features 配置策略

```toml
# hal/Cargo.toml
[features]
default = ["arch-x86_64"]
arch-x86_64 = []
arch-aarch64 = []
```

参考 Linux Kconfig 的思路，通过 Cargo features 控制编译时配置（目标架构、可选驱动等）。
