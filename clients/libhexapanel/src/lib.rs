use image::DynamicImage;
use image::GenericImageView;
use std::fs::File;
use std::io::prelude::*;
use std::net::ToSocketAddrs;
use std::net::UdpSocket;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

enum Command {
    NOOP = 0,
    RESET = 1 << 0,
    CLRSCN = 1 << 1,
    SHOW = 1 << 2,
}

pub trait SendPayload {
    const WINDOW_SIZE: Option<usize>;
    fn send_payload(&mut self, payload: &[u8]) -> Result<()>;
}

pub struct HexaPanel<T>(T);

impl HexaPanel<UdpSocket> {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(addr)?;
        Ok(Self(socket))
    }
}

impl SendPayload for UdpSocket {
    const WINDOW_SIZE: Option<usize> = Some(512);
    fn send_payload(&mut self, payload: &[u8]) -> Result<()> {
        self.send(payload)?;
        Ok(())
    }
}

impl HexaPanel<File> {
    pub fn connect<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let file = File::create(path.into())?;
        Ok(Self(file))
    }
}

impl SendPayload for File {
    const WINDOW_SIZE: Option<usize> = None;
    fn send_payload(&mut self, payload: &[u8]) -> Result<()> {
        self.write_all(payload)?;
        Ok(())
    }
}

impl<T> HexaPanel<T>
where
    T: SendPayload,
{
    pub fn send_frame(&mut self, img: DynamicImage) -> Result<()> {
        let dimensions = img.dimensions();
        if dimensions != (23, 18) {
            Err("Wrong dimensions!")?;
        }
        let mut payload = vec![Command::RESET as u8 | Command::CLRSCN as u8];

        for y in 0..dimensions.1 {
            for x in 0..dimensions.0 {
                if let Some(window_size) = T::WINDOW_SIZE {
                    if payload.len() + 3 > window_size {
                        self.send_payload(&payload)?;
                        payload = vec![Command::NOOP as u8];
                    }
                }
                payload.extend_from_slice(&img.get_pixel(x, y).0[0..3]);
            }
        }
        payload[0] |= Command::SHOW as u8;
        self.send_payload(&payload)?;
        Ok(())
    }

    fn send_payload(&mut self, payload: &[u8]) -> Result<()> {
        &self.0.send_payload(payload)?;
        Ok(())
    }
}
