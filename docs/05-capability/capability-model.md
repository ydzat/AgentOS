# Capability 安全模型 / Capability-Based Security

## 设计决策

**Capability 嵌入 Agent 内核原语（CapabilitySet 作为 AgentKernel 的字段）。**

## 推导依据

### 正向推导（从内核实现）

| 方案 | 权限检查速度 | 内存开销 | 复杂度 |
|------|------------|---------|--------|
| 嵌入 Agent 结构体 | O(1) bitset 查找 | Agent 结构体变大 | 低 |
| 外挂 Capability 表 | O(1) 表查找 + 间接访问 | 独立表需管理生命周期 | 中 |

### 反向推导（从使用模式）

- 每次 Tool 调用都要检查权限 → **高频操作，必须快**
- 权限需要动态调整（Safety Agent 可撤销）→ 嵌入也支持 `mut`
- 子 Agent 权限从父 Agent 派生 → `derive_subset()` 操作

### 结论

嵌入方案更简单、更快，适合 AOS 的场景。

## Capability 定义

```rust
/// Capability 集合（bitset 实现）
struct CapabilitySet {
    bits: u128,  // 支持最多 128 种能力
}

impl CapabilitySet {
    /// 检查是否拥有某能力
    fn has(&self, cap: Capability) -> bool;

    /// 授予能力
    fn grant(&mut self, cap: Capability);

    /// 撤销能力
    fn revoke(&mut self, cap: Capability);

    /// 从当前集合派生子集（子 Agent 创建时使用）
    fn derive_subset(&self, mask: u128) -> CapabilitySet;
}
```

## 能力类型（初步定义）

```rust
enum Capability {
    // ---- Agent 管理 ----
    AgentCreate     = 0,    // 创建子 Agent
    AgentDestroy    = 1,    // 销毁 Agent
    AgentSuspend    = 2,    // 挂起 Agent

    // ---- IPC ----
    IpcSend         = 8,    // 发送消息
    IpcRecv         = 9,    // 接收消息
    IpcBroadcast    = 10,   // 广播消息

    // ---- 内存 ----
    MemAlloc        = 16,   // 分配内存
    MemMap          = 17,   // 映射共享内存

    // ---- 设备 ----
    DeviceAccess    = 24,   // 访问设备
    DeviceControl   = 25,   // 设备控制（ioctl）

    // ---- 推理 ----
    InferenceLocal  = 32,   // 使用本地推理
    InferenceRemote = 33,   // 使用远程推理

    // ---- 记忆 ----
    MemoryStore     = 40,   // 存储记忆
    MemoryRecall    = 41,   // 检索记忆
    MemoryForget    = 42,   // 删除记忆

    // ---- 特权 ----
    CapGrant        = 56,   // 授予能力给其他 Agent
    CapRevoke       = 57,   // 撤销其他 Agent 的能力
    SafetyOverride  = 63,   // Safety Agent 专用：覆盖任何决策

    // 64-127: 预留给 AgentPack 自定义能力
}
```

## 安全规则

### 1. 权限不可提升

子 Agent 的 Capability **不能超过**父 Agent 的 Capability：

```
Parent Agent: {AgentCreate, IpcSend, IpcRecv, MemAlloc, InferenceLocal}

可以创建:
  Child Agent: {IpcSend, IpcRecv, InferenceLocal}  ✓（是子集）

不可以创建:
  Child Agent: {IpcSend, DeviceAccess}  ✗（DeviceAccess 不在父 Agent 中）
```

### 2. Safety Agent

Safety Agent 是系统中拥有 `SafetyOverride` 能力的特权 Agent：

- 可以撤销任何 Agent 的任何 Capability
- 可以挂起/销毁任何 Agent
- 可以覆盖任何 Agent 的决策
- 系统启动时由内核直接创建，不受其他 Agent 管控

### 3. 每次 Tool 调用检查

Agent 执行任何系统调用时，内核检查流程：

```
Agent 发起 syscall
  │
  ▼
内核检查 agent.capabilities.has(required_cap)
  │
  ├── 有权限 → 检查 budget → 执行
  │
  └── 无权限 → 返回 PermissionDenied
```

### 4. 动态调整

```rust
// 父 Agent 授予子 Agent 新能力（不能超过自身）
aos_cap_grant(child_id, Capability::MemoryStore);

// Safety Agent 撤销某 Agent 的能力
aos_cap_revoke(target_id, Capability::DeviceAccess);
```
