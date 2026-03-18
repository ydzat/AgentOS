# 系统调用 ABI / Syscall ABI

## 设计原则

1. **按模块分组编号**，每组预留 64 个号，便于未来扩展
2. **编号一旦发布不可更改**（向后兼容）
3. **前 4 个参数走寄存器**，复杂参数走用户态指针
4. **逐里程碑实现**，早期只实现少数核心调用

## 调用约定

### x86-64

- 系统调用指令: `syscall`
- 系统调用号: `rax`
- 参数: `rdi`, `rsi`, `rdx`, `r10`, `r8`, `r9`
- 返回值: `rax`

### ARM64

- 系统调用指令: `svc #0`
- 系统调用号: `x8`
- 参数: `x0` - `x5`
- 返回值: `x0`

## 系统调用编号表

### 0x00 - 0x3F: Agent 生命周期（M1 阶段实现）

| 编号 | 名称 | 参数 | 说明 |
|------|------|------|------|
| 0x00 | `aos_agent_create` | config: *const AgentConfig | 创建子 Agent |
| 0x01 | `aos_agent_destroy` | id: AgentId | 销毁 Agent |
| 0x02 | `aos_agent_suspend` | id: AgentId | 挂起 Agent |
| 0x03 | `aos_agent_resume` | id: AgentId | 恢复 Agent |
| 0x04 | `aos_agent_info` | id: AgentId, buf: *mut AgentInfo | 查询 Agent 状态 |
| 0x05 | `aos_agent_wait` | id: AgentId | 等待 Agent 结束 |
| 0x06 | `aos_agent_list` | buf: *mut AgentId, count: *mut u32 | 列出子 Agent |
| 0x07-0x3F | *预留* | | |

### 0x40 - 0x7F: IPC（M2 阶段实现）

| 编号 | 名称 | 参数 | 说明 |
|------|------|------|------|
| 0x40 | `aos_ipc_send` | receiver, msg_type, request_id, payload, len | 发送消息 |
| 0x41 | `aos_ipc_recv` | buf: *mut Message, flags: RecvFlags | 接收消息 |
| 0x42 | `aos_ipc_reply` | request_id, payload, len | 回复请求 |
| 0x43 | `aos_ipc_peek` | buf: *mut Message | 查看队列头部消息（不取出） |
| 0x44-0x7F | *预留* | | |

### 0x80 - 0xBF: Capability（M3 阶段实现）

| 编号 | 名称 | 参数 | 说明 |
|------|------|------|------|
| 0x80 | `aos_cap_grant` | target: AgentId, cap: Capability | 授予能力 |
| 0x81 | `aos_cap_revoke` | target: AgentId, cap: Capability | 撤销能力 |
| 0x82 | `aos_cap_check` | target: AgentId, cap: Capability | 检查能力 |
| 0x83 | `aos_cap_list` | target: AgentId, buf: *mut CapList | 列出能力集合 |
| 0x84-0xBF | *预留* | | |

### 0xC0 - 0xFF: 内存管理（M0 阶段实现）

| 编号 | 名称 | 参数 | 说明 |
|------|------|------|------|
| 0xC0 | `aos_mem_alloc` | size: usize, flags: AllocFlags | 分配内存 |
| 0xC1 | `aos_mem_free` | addr: *mut u8, size: usize | 释放内存 |
| 0xC2 | `aos_mem_map` | handle: MemHandle, addr: *mut u8, len: usize | 映射共享内存 |
| 0xC3 | `aos_mem_unmap` | addr: *mut u8, len: usize | 取消映射 |
| 0xC4-0xFF | *预留* | | |

### 0x100 - 0x13F: 资源预算（M3 阶段实现）

| 编号 | 名称 | 参数 | 说明 |
|------|------|------|------|
| 0x100 | `aos_budget_query` | buf: *mut ResourceBudget | 查询剩余预算 |
| 0x101 | `aos_budget_yield` | — | 主动让出 CPU |
| 0x102 | `aos_budget_set` | target: AgentId, budget: *const ResourceBudget | 设置子 Agent 预算 |
| 0x103-0x13F | *预留* | | |

### 0x140 - 0x17F: 记忆文件系统（M6 阶段实现）

| 编号 | 名称 | 参数 | 说明 |
|------|------|------|------|
| 0x140 | `aos_memory_store` | key, value, len, flags | 存储记忆 |
| 0x141 | `aos_memory_recall` | key, buf, buf_len | 检索记忆 |
| 0x142 | `aos_memory_forget` | key | 删除记忆 |
| 0x143 | `aos_memory_list` | prefix, buf, count | 列出记忆 |
| 0x144-0x17F | *预留* | | |

### 0x180 - 0x1BF: 设备（M5 阶段实现）

| 编号 | 名称 | 参数 | 说明 |
|------|------|------|------|
| 0x180 | `aos_device_open` | name, flags | 打开设备 |
| 0x181 | `aos_device_read` | fd, buf, len | 读取设备 |
| 0x182 | `aos_device_write` | fd, buf, len | 写入设备 |
| 0x183 | `aos_device_close` | fd | 关闭设备 |
| 0x184 | `aos_device_ioctl` | fd, cmd, arg | 设备控制 |
| 0x185-0x1BF | *预留* | | |

### 0x200+: 预留扩展

未来可能的扩展方向：
- 网络通信
- 跨机器 Agent 迁移
- 调试/追踪接口
- AgentPack 管理

## 错误码

```rust
enum AosError {
    Ok              = 0,
    InvalidAgent    = -1,   // AgentId 不存在
    PermissionDenied = -2,  // Capability 不足
    BudgetExhausted = -3,   // 资源预算耗尽
    InvalidArgument = -4,   // 参数无效
    WouldBlock      = -5,   // 非阻塞调用但当前无数据
    NoMemory        = -6,   // 内存不足
    QueueFull       = -7,   // IPC 队列已满
    NotFound        = -8,   // 资源未找到
}
```
