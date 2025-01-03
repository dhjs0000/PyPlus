use pyo3::prelude::*;
use std::io::{self, Write};

#[pyclass]
struct RustProgressBar {
    total: i64,
    iteration: i64,
    last_percent: i32,
    update_step: i64,
    bar_length: i32,
    bar_cache: Vec<String>,
}

#[pymethods]
impl RustProgressBar {
    #[new]
    fn new(total: i64) -> Self {
        let mut bar_cache = Vec::with_capacity(101);
        let bar_length = 50;
        
        // 预计算所有可能的进度条字符串
        for percent in 0..=100 {
            let filled = (bar_length * percent) / 100;
            let empty = bar_length - filled;
            let bar = "█".repeat(filled as usize) + &"░".repeat(empty as usize);
            bar_cache.push(bar);
        }

        RustProgressBar {
            total,
            iteration: 0,
            last_percent: -1,
            update_step: std::cmp::max(1, total / 50), // 每2%更新一次
            bar_length: 50,
            bar_cache,
        }
    }

    fn update(&mut self, n: Option<i64>) {
        let step = n.unwrap_or(1);
        self.iteration = std::cmp::min(self.iteration + step, self.total);
        self.print();
    }

    fn finish(&mut self) {
        self.iteration = self.total;
        self.print();
    }

    fn print(&mut self) {
        // 只在达到更新步长时更新显示
        if self.iteration % self.update_step != 0 && self.iteration != self.total {
            return;
        }

        let percent = ((self.iteration as f64 / self.total as f64) * 100.0) as i32;
        let percent = std::cmp::min(100, percent);

        // 只在百分比变化时更新显示
        if percent == self.last_percent && self.iteration != self.total {
            return;
        }

        self.last_percent = percent;

        // 使用预计算的进度条字符串
        let bar = &self.bar_cache[percent as usize];
        
        // 直接写入标准输出，避免 Python 的 print 开销
        let output = format!("\r|{}|{:3}%", bar, percent);
        io::stdout().write_all(output.as_bytes()).unwrap();
        io::stdout().flush().unwrap();

        if self.iteration == self.total {
            println!();
        }
    }
}

/// Rust 实现的超高速进度条模块
#[pymodule]
fn rust_progressbar(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RustProgressBar>()?;
    Ok(())
} 