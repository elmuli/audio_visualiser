use alsa::{Direction, ValueOr};
use alsa::pcm::{PCM, HwParams, Format, Access};

use crate::app;

pub fn read_audio(app: &mut app::App){
    let pcm = PCM::new("default", Direction::Capture, false).unwrap();

    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(1).unwrap();
    hwp.set_rate(44100, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::s16()).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();

    let io = pcm.io_i16().unwrap();

    for _ in 0..2 * 4410/1024{
        io.readi(&mut app.buf[..]).unwrap();
        print!("buf {:?}", app.buf);
    }
}
