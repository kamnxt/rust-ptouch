use ptouch::device::*;
use ptouch::{
    self,
    device::{DeviceStatus, Media, MediaKind, PrintInfo},
    render::{ops, Render, RenderConfig},
    PTouch,
};
use simplelog::{ColorChoice, LevelFilter, TermLogger, TerminalMode};

use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    TermLogger::init(
        LevelFilter::Debug,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let opts = ptouch::Options::from_args();
    let mut pt = PTouch::new(&opts);
    let (status, mut pt) = if let Ok(pt) = pt.as_mut() {
        (pt.status().unwrap(), Some(pt))
    } else {
        (
            Status {
                error1: Error1::empty(),
                error2: Error2::empty(),
                media_kind: MediaKind::LaminatedTape,
                media_width: 12,
                status_type: DeviceStatus::Unknown,
                tape_colour: TapeColour::White,
                model: 0,
                phase: Phase::Printing,
                text_colour: TextColour::Black,
            },
            None,
        )
    };
    println!("Status: {:?}", status);

    let media = Media::from((status.media_kind, status.media_width));

    let ops = [ops::Op::text("good")];
    let rc = RenderConfig {
        y: media.area().1 as usize,
        ..Default::default()
    };
    let mut render = Render::new(rc);
    render.render(&ops).unwrap();
    let data = render.raster(media.area()).unwrap();
    println!("{:?}", data);
    render.show().unwrap();
    let info = PrintInfo {
        width: Some(status.media_width),
        length: Some(0),
        raster_no: data.len() as u32,
        recover: false,
        ..Default::default()
    };
    if let Some(pt) = pt.as_mut() {
        pt.print_raw(data, &info, false)?;
    }
    let ops = [ops::Op::text("morning")];
    let rc = RenderConfig {
        y: media.area().1 as usize,
        ..Default::default()
    };
    let mut render = Render::new(rc);
    render.render(&ops).unwrap();
    let data = render.raster(media.area()).unwrap();
    println!("{:?}", data);
    render.show().unwrap();
    let info = PrintInfo {
        width: Some(status.media_width),
        length: Some(0),
        raster_no: data.len() as u32,
        otherpage: true,
        recover: false,
        ..Default::default()
    };
    if let Some(pt) = pt.as_mut() {
        pt.print_raw(data, &info, false)?;
    }
    let ops = [ops::Op::text("world!")];
    let rc = RenderConfig {
        y: media.area().1 as usize,
        ..Default::default()
    };
    let mut render = Render::new(rc);
    render.render(&ops).unwrap();
    let data = render.raster(media.area()).unwrap();
    println!("{:?}", data);
    render.show().unwrap();
    let info = PrintInfo {
        width: Some(status.media_width),
        length: Some(0),
        raster_no: data.len() as u32,
        otherpage: true,
        recover: false,
        ..Default::default()
    };
    if let Some(pt) = pt.as_mut() {
        pt.print_raw(data, &info, true)?;
    }
    Ok(())
}
