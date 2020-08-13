use rainbow_emanator::rainbow_emanator;
use ws2818_rgb_led_spi_driver::{
    adapter::WS28xxAdapter,
    encoding::{encode_rgb, SPI_BYTES_PER_RGB_PIXEL},
};

fn main() {
    let mut adapter =
        WS28xxAdapter::new("/dev/spidev0.0").expect("failed to access /dev/spidev0.0");
    let start = std::time::Instant::now();
    let mut spi_data = vec![];

    loop {
        spi_data.clear();
        let elapsed = start.elapsed().as_millis() as f32;
        rainbow_emanator(8, 5, elapsed)
            .for_each(|rgb| spi_data.extend_from_slice(&encode_rgb_array(rgb)));
        adapter.write_encoded_rgb(&spi_data).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10))
    }
}

fn encode_rgb_array([r, g, b]: [u8; 3]) -> [u8; SPI_BYTES_PER_RGB_PIXEL] {
    encode_rgb(r, g, b)
}
