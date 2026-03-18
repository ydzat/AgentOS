# Licensing / 许可声明

AOS (AgentOS / 智核) 采用分层许可模型，内核与用户态服务使用不同的许可证。

AOS uses a layered licensing model. The kernel and userspace services are licensed under different terms.

## Kernel — GPL v2

The kernel components are licensed under the **GNU General Public License v2.0 only** (GPL-2.0-only).

内核组件使用 **GNU 通用公共许可证 v2.0** 授权。

This includes:
- `api/` — Shared interface layer (uabi types + port traits)
- `hal/` — Hardware abstraction layer
- `kernel/` — Kernel core
- `init/` — Boot entry point

See [LICENSE-GPL2](LICENSE-GPL2) for the full license text.

Any modifications to these components must be distributed under the same GPL v2.0 license.

对内核组件的任何修改都必须以相同的 GPL v2.0 许可证分发。

## Userspace Services — Apache 2.0

The userspace components are licensed under the **Apache License 2.0**.

用户态组件使用 **Apache 许可证 2.0** 授权。

This includes:
- `userspace/` — Userspace services (inference engine, memory filesystem, device manager, etc.)
- `examples/` — Example agents
- `tools/` — Development tools

See [LICENSE-APACHE2](LICENSE-APACHE2) for the full license text.

## AgentPack Ecosystem

AgentPacks distributed via AOS Hub may use any license chosen by their respective authors. AOS does not impose licensing requirements on third-party AgentPacks.

通过 AOS Hub 分发的 AgentPack 可以使用其作者选择的任何许可证。AOS 不对第三方 AgentPack 施加许可要求。

## Licensing Boundary / 许可边界

The syscall interface (AOS ABI) serves as the licensing boundary between kernel and userspace, consistent with how the Linux kernel treats the kernel/userspace boundary under GPL v2.

系统调用接口（AOS ABI）是内核与用户态之间的许可边界，与 Linux 内核在 GPL v2 下对内核/用户态边界的处理方式一致。

```
┌────────────────────────────┐
│ AgentPack (any license)    │
├────────────────────────────┤
│ Userspace (Apache 2.0)     │
├══════════════════════════════  ← Syscall ABI = licensing boundary
│ Kernel (GPL v2.0)          │
└────────────────────────────┘
```
