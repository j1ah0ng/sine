use std::env::args;
use std::f64::consts::PI as PI;
use std::thread::sleep;
use std::time::Duration;

fn min(v1: f64, v2: f64) -> f64 {
    if v1 <= v2 {
        v1
    } else {
        v2
    }
}

fn max(v1: f64, v2: f64) -> f64 {
    if v1 >= v2 {
        v1
    } else {
        v2
    }
}

fn get_term_size() -> Option<(f64, f64)> {
    use libc::ioctl;
    use libc::isatty;
    use libc::{winsize as WinSize, TIOCGWINSZ};

    // check that we currently are in a tty
    let is_tty: bool = unsafe { isatty(libc::STDOUT_FILENO) == 1 };

    // avoid weird fds
    if !is_tty { 
        eprintln!("Not a TTY");
        return None;
    }

    // init winsize struct as mut
    let mut winsize = WinSize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    // avoid ioctl fucky wucky
    if unsafe { ioctl(libc::STDOUT_FILENO, TIOCGWINSZ, &mut winsize) } != 0 {
        eprintln!("ioctl tty failed");
        return None;
    }

    if winsize.ws_row != 0 && winsize.ws_col != 0 {
        Some((winsize.ws_row as f64, winsize.ws_col as f64))
    } else {
        eprintln!("ws was zero");
        None
    }
}

fn main() {

    // get args and make sure all args are present
    let argv: Vec<String> = args().collect();
    if argv.len() < 4 {
        eprintln!("Usage: {} [duration] [step] [time]", argv[0]);
        return;
    }

    // collect args
    let duration: i32   = argv[1].parse::<i32>().unwrap();
    // let step: i32       = argv[2].parse::<i32>().unwrap();
    let time: f64       = argv[3].parse::<f64>().unwrap();
    // let is_pos: bool    = true;
    let mut lowerbound: i32 =  0;
    let mut output: String = String::from("");

    // get terminal size
    let size: Option<(f64, f64)> = get_term_size();
    if size == None { 
        eprintln!("Your tty is broken ono");
        return;
    }
    let ws: (f64, f64) = size.unwrap();

    // out
    for i in 0..duration {
        for j in 0..ws.1 as i32 {

            if ((i as f64) * PI / 180.0).sin() > 0.0 { 
                lowerbound = max(ws.1 / 2.0, 
                                 (ws.1 / 2.0) * ((i as f64) / 180.0 * PI).sin())
                             .floor() as i32;
            }

            if ((i as f64) * PI / 180.0).sin() <= 0.0 { 
                lowerbound = min(ws.1 / 2.0, 
                                 (ws.1 / 2.0) + (ws.1 / 2.0) * ((i as f64) / 180.0 * PI).sin())
                             .floor() as i32;
            }

            if j > ((ws.1 / 2.0) + (ws.1 / 2.0) * ((i as f64) / 180.0 * PI).sin()).floor() as i32 || j < lowerbound {
                output.push_str(" ");
            } else {
                output.push_str("#");
            }
        }
        println!("{}", output);
        sleep(Duration::from_millis((time * 1000.0) as u64));
        output = String::from("");
    }
}
