# IPC 消息格式 / IPC Message Format

## 设计决策

**异步消息队列 + 固定消息头 + 可变载荷**

## 推导过程

### 正向推导（内核能高效传递什么？）

| 方案 | 优点 | 缺点 |
|------|------|------|
| 同步消息（seL4 风格） | 实现简单，无需内核缓冲区 | 发送方阻塞，不适合等待推理的场景 |
| 异步消息队列（Mach/QNX 风格） | 发送后继续工作 | 需要内核管理队列和缓冲区 |

Agent 的"思考"需要时间（等待推理），同步模式会导致发送方白白等待。

### 反向推导（Agent 间通信的实际模式）

Agent 间通信通常是：
1. "请帮我做 X"（任务请求）
2. "X 的结果是 Y"（任务响应）
3. "我发现了 Z，你可能需要知道"（事件通知）
4. "停下来"（控制指令）

特征：
- 有明确的消息类型
- 请求-响应是最常见模式
- 载荷大小变化大（控制指令几字节，任务结果可能几 KB）
- 发出请求后通常想继续工作（异步）
- 同步语义可以在异步之上模拟

### 结论

选择异步消息队列。内核不解析载荷（保持精简），语义由用户态 Agent 运行时定义。

## 消息结构

```rust
/// IPC 消息
struct Message {
    // ---- 固定头（内核解析）----

    /// 发送方 AgentId（由内核填写，不可伪造）
    sender:      AgentId,

    /// 接收方 AgentId
    receiver:    AgentId,

    /// 消息类型
    msg_type:    MessageType,

    /// 请求 ID（用于匹配请求-响应对）
    request_id:  u64,

    /// 载荷长度（字节）
    payload_len: u32,

    // ---- 可变载荷（内核不解析，直接传递）----

    /// 载荷数据（Agent 自己定义序列化格式）
    payload:     [u8],
}
```

### 消息类型

```rust
enum MessageType {
    /// 请求对方执行任务
    Request,

    /// 返回执行结果（携带对应的 request_id）
    Response,

    /// 单向通知（不期待回复）
    Notify,

    /// 系统控制指令（停止/挂起/查询等）
    Control,
}
```

## 关键设计细节

### sender 由内核填写

Agent 发送消息时，内核自动填写 `sender` 字段为发送方的 `AgentId`。Agent 无法伪造身份。

这是安全模型的基础——接收方可以信任 `sender` 字段。

### request_id 用于请求-响应匹配

异步模式下，一个 Agent 可能同时发出多个请求。`request_id` 由发送方生成（单调递增即可），响应方原样返回，发送方据此匹配。

```
Agent A                    Agent B
  │                          │
  │── Request(id=1) ────────►│
  │── Request(id=2) ────────►│  A 可以继续工作
  │                          │
  │◄── Response(id=2) ──────│  B 先完成了 id=2
  │◄── Response(id=1) ──────│  B 再完成 id=1
  │                          │
  A 通过 id 匹配哪个请求被回复了
```

### 载荷序列化

内核不关心载荷格式。用户态 Agent 运行时可以选择任意序列化方案：
- 简单场景：自定义二进制格式
- 复杂场景：MessagePack / FlatBuffers 等

载荷大小限制由内核配置决定（防止单条消息耗尽内核缓冲区）。

### 大载荷传递

对于超大数据（如图像、模型权重），不通过 IPC 消息体传递。改为：
1. 发送方在共享内存区域写入数据
2. 通过 IPC 消息传递共享内存的 handle
3. 接收方通过 `aos_mem_map()` 映射到自己的地址空间

## 相关系统调用

```rust
/// 发送消息（非阻塞）
fn aos_ipc_send(receiver: AgentId, msg_type: MessageType,
                request_id: u64, payload: *const u8,
                payload_len: u32) -> Result<()>;

/// 接收消息
/// flags: BLOCK（阻塞等待）/ NONBLOCK（立即返回）
fn aos_ipc_recv(buf: *mut Message, flags: RecvFlags) -> Result<()>;

/// 回复请求（语法糖：自动设置 msg_type=Response 和对应 request_id）
fn aos_ipc_reply(original_request_id: u64, payload: *const u8,
                 payload_len: u32) -> Result<()>;
```
