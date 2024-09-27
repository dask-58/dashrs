use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    cursor,
};
use indicatif::{ProgressBar, ProgressStyle};
use sysinfo::{System, SystemExt, CpuExt, DiskExt};
use std::io::{stdout, Write};
use std::time::Duration;

fn main() -> crossterm::Result<()> {
    let mut stdout = stdout();
    let mut sys = System::new_all();
    loop {
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        sys.refresh_all();
        // CPU 
        let cpu_usage = sys.global_cpu_info().cpu_usage();
        _print("CPU Usage", cpu_usage, Color::Green)?;
        // Memory
        let mem_usage = sys.used_memory() as f32 / sys.total_memory() as f32 * 100.0;
        _print("Memory Usage", mem_usage, Color::Yellow)?;
        // Disk
        if let Some(disk) = sys.disks().first() {
            let disk_usage = (1.0 - (disk.available_space() as f32 / disk.total_space() as f32)) * 100.0;
            _print("Disk Usage", disk_usage, Color::Cyan)?;
        }
        stdout.flush()?;
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn _print(label: &str, usage: f32, color: Color) -> crossterm::Result<()> {
    let mut stdout = stdout();
    execute!(
        stdout,
        SetForegroundColor(color),
        Print(format!("{}: ", label)),
        ResetColor
    )?;
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::default_bar()
        .template("{bar:40.cyan/blue} {pos:>3}%")
        .unwrap()
        .progress_chars("##-"));
    pb.set_position(usage as u64);
    println!();
    Ok(())
}
