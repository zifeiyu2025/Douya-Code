//! 豆芽Code (douya-code) 库入口
//!
//! 这是整个程序的库根，汇总所有功能模块。
//! 模块划分参考 Claude Code 架构：
//! - config: 配置管理（命令行参数 + 配置文件）
//! - api: 豆芽 API 客户端（HTTP + SSE 流式）
//! - agent: 智能体核心（Agentic 主循环 + 提示词构建）
//! - tools: 工具系统（Read/Write/Edit/Bash/Glob/Grep/LS/TodoWrite）
//! - tui: 终端富文本界面（ratatui + crossterm + reedline）
//! - utils: 工具函数（文件系统、安全检查、UTF-8 修复）

pub mod agent;
pub mod api;
pub mod config;
pub mod tools;
pub mod tui;
pub mod utils;