use std::time::Duration;

pub struct LCD {
    port: Box<dyn serialport::SerialPort>,
}

impl LCD {
    pub fn new() -> Result<Self, serialport::Error> {
        Ok(LCD {
            port: serialport::new("/dev/ttyS1", 1_200)
                .timeout(Duration::from_millis(10))
                .open()?,
        })
    }

    pub fn backlight(&mut self, val: bool) -> Result<(), std::io::Error> {
        if val {
            self.port.write(&[0x4D, 0x5E, 0x01])?;
        } else {
            self.port.write(&[0x4D, 0x5E, 0x00])?;
        }
        self.port.flush()?;
        Ok(())
    }

    pub fn write_l1(&mut self, msg: String) -> Result<(), std::io::Error> {
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
    pub fn write_l2(&mut self, msg: String) -> Result<(), std::io::Error> {
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
