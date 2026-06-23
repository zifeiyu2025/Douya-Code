# 豆芽Code (douya-code)

> 基于 Rust 的本地 AI 编程 CLI，连接豆芽桌面版的 OpenAI 兼容 API，在终端中提供 Agentic（自主行动）编程能力。

## 项目简介

豆芽Code 是一个类似 Claude Code 和 OpenAI Codex CLI 的终端 AI 编程助手。它通过连接[豆芽桌面版](https://github.com/zifeiyu2025/douya)暴露的 OpenAI 兼容 API，在终端中提供：

- **Agentic 编程**：AI 自主规划任务、读写文件、执行命令、搜索代码
- **流式对话**：实时显示 AI 的思考和回答过程
- **工具系统**：Read/Write/Edit/Bash/Glob/Grep/LS/TodoWrite 等 8 个核心工具
- **富文本 TUI**：Markdown 渲染、语法高亮、思考过程折叠
- **安全机制**：破坏性命令确认、项目外文件拒绝、先读后改

## 技术栈

| 组件 | 技术 | 说明 |
|------|------|------|
| 语言 | Rust | 跨平台、内存安全、单文件二进制分发 |
| 异步运行时 | tokio | 异步 IO、并发任务 |
| HTTP 客户端 | reqwest + reqwest-eventsource | 连接豆芽 API、SSE 流式 |
| TUI 界面 | ratatui + crossterm + reedline | 富文本终端界面 |
| 语法高亮 | syntect | 代码块语法高亮 |
| CLI 参数 | clap | 命令行参数解析 |

## 架构设计

```
用户输入
   ↓
构建请求（系统提示词 + 对话历史 + 工具定义）
   ↓
发送流式请求到豆芽 API（/v1/chat/completions, stream=true）
   ↓
边接收边显示（思考过程 + 正文 + 工具调用预览）
   ↓
收到完整响应（StreamAccumulator 累积完毕）
   ↓
   ├── 有 tool_calls? → 执行工具 → 结果加入对话历史 → 回到"构建请求"（循环）
   │
   └── 无 tool_calls? → 显示最终回答 → 等待用户下一条输入
```

## 前置要求

1. **豆芽桌面版**已运行且模型已加载（提供 API 服务）
2. **Rust 工具链**（`rustup`、`cargo`）
3. 默认连接地址 `http://127.0.0.1:8080/v1`（同机部署）

## 快速开始

```bash
# 克隆仓库
git clone https://github.com/zifeiyu2025/Douya-Code.git
cd Douya-Code

# 编译
cargo build --release

# 运行（需要豆芽桌面版在后台运行）
cargo run --release
```

## 项目结构

```
Douya-Code/
├── src/
│   ├── main.rs             # 程序入口
│   ├── lib.rs              # 库入口
│   ├── config/             # 配置管理（命令行参数 + 配置文件）
│   ├── api/                # 豆芽 API 客户端（HTTP + SSE 流式）
│   ├── agent/              # 智能体核心（Agentic 主循环 + 提示词）
│   ├── tools/              # 工具系统（8 个核心工具）
│   ├── tui/                # 终端富文本界面
│   └── utils/              # 工具函数（安全检查、UTF-8 修复等）
└── tests/                  # 集成测试
```

## 开发路线

- [x] 阶段零：项目骨架和初始化
- [ ] 阶段一：API 连接和流式对话
- [ ] 阶段二：工具系统骨架（Read/Write/LS）
- [ ] 阶段三：完整工具集（Edit/Bash/Glob/Grep/TodoWrite）
- [ ] 阶段四：系统提示词和智能行为
- [ ] 阶段五：TUI 富文本界面
- [ ] 阶段六：打磨和优化

## 许可证

MIT