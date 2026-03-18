# Agent 内核原语 / Agent Kernel Primitive

## 设计原则

内核侧的 Agent 原语应当**精简**，只包含内核调度、安全管控、IPC 所必需的字段。Agent 的丰富状态（推理上下文、记忆、目标）由用户态管理。

## 推导过程

### 正向推导（内核需要维护什么最小状态？）

内核必须知道的（否则无法调度/管控）：
- `identity` → 谁？（调度、审计、权限查找）
- `state` → 什么状态？（运行/等待/挂起）
- `address_space` → 页表指针（独立地址空间）
- `capabilities` → 能做什么？（Capability 系统需要）
- `budget` → 还剩多少资源？（预算强制执行）
- `ipc_endpoint` → 消息队列（IPC 是内核管的）

内核不需要知道的（应该在用户态管理）：
- `model_context` → 模型状态（推理引擎的事）
- `memory` → 记忆内容（文件系统的事）
- `goal` → 当前目标（Agent 运行时自己管）

### 反向推导（Agent 运行时需要什么？）

Agent 运行时需要的丰富状态，通过 handle/引用与内核关联，但内容在用户态。

## 数据结构定义

### 内核态

```rust
/// 内核维护的 Agent 状态（内核直接管理）
struct AgentKernel {
    /// 唯一标识符
    id:            AgentId,

    /// 生命周期状态
    state:         AgentState,

    /// 独立地址空间（页表引用）
    address_space: AddressSpaceRef,

    /// 可调用的 Tool/能力集合（嵌入，O(1) 检查）
    capabilities:  CapabilitySet,

    /// 资源预算余额（算力/内存/Token）
    budget:        ResourceBudget,

    /// IPC 消息收发端点
    ipc_endpoint:  Endpoint,

    /// 创建者（用于层级管控，根 Agent 为 None）
    parent:        Option<AgentId>,

    /// 调度优先级
    priority:      Priority,
}
```

### 用户态

```rust
/// Agent 运行时自己管理的状态（内核不关心内容）
struct AgentUser {
    /// 当前目标
    goal:          Goal,

    /// 模型状态引用（指向用户态推理服务）
    model_context: ModelRef,

    /// 记忆句柄（通过 syscall 访问记忆文件系统）
    memory:        MemoryHandle,

    /// 可用工具注册表
    tools:         ToolRegistry,
}
```

## Agent 生命周期状态机

```
          aos_agent_create()
                │
                ▼
         ┌──────────┐
         │ Created  │
         └────┬─────┘
              │ 初始化完成
              ▼
         ┌──────────┐ ◄──── aos_agent_resume()
    ┌───►│ Running  │
    │    └────┬─────┘
    │         │
    │    ┌────┴──────────────┐
    │    │                   │
    │    ▼                   ▼
    │ ┌──────────┐    ┌───────────┐
    │ │ Waiting  │    │ Suspended │
    │ │(等待IPC/ │    │ (被挂起)   │
    │ │ 推理结果) │    └───────────┘
    │ └────┬─────┘          │
    │      │ 消息到达/       │ aos_agent_suspend()
    │      │ 结果返回        │  (来自父Agent或SafetyAgent)
    └──────┘                │
                            │
              ▼             ▼
         ┌──────────┐
         │  Dead    │
         └──────────┘
              │
              ▼
        资源回收完成
```

### 状态说明

| 状态 | 说明 |
|------|------|
| Created | Agent 已创建，地址空间和结构体已初始化，尚未开始运行 |
| Running | Agent 正在 CPU 上执行（或在就绪队列中等待调度） |
| Waiting | Agent 主动等待（IPC 消息、推理结果等），让出 CPU |
| Suspended | 被父 Agent 或 Safety Agent 强制挂起 |
| Dead | Agent 已终止，等待内核回收资源 |

## Capability 嵌入设计

### 决策：Capability 嵌入 Agent 原语

推导依据：
- **正向**：嵌入后权限检查是 O(1)，无需查外部表
- **反向**：每次 Tool 调用都要检查权限，是高频操作，必须快

### 权限传递规则

1. 子 Agent 的 Capability **不能超过**父 Agent 的 Capability
2. Safety Agent 拥有最高权限，可撤销任何 Agent 的 Capability
3. Capability 动态调整通过内核接口 `aos_cap_grant()` / `aos_cap_revoke()`

```rust
/// Capability 集合（bitset 实现，O(1) 检查）
struct CapabilitySet {
    bits: u128,  // 支持最多 128 种能力，可扩展
}

impl CapabilitySet {
    fn has(&self, cap: Capability) -> bool;
    fn grant(&mut self, cap: Capability);
    fn revoke(&mut self, cap: Capability);
    fn derive_subset(&self, mask: u128) -> CapabilitySet;
}
```

## 资源预算

```rust
struct ResourceBudget {
    /// 推理 Token 预算（剩余可用量）
    token_remaining:  u64,

    /// 内存使用上限（字节）
    memory_limit:     usize,

    /// 当前内存使用量
    memory_used:      usize,

    /// CPU 时间预算（纳秒）
    cpu_time_limit:   u64,

    /// 已使用 CPU 时间
    cpu_time_used:    u64,
}
```

预算由内核强制执行。Agent 超出预算时，内核将其状态设为 Suspended 并通知父 Agent。
