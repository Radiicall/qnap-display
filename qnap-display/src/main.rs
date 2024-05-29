use std::{process::Command, thread::sleep, time::Duration};
use sysinfo::System;

fn main() {
    let mut lcd = a125::LCD::new().unwrap();
    let mut sys = System::new_all();

    lcd.backlight(true).unwrap();

    loop {
        for _ in 0..15 {
            sys.refresh_cpu();
            let mut cpu_usage: f32 = 0.0;
            for cpu in sys.cpus() {
                cpu_usage += cpu.cpu_usage();
            }
            cpu_usage /= sys.cpus().len() as f32;
    
            lcd.write_l1(format!("CPU: {:.1}%", cpu_usage)).unwrap();

            sleep(Duration::from_secs(2))
        }

        for _ in 0..15 {
            sys.refresh_memory();

            let used_mem = ((sys.used_memory() as f64 / 1024.0) / 1024.0) / 1024.0;
            let total_mem = ((sys.total_memory() as f64 / 1024.0) / 1024.0) / 1024.0;

            lcd.write_l1(format!("MEM: {:.1}/{:.1}GB", used_mem, total_mem)).unwrap();
            sleep(Duration::from_secs(2));
        }

        for _ in 0..15 {
            match Command::new("zfs").args(["list", "-H", "-p"]).output() {
                Ok(output) => {
                    let output = String::from_utf8_lossy(&output.stdout).to_string();
                    let output = output.split("\n").nth(0).unwrap();
                    let mut output = output.split_whitespace();

                    let used = (((output.nth(1).unwrap().parse::<u64>().unwrap() as f64 / 1024.0) / 1024.0) / 1024.0) / 1024.0;
                    let avail = (((output.nth(0).unwrap().parse::<u64>().unwrap() as f64 / 1024.0) / 1024.0) / 1024.0) / 1024.0;

                    let total = used + avail;

                    lcd.write_l1(format!("STR: {:.1}/{:.1}TB", used, total)).unwrap();
                },
                Err(_) => (),
            }

            sleep(Duration::from_secs(2))
        }
    }
}
