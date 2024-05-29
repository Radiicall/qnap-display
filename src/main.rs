use std::{process::Command, thread::sleep, time::Duration};
use sysinfo::System;

fn main() {
    let mut lcd = LCD::new().unwrap();
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

struct LCD {
    port: Box<dyn serialport::SerialPort>,
}

impl LCD {
    fn new() -> Result<Self, serialport::Error> {
        Ok(LCD {
            port: serialport::new("/dev/ttyS1", 1_200)
                .timeout(Duration::from_millis(10))
                .open()?,
        })
    }

    fn backlight(&mut self, val: bool) -> Result<(), std::io::Error> {
        if val {
            self.port.write(&[0x4D, 0x5E, 0x01])?;
        } else {
            self.port.write(&[0x4D, 0x5E, 0x00])?;
        }
        self.port.flush()?;
        Ok(())
    }

    fn write_l1(&mut self, msg: String) -> Result<(), std::io::Error> {
        if msg.chars().count() > 16 {
            println!("Message too long!");
            return Ok(())
        }

        println!("l1: {}", msg);

        let mut buf = vec![0x4D, 0x0C, 0x00, 0x20];

        for c in msg.chars() {
            buf.push(c as u8);
        }

        for _ in buf.len()..20 {
            buf.push(0x20);
        }

        self.port.write(&buf)?;
        self.port.flush()?;
        Ok(())
    }
    
    // L2 does not want to work properly, very annoying
    fn write_l2(&mut self, msg: String) -> Result<(), std::io::Error> {
        if msg.chars().count() > 16 {
            println!("Message too long!");
            return Ok(())
        }

        println!("l2: {}", msg);

        let mut buf = vec![0x4D, 0x0C, 0x01, 0x20];

        for c in msg.chars() {
            buf.push(c as u8);
        }
        
        for _ in buf.len()..20 {
            buf.push(0x20);
        }

        self.port.write(msg.as_bytes())?;
        self.port.flush()?;
        Ok(())
    }
    
}
