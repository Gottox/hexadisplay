use image::DynamicImage;
use image::GenericImageView;
use raqote::DrawTarget;
use std::convert::TryInto;
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

pub trait ImageSource {
    fn size(&self) -> (u32, u32);
    fn rgb(&self) -> Vec<u8>;
}

impl ImageSource for DynamicImage {
    fn size(&self) -> (u32, u32) {
        self.dimensions()
    }

    fn rgb(&self) -> Vec<u8> {
        let mut pixels = vec![];
        let dimensions = self.size();
        for y in 0..dimensions.1 {
            for x in 0..dimensions.0 {
                pixels.extend_from_slice(&self.get_pixel(x, y).0[0..3]);
            }
        }
        pixels
    }
}

impl ImageSource for &DrawTarget {
    fn size(&self) -> (u32, u32) {
        (
            self.width().try_into().unwrap(),
            self.height().try_into().unwrap(),
        )
    }
    fn rgb(&self) -> Vec<u8> {
        let mut pixels = vec![];
        for pixel in self.get_data() {
            pixels.extend_from_slice(&pixel.to_be_bytes()[1..4]);
        }
        pixels
    }
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
    pub fn aligned_size() -> Option<usize> {
        let mut size = T::WINDOW_SIZE? - 1;
        size -= size % 3;
        Some(size + 1)
    }

    pub fn send_frame<I>(&mut self, img: I) -> Result<()>
    where
        I: ImageSource,
    {
        let dimensions = img.size();

        if dimensions != (23, 18) {
            Err("Wrong dimensions!")?;
        }

        let mut payload = vec![Command::RESET as u8 | Command::CLRSCN as u8];

        let mut pixels = img.rgb();
        let window_size = Self::aligned_size().unwrap_or(pixels.len() + payload.len());

        loop {
            let back = pixels.split_off(pixels.len().min(window_size - payload.len()));
            payload.extend_from_slice(&pixels);
            pixels = back;

            if pixels.is_empty() {
                payload[0] |= Command::SHOW as u8;
                self.send_payload(&payload)?;
                break;
            } else {
                self.send_payload(&payload)?;
                payload = vec![Command::NOOP as u8];
            }
        }

        Ok(())
    }

    fn send_payload(&mut self, payload: &[u8]) -> Result<()> {
        &self.0.send_payload(payload)?;
        Ok(())
    }
}
