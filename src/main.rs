//! 豆芽Code 程序入口
//!
//! 职责：
//! 1. 初始化日志（tracing）
//! 2. 解析命令行参数
//! 3. 启动 tokio 异步运行时
//! 4. 进入主交互循环
//!
//! 当前为阶段零骨架，后续阶段会逐步填充实际功能。

use anyhow::Result;

fn main() -> Result<()> {
    println!("豆芽Code v{}  —  基于 Rust 的本地 AI 编程 CLI", env!("CARGO_PKG_VERSION"));
    println!("正在启动...（阶段零：项目骨架）");

    // 阶段零：仅验证项目能编译运行
    // 后续阶段会在这里：
    // 1. 解析命令行参数（clap）
    // 2. 加载配置文件
    // 3. 启动 tokio 运行时
    // 4. 连接豆芽 API
    // 5. 进入 TUI 交互界面

    Ok(())
}