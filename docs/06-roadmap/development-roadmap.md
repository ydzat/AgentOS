# 开发路线图 / Development Roadmap

## 参考框架

本路线图参考了 RWTH Aachen 大学 Linux Kernel Programming 课程（LKP）的教学顺序，将 Linux 内核从零到一的构建过程映射到 AOS 的设计。

### LKP 课程章节 → AOS 映射

| LKP 章节 | AOS 对应阶段 |
|----------|-------------|
| Ch1: History & Architecture | M0-prep: 架构定义 |
| Ch2: C Bootcamp & Kernel Programming | M0-prep: Rust 内核编程基础 |
| Ch3: Implementing Kernel Modules | M4: AgentPack 模块系统 |
| Ch4: User-Kernel Communication | M2: 系统调用 + Agent IPC |
| Ch5: Memory Management | M0: 内存管理 |
| Ch6: Virtual File System | M6: 记忆文件系统 |
| Ch7: Tracing Facilities | M7: 调试与可观测性 |

## 里程碑

### M0 — Boot & Memory Foundation [地基]

**目标：** 内核能启动，基本内存管理就位。

实现内容：
- UEFI 引导 → 内核启动
- 物理内存管理（页帧分配器）
- 虚拟内存（页表、地址空间）
- 内核堆分配器（类 slab）
- 串口输出（调试用）

**顶层设计锁定：** 内存模型（每 Agent 独立地址空间）

**参考：** LKP Ch5 Memory Management — 页帧分配、伙伴分配器、Slab 层

---

### M1 — Agent Primitive & Scheduling [核心原语]

**目标：** 第一个 Agent 能在内核上创建并运行。

实现内容：
- AgentKernel 结构体定义与创建/销毁
- 最小调度器（协作式 → 后续迭代为抢占式）
- Agent 状态机（Created / Running / Waiting / Suspended / Dead）
- Mock 推理（硬编码响应，验证 Agent 框架）

**顶层设计锁定：** Agent 原语结构、生命周期状态机

**里程碑验证：** "Hello Agent" — 一个 Agent 启动，执行 mock 推理，输出结果到串口。

---

### M2 — IPC & Syscall Interface [通信层]

**目标：** 多个 Agent 能互相通信，内核/用户态分离。

实现内容：
- Agent 间异步消息传递（Message 结构体、消息队列）
- 系统调用接口定义（AOS ABI）
- 内核/用户态分离
- 用户态进程加载（ELF loader）

**顶层设计锁定：** IPC 协议、Syscall ABI 编号（一旦稳定不应再改）

**参考：** LKP Ch4 User-Kernel Communication — 系统调用架构、ABI 设计

**里程碑验证：** Agent A 发送 Request 给 Agent B，B 处理后返回 Response，A 收到并输出。

---

### M3 — Capability & Budget [安全层]

**目标：** Agent 权限受控，资源使用受限。

实现内容：
- Capability-based 权限系统（CapabilitySet bitset）
- 资源预算计量与强制执行（ResourceBudget）
- Safety Agent 特权机制
- 权限不可提升规则

**里程碑验证：** Agent 尝试无权限操作 → 被内核拒绝；Agent 超出预算 → 被内核挂起。

---

### M4 — AgentPack Module System [模块化]

**目标：** Agent 能力可动态加载。

实现内容：
- AgentPack 动态加载/卸载
- 能力注册与依赖解析
- 模块生命周期管理

**参考：** LKP Ch3 Implementing Kernel Modules — 模块接口、EXPORT_SYMBOL、模块依赖

**里程碑验证：** 运行时加载一个 AgentPack，Agent 获得新能力并使用。

---

### M5 — Device Drivers & Real Inference [连接真实世界]

**目标：** AOS 能与真实硬件和推理引擎交互。

实现内容：
- 用户态驱动框架
- GPU/NPU 驱动（或网络栈用于远程推理）
- 推理引擎作为用户态服务接入

**里程碑验证：** Agent 通过用户态推理服务调用真实模型（本地或远程），返回推理结果。

---

### M6 — Memory Filesystem [Agent 记忆]

**目标：** Agent 拥有持久化记忆。

实现内容：
- VFS 抽象层
- Agent 记忆持久化存储（基于存储驱动）
- 短期/长期记忆分层
- 记忆检索接口

**参考：** LKP Ch6 Virtual File System — VFS 抽象层、inode/dentry/superblock 概念映射

**里程碑验证：** Agent 存储记忆 → 重启后 → Agent 检索到之前存储的记忆。

---

### M7 — Tracing & Observability [可观测性]

**目标：** 能观测和调试 Agent 行为。

实现内容：
- Agent 行为追踪（类似 ftrace/eBPF 的 Agent 版本）
- 性能分析
- 调试接口

**参考：** LKP Ch7 Tracing Facilities in the Kernel

---

### M8 — End-to-End Demo [整合演示]

**目标：** 完整的 AOS 端到端演示。

实现内容：
- PersonalAgent 运行
- 接收用户意图 → 拆解任务 → 协调多个 Agent 协作 → 返回结果
- 动态加载 AgentPack 获取新能力
- 完整的安全与预算管控

**里程碑验证：** 用户输入 "整理数据并生成报告" → PersonalAgent 创建 DataSci Agent + Writer Agent → 协作完成 → 返回结果。

## M0 之前需要锁定的顶层设计

| 设计决策 | 结论 | 影响范围 | 文档 |
|---------|------|---------|------|
| Agent 地址空间 | 独立页表 | M0 内存模型 | [01-architecture](../01-architecture/microkernel.md) |
| Agent 内核原语字段 | 精简内核态 + 丰富用户态 | M1 起所有模块 | [02-agent-primitive](../02-agent-primitive/agent-kernel-struct.md) |
| IPC 消息格式 | 异步队列 + 固定头 + 可变载荷 | M2 起所有通信 | [03-ipc](../03-ipc/message-format.md) |
| Syscall ABI 编号 | 分组编号，每组预留 64 | M2 起所有用户态程序 | [04-syscall-abi](../04-syscall-abi/syscall-table.md) |
| Capability 嵌入 | 嵌入 Agent 原语 | M1 预留，M3 实现 | [05-capability](../05-capability/capability-model.md) |
