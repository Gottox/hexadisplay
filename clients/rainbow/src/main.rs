use std::net::UdpSocket;
use std::net::SocketAddr;
use std::time::Duration;
use libhexapanel::*;
use raqote::*;
use std::f32::consts::PI;
use structopt::*;

const SIZE: (i32, i32) = (23, 18);

#[derive(StructOpt, Debug)]
#[structopt(name = "rainbow")]
struct Opt {
    #[structopt(short, long, default_value = "360")]
    /// interval in milliseconds
    steps: u64,

    #[structopt(short, long, default_value = "100")]
    /// interval in milliseconds
    interval: u64,

    #[structopt(name = "HOST:PORT")]
    addr: SocketAddr,
}


fn rotate(angle: f32, p: Point, cot: Point) -> Point {
    let angle = 2. * PI * angle;
    let p = p - cot;
    Point::new(
        p.x * angle.cos() - p.y * angle.sin() + cot.x,
        p.x * angle.sin() + p.y * angle.cos() + cot.y,
    )
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut dt = DrawTarget::new(SIZE.0, SIZE.1);

    let mut hexapanel = HexaPanel::<UdpSocket>::connect(opt.addr)?;
    for i in std::iter::repeat(0..opt.steps).flatten() {
        let angle = i as f32 / opt.steps as f32;
        draw_gradient(angle, &mut dt);
        hexapanel.send_frame(&dt)?;
        std::thread::sleep(Duration::from_millis(opt.interval));
    }

    unreachable!();
}

fn draw_gradient(angle: f32, dt: &mut DrawTarget) {
    let mut pb = PathBuilder::new();
    pb.rect(0., 0., SIZE.0 as f32, SIZE.1 as f32);
    pb.close();
    let path = pb.finish();

    let center_of_rotation = Point::new(SIZE.0 as f32 / 2., SIZE.1 as f32 / 2.);

    let gradient = Source::new_linear_gradient(
        Gradient {
            stops: vec![
                GradientStop {
                    position: 0. / 6.,
                    color: Color::new(0xff, 0xff, 0, 0),
                },
                GradientStop {
                    position: 1. / 6.,
                    color: Color::new(0xff, 0xff, 0xff, 0),
                },
                GradientStop {
                    position: 2. / 6.,
                    color: Color::new(0xff, 0, 0xff, 0),
                },
                GradientStop {
                    position: 3. / 6.,
                    color: Color::new(0xff, 0, 0xff, 0xff),
                },
                GradientStop {
                    position: 4. / 6.,
                    color: Color::new(0xff, 0, 0, 0xff),
                },
                GradientStop {
                    position: 5. / 6.,
                    color: Color::new(0xff, 0xff, 0, 0xff),
                },
                GradientStop {
                    position: 6. / 6.,
                    color: Color::new(0xff, 0xff, 0, 0),
                },
            ],
        },
        rotate(angle, Point::new(0., 0.), center_of_rotation),
        rotate(
            angle,
            Point::new(0., SIZE.1 as f32),
            center_of_rotation,
        ),
        Spread::Repeat,
    );
    dt.fill(&path, &gradient, &DrawOptions::new());

}
