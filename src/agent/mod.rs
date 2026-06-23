//! 智能体核心模块
//!
//! 实现 Agentic 主循环：用户输入 -> AI 决策 -> 工具执行 -> 结果反馈 -> 循环。

pub mod agent_loop;
pub mod conversation;
pub mod prompt;
pub mod tool_handler;