Language: [🇨🇳](README.md)

# AOS – Agent Operating System

> Agent as Process. Inference as Computation. Memory as Filesystem.

---

## Overview

**AOS is a standalone operating system kernel built from scratch in Rust, with the AI Agent as its first-class citizen.**

AOS is not middleware running on top of Linux/macOS/Windows. It is a fourth category of operating system — designed for autonomous AI Agent execution, not for manual human operation.

---

## Design Philosophy

Traditional operating systems assume "a human sits in front of the machine." AOS assumes: **the executor is an AI Agent, not a human-written program.**

| Traditional OS | AOS |
|----------------|-----|
| Core abstraction: Process + File | Core abstraction: Agent + Memory |
| Scheduled resource: CPU time slices | Scheduled resource: Inference compute + Token budget |
| Security model: User/Group/Permission bits | Security model: Capability-based |
| Interaction: Human → Program → Result | Interaction: Human → Intent → Agent autonomous execution |

---

## System Architecture

AOS uses a **microkernel architecture**. The kernel does the bare minimum; the inference engine, drivers, and all services run in userspace.

```
┌─────────────────────────────────────────────┐
│                User Layer                    │
│  PersonalAgent (user's proxy identity)       │
│  Application Agents · Tool Agents            │
├─────────────────────────────────────────────┤
│             Userspace Services               │
│  Inference Engine · Memory Filesystem        │
│  Device Drivers · AgentPack Module System    │
├─────────────────────────────────────────────┤
│             AOS Microkernel (Rust)            │
│  Agent Lifecycle · IPC · Capability Control   │
│  Resource Budget · Interrupts · Minimal MM    │
├─────────────────────────────────────────────┤
│                Hardware                      │
│  CPU · GPU/NPU · Sensors · Actuators         │
└─────────────────────────────────────────────┘
```

### Microkernel Responsibilities

The kernel handles only mechanisms, never policy:

- **Agent lifecycle management** — create / destroy / suspend / resume
- **IPC (Inter-Process Communication)** — the sole channel for Agent-to-Agent messaging
- **Capability management** — kernel-enforced access control over Tools
- **Resource budget enforcement** — hard limits on compute / memory / token quotas
- **Interrupt & exception handling** — basic hardware abstraction
- **Minimal memory management** — page tables, address space isolation

The inference engine, device drivers, and filesystems all run in userspace. If the inference service crashes, the kernel safely restarts it without bringing down the system.

### Agent Primitive

The Agent is AOS's first-class citizen, analogous to a process in traditional operating systems:

```rust
struct Agent {
    identity: AgentId,          // Identity and permissions
    model_context: ModelRef,    // Model state reference
    memory: MemoryHandle,       // Persistent memory (short-term + long-term)
    tools: CapabilitySet,       // Callable system capabilities
    goal: Goal,                 // Current objective (replaces program counter)
    budget: ResourceBudget,     // Compute / token / time budget
}
```

### Inference Architecture

AOS fully decouples the inference engine from the kernel, supporting flexible backends:

- **Local inference** — access GPU/NPU via userspace drivers, run quantized models
- **Remote inference** — offload inference requests to cloud APIs
- **Hybrid inference** — local small models for fast decisions, complex tasks offloaded remotely

The kernel does not care where inference happens. It only manages Agent lifecycle and resource budgets.

---

## Core Capabilities

### Autonomous Agent Collaboration

Multiple Agents communicate and collaborate directly through kernel IPC, with no human intermediary:

```
User: "Organize my experiment data and generate a report"

Planner Agent → decompose task
  ├→ DataSci Agent → analyze data, generate statistics
  ├→ Writer Agent  → draft text, organize structure
  └→ Critic Agent  → review consistency and accuracy
```

### Agent as User

A PersonalAgent is the user's persistent proxy identity within AOS:
- Understands user preferences, habits, and goals
- Interacts with other Agents on the user's behalf
- Continues working while the user is away
- Multiple PersonalAgents can collaborate, enabling organization-level coordination

### Dynamic Capability Synthesis

When the system lacks a required capability, it can dynamically pull AgentPack modules from AOS Hub and compose them into a new Agent on-demand. Like dynamic linking, but linking capabilities instead of functions.

### Security & Isolation

- **Capability-based security** — Agents can only use explicitly granted Tools
- **Resource budget hard limits** — prevents Agents from consuming unbounded compute
- **Safety Agent** — a privileged supervisor Agent that can override any other Agent's decisions
- **Kernel-level address space isolation** — a single Agent crash does not affect the system

---

## Target Hardware

The AOS microkernel itself is extremely lightweight. Hardware requirements depend on the userspace inference service configuration:

| Scenario | Minimum Requirements |
|----------|---------------------|
| Development (QEMU) | x86-64 / ARM64, 4 cores, 32GB RAM, 8GB VRAM GPU |
| Edge / Embedded | ARM / RISC-V + dedicated NPU, 512MB–2GB RAM |
| Robotics | ARM64 + NPU, 8GB+ RAM, real-time sensor interfaces |

---

## Use Cases

### Autonomous Software Development

Agent teams autonomously complete the full development lifecycle from requirements analysis to code review.

### Robotic Autonomous Control

AOS is a natural fit for robotics — the Agent-as-process design enables perception, planning, motion control, and safety supervision modules to run as independent Agents, collaborating via kernel IPC at microsecond latency without middleware layers like ROS.

### Self-Adaptive System Management

The system self-monitors and self-maintains without human administration.

### Multimodal Creative Collaboration

Multiple specialized Agents collaborate on cross-domain creative tasks (data analysis + writing + visualization).

---

## Technology Stack

- **Kernel language**: Rust (memory safety, zero-cost abstractions, concurrency safety)
- **Target architectures**: x86-64 / ARM64 (initial), RISC-V (planned)
- **Boot**: UEFI
- **Development environment**: QEMU/KVM virtual machines

---

## Design References

| Project | Reference Value |
|---------|----------------|
| [Redox OS](https://www.redox-os.org/) | Pioneer Rust microkernel OS implementation |
| [Asterinas](https://github.com/asterinas/asterinas) | Production-grade Linux alternative written in Rust |
| [seL4](https://sel4.systems/) | Formally verified microkernel, capability security model reference |
| [OpenClaw](https://github.com/openclaw/openclaw) | Application-layer Agent collaboration framework, validates the viability of autonomous Agent cooperation |

---

## Project Status

> 🚧 Early design phase — discussion and contributions welcome

---

## Naming

- **Full name**: Agent Operating System
- **Abbreviation**: AOS
- **Chinese name**: 智核 (Zhì Hé) — "Cognitive Core"
