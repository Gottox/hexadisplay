use std::path::PathBuf;
use std::net::SocketAddr;
use std::net::UdpSocket;
use libhexapanel::HexaPanel;
use image::imageops::FilterType;
use std::time::Duration;
use structopt::*;

static IMAGE_SIZE: (u32, u32) = (23, 18);

#[derive(StructOpt, Debug)]
#[structopt(name = "send_image")]
struct Opt {
    #[structopt(short, long, default_value = "100")]
    /// interval in milliseconds
    interval: u64,

    #[structopt(short, long)]
    /// send files continuesly
    _loop: bool,

    #[structopt(name = "HOST:PORT")]
    addr: SocketAddr,

    #[structopt(name = "FILE")]
    files: Vec<PathBuf>,
}

fn main() -> libhexapanel::Result<()> {
    let opt = Opt::from_args();

    let mut hexapanel = HexaPanel::<UdpSocket>::connect(opt.addr)?;
    loop {
        let mut iter = opt.files.iter().peekable();
        while let Some(path) = iter.next() {
            let img = image::open(path)?
                .resize_exact(IMAGE_SIZE.0, IMAGE_SIZE.1, FilterType::Gaussian);

            //let mut hexapanel = HexaPanel::<std::fs::File>::connect(addr)?;
            hexapanel.send_frame(img)?;
            if iter.peek().is_some() {
                std::thread::sleep(Duration::from_millis(opt.interval));
            }
        }
        if !opt._loop {
            break;
        }
    }

    Ok(())
}
