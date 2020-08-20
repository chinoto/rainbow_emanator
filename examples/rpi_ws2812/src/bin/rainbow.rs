use arrayvec::ArrayVec;
use rainbow_emanator::rainbow_emanator;
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::io::{self, Write};
use std::{thread, time};

fn main() {
    let mut color_writer = Ws281xColorWriter::new("/dev/spidev0.0").unwrap();
    let start = time::Instant::now();

    loop {
        let elapsed = start.elapsed().as_millis() as f32;
        color_writer
            .write_colors(rainbow_emanator(8, 5, elapsed))
            .unwrap();
        thread::sleep(time::Duration::from_millis(10))
    }
}

/// A wrapper over an `Spidev` to simplify writing RGB values to it.
/// Spidev requires the input to be a u8 slice, so `buffer` is used to avoid
/// allocating on every write.
struct Ws281xColorWriter {
    spi: Spidev,
    buffer: Vec<u8>,
}

impl Ws281xColorWriter {
    /// Opens the given path with `Spidev`, setting it to 15.6MHz, and returns
    /// Self wrapped in a Result.
    pub fn new(spidev_path: &str) -> io::Result<Self> {
        let mut spi = Spidev::open(spidev_path)?;
        let options = SpidevOptions::new()
            .bits_per_word(8)
            // According to https://www.raspberrypi.org/documentation/hardware/raspberrypi/spi/README.md
            .max_speed_hz(15_600_000)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&options)?;

        Ok(Ws281xColorWriter {
            spi,
            buffer: vec![],
        })
    }

    /// Writes the result of calling `Self::encode_rgb_array_as_spi` on every
    /// element of `colors` to the SPI device.
    pub fn write_colors(&mut self, colors: impl IntoIterator<Item = [u8; 3]>) -> io::Result<()> {
        self.buffer.clear();
        self.buffer
            .extend(colors.into_iter().flat_map(Self::encode_rgb_array_as_spi));
        self.spi.write_all(&self.buffer)
    }

    /// Shuffles into GRB and turns every bit into an on or off signal.
    pub fn encode_rgb_array_as_spi([r, g, b]: [u8; 3]) -> impl Iterator<Item = u8> {
        ArrayVec::from([g, r, b])
            .into_iter()
            .flat_map(|component_intensity| {
                (0..8).rev().flat_map(move |bits| {
                    // On/Off signals found at https://github.com/phip1611/ws2818-rgb-led-spi-driver/blob/master/src/timings.rs#L55
                    ArrayVec::from(if component_intensity >> bits & 1 == 0 {
                        [0b1111_1000, 0b0000_0000]
                    } else {
                        [0b1111_1111, 0b1000_0000]
                    })
                })
            })
    }
}
