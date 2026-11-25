use esp_hal::{
    peripherals::{GPIO10, RMT},
    rmt::Rmt,
    time::Rate,
};

use rgb::RGB8;
use smart_leds_trait::SmartLedsWrite;

use rustyfarian_esp_hal_ws2812::{
    buffer_size,
    LongFramePolicy,
    Ws2812Rmt,
};

use crate::config::NUM_LEDS;

const RMT_BUFFER_SIZE: usize = buffer_size(NUM_LEDS);

pub fn init(rmt_peripheral: RMT<'static>, led_pin: GPIO10<'static>) -> impl SmartLedsWrite<Color = RGB8> {
    // Initialize RMT at 80 MHz
    let rmt = Rmt::new(rmt_peripheral, Rate::from_mhz(80)).unwrap();

    // Initialize WS2812 driver
    let led: Ws2812Rmt<'_, _, RMT_BUFFER_SIZE> = Ws2812Rmt::from_channel_creator(rmt.channel0, led_pin).unwrap();

    // Configure long frame transmission
    led.with_long_frame_policy(LongFramePolicy::Poll)
}
