use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short = 'b', long = "backlight", help = "Backlight control")]
    backlight: Option<bool>,
    #[arg(short = 'l', long = "line-1", help = "What to display on line 1")]
    line_one: Option<String>,
    #[arg(short = 'L', long = "line-2", help = "What to display on line 2")]
    line_two: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut lcd = a125::LCD::new().unwrap();

    match args.backlight {
        Some(backlight) => lcd.backlight(backlight).expect("Can't set backlight"),
        None => (),
    }

    match args.line_one {
        Some(l1) => lcd.write_l1(l1).expect("Can't write to line 1"),
        None => (),
    }

    match args.line_two {
        Some(l2) => lcd.write_l2(l2).expect("Can't write to line 2"),
        None => (),
    }
}
