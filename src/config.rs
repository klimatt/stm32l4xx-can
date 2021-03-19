
use stm32l4xx_hal::{
    gpio::gpioa::{PA11, PA12},
    gpio::{Alternate, AF9, Output, PushPull, Floating, Input},
    spi::Spi,
    stm32::{SPI1}
};

pub type CAN_TX_PIN = PA12<Alternate<AF9, Input<Floating>>>;
pub type CAN_RX_PIN = PA11<Alternate<AF9, Input<Floating>>>;
