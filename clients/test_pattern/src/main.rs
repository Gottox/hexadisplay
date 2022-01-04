use libhexapanel::HexaPanel;
use raqote::DrawTarget;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::path::PathBuf;
use std::time::Duration;
use structopt::*;

static SIZE: (i32, i32) = (23, 18);

#[derive(Debug, enum_utils::FromStr, derive_more::Display)]
enum Pattern {
    Chaser,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "send_image")]
struct Opt {
    #[structopt(short, long, default_value = "100")]
    /// interval in milliseconds
    interval: u64,

    //#[structopt(short, long)]
    //pattern: Pattern,
    #[structopt(short, long)]
    /// send files continuesly
    _loop: bool,

    #[structopt(name = "HOST:PORT")]
    addr: SocketAddr,
}

fn draw_color(iteration: u64) -> DrawTarget {
    let mut dt = DrawTarget::new(SIZE.0, SIZE.1);
    for pixel in dt.get_data_mut().iter_mut() {
        match iteration % 3 {
            0 => *pixel = 0xFF040000,
            1 => *pixel = 0xFF000400,
            2 => *pixel = 0xFF000004,
            _ => *pixel = 0xFFFFFFFF,
        }
    }
    dt
}
fn draw_chaser(iteration: u64) -> DrawTarget {
    let mut dt = DrawTarget::new(SIZE.0, SIZE.1);
    for (i, pixel) in dt.get_data_mut().iter_mut().enumerate() {
        if iteration % ((SIZE.0 * SIZE.1) as u64) == i.try_into().unwrap() {
            *pixel = 0xFFFFFFFF;
        } else {
            *pixel = 0x00000000;
        }
    }
    dt
}

fn main() -> libhexapanel::Result<()> {
    let opt = Opt::from_args();

    let mut hexapanel = HexaPanel::<UdpSocket>::connect(opt.addr)?;
    let mut iteration = 0;
    loop {
        let mut dt = draw_chaser(iteration);

        iteration += 1;
        hexapanel.send_frame(&dt)?;
        std::thread::sleep(Duration::from_millis(opt.interval));
    }
}
