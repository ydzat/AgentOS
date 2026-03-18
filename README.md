Language: [🇬🇧](README_EN.md)

# 智核 – Agent Operating System

> Agent 即进程，推理即计算，记忆即文件系统。

---

## 简介

**智核（AOS）是一个独立的操作系统内核，以 AI Agent 为一等公民，用 Rust 从零构建。**

AOS 不是运行在 Linux/macOS/Windows 之上的中间件，而是与它们并列的第四类操作系统——为 AI Agent 的自主运行而设计，而非为人类的手动操作而设计。

---

## 设计哲学

传统操作系统的核心假设是"人坐在机器前操作"。AOS 的核心假设是：**执行者是 AI Agent，而非人类编写的程序。**

| 传统 OS | AOS |
|---------|-----|
| 核心抽象：进程 + 文件 | 核心抽象：Agent + 记忆 |
| 调度资源：CPU 时间片 | 调度资源：推理算力 + Token 预算 |
| 安全模型：用户/组/权限位 | 安全模型：Capability-based |
| 交互模式：人 → 程序 → 结果 | 交互模式：人 → 意图 → Agent 自主执行 |

---

## 系统架构

AOS 采用**微内核架构**，内核只做最小必要的事，推理引擎、驱动等全部运行在用户态。

```
┌─────────────────────────────────────────────┐
│                  用户层                       │
│  PersonalAgent（用户的代理身份）               │
│  应用 Agent · 工具 Agent · 协作 Agent         │
├─────────────────────────────────────────────┤
│              用户态服务                       │
│  推理引擎 · 记忆文件系统 · 设备驱动            │
│  AgentPack 模块系统 · AOS Hub                │
├─────────────────────────────────────────────┤
│              AOS 微内核 (Rust)                │
│  Agent 生命周期 · IPC · Capability 管控       │
│  资源预算执行 · 中断/异常 · 最小内存管理        │
├─────────────────────────────────────────────┤
│              硬件                            │
│  CPU · GPU/NPU · 传感器 · 执行器 · 存储       │
└─────────────────────────────────────────────┘
```

### 微内核职责

内核只负责以下机制，不包含任何策略：

- **Agent 生命周期管理** — 创建 / 销毁 / 挂起 / 恢复
- **IPC（进程间通信）** — Agent 间消息传递的唯一通道
- **Capability 权限管理** — Agent 能调用哪些 Tool，由内核强制执行
- **资源预算强制执行** — 算力 / 内存 / Token 配额的硬限制
- **中断与异常处理** — 基础硬件抽象
- **最小内存管理** — 页表、地址空间隔离

推理引擎、设备驱动、文件系统等全部在用户态运行。推理服务崩溃时，内核可以安全地重启它，不会导致系统崩溃。

### Agent 原语

Agent 是 AOS 的一等公民，类似传统 OS 中的进程：

```rust
struct Agent {
    identity: AgentId,          // 身份与权限
    model_context: ModelRef,    // 模型状态引用
    memory: MemoryHandle,       // 持久化记忆（短期 + 长期）
    tools: CapabilitySet,       // 可调用的系统能力
    goal: Goal,                 // 当前目标（替代 program counter）
    budget: ResourceBudget,     // 算力/Token/时间预算
}
```

### 推理架构

AOS 将推理引擎与内核完全解耦，支持灵活的推理后端：

- **本地推理** — 通过用户态驱动访问 GPU/NPU，运行量化模型
- **远程推理** — 将推理请求卸载到云端 API
- **混合推理** — 本地小模型做快速决策，复杂任务卸载到远程

内核不关心推理在哪里发生，只管理 Agent 的生命周期和资源预算。

---

## 核心能力

### Agent 自主协作

多个 Agent 通过内核 IPC 直接通信、分工协作，无需人类中介：

```
用户: "帮我整理实验数据并生成报告"

Planner Agent → 拆解任务
  ├→ DataSci Agent → 分析数据、生成统计
  ├→ Writer Agent  → 撰写文字、组织结构
  └→ Critic Agent  → 审查一致性与准确性
```

### Agent 即用户

Personal Agent 是用户在 AOS 中的常驻代理身份：
- 理解用户偏好、习惯和目标
- 代替用户与其他 Agent 交互
- 在用户不在场时持续工作
- 多个 Personal Agent 可互相协作，形成组织级协作

### 能力动态合成

当系统不具备某项能力时，可从 AOS Hub 动态拉取 AgentPack 模块，按需组合成新的 Agent。类似动态链接库，但链接的不是函数，而是能力。

### 安全与隔离

- **Capability-based 安全模型** — Agent 只能使用被显式授予的 Tool
- **资源预算硬限制** — 防止 Agent 无限消耗算力
- **Safety Agent** — 特权监督 Agent，可覆盖任何其他 Agent 的决策
- **内核级地址空间隔离** — 单个 Agent 崩溃不影响系统

---

## 目标硬件平台

AOS 微内核本身极为轻量，硬件需求取决于用户态推理服务的配置：

| 场景 | 最低配置 |
|------|---------|
| 开发/调试（QEMU） | x86-64 / ARM64, 4 核, 32GB RAM, 8GB VRAM GPU |
| 边缘/嵌入式 | ARM/RISC-V + 专用 NPU, 512MB-2GB RAM |
| 机器人平台 | ARM64 + NPU, 8GB+ RAM, 实时传感器接口 |

---

## 应用场景

### 自主软件开发

Agent 群组自主完成从需求分析到代码审查的完整开发流程。

### 机器人自主控制

AOS 天然适合机器人平台——Agent 即进程的设计让感知、规划、运动控制、安全监督等模块作为独立 Agent 运行，通过内核 IPC 实现微秒级协作，无需 ROS 等中间件层。

### 自适应系统管理

系统自我监控、自我维护，无需人工运维。

### 多模态创作协作

多个专业 Agent 协同完成跨领域创作任务（数据分析 + 写作 + 可视化）。

---

## 技术栈

- **内核语言**: Rust（内存安全、零开销抽象、并发安全）
- **目标架构**: x86-64 / ARM64（初期），RISC-V（计划中）
- **引导方式**: UEFI
- **开发环境**: QEMU/KVM 虚拟机

---

## 设计参考

| 项目 | 参考价值 |
|------|---------|
| [Redox OS](https://www.redox-os.org/) | Rust 微内核 OS 的先驱实现 |
| [Asterinas](https://github.com/asterinas/asterinas) | Rust 编写的生产级 Linux 替代方案 |
| [seL4](https://sel4.systems/) | 形式化验证微内核，Capability 安全模型参考 |
| [OpenClaw](https://github.com/openclaw/openclaw) | 应用层 Agent 协作框架，验证了 Agent 自主协作的可行性 |

---

## 许可证

AOS 采用分层许可模型：

- **内核**（`api/`, `hal/`, `kernel/`, `init/`）— [GPL v2.0](LICENSE-GPL2)
- **用户态服务**（`userspace/`, `examples/`, `tools/`）— [Apache 2.0](LICENSE-APACHE2)
- **AgentPack 生态** — 开发者自选许可

系统调用接口（AOS ABI）为许可边界。详见 [LICENSING.md](LICENSING.md)。

---

## 项目状态

> 🚧 早期设计阶段 — 欢迎参与讨论与贡献

---

## 命名

- **中文全称**: 智能代理操作系统
- **简称**: 智核（Zhì Hé）
- **英文**: Agent Operating System (AOS)
