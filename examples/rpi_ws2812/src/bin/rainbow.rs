use arrayvec::ArrayVec;
use rainbow_emanator::rainbow_emanator;
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::io::Write;

fn main() {
    let mut spi = Spidev::open("/dev/spidev0.0").unwrap();
    let options = SpidevOptions::new()
        .bits_per_word(8)
        // According to https://www.raspberrypi.org/documentation/hardware/raspberrypi/spi/README.md
        .max_speed_hz(15_600_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options).unwrap();

    let start = std::time::Instant::now();
    let mut spi_data = vec![];

    loop {
        spi_data.clear();
        let elapsed = start.elapsed().as_millis() as f32;
        spi_data.extend(rainbow_emanator(8, 5, elapsed).flat_map(encode_rgb_array));
        spi.write_all(&spi_data).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10))
    }
}

fn encode_rgb_array([r, g, b]: [u8; 3]) -> impl Iterator<Item = u8> {
    ArrayVec::from([g, r, b])
        .into_iter()
        .flat_map(|component_intensity| {
            (0..8).rev().flat_map(move |bits| {
                ArrayVec::from(if component_intensity >> bits & 1 == 0 {
                    [0b1111_1000, 0b0000_0000]
                } else {
                    [0b1111_1111, 0b1000_0000]
                })
            })
        })
}
