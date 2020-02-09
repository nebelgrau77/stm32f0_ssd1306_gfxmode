#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f0xx_hal as hal;

use cortex_m_rt::entry;
// use cortex_m::peripheral::syst::SystClkSource;
use ssd1306::{prelude::*, Builder as SSD1306Builder};
use ssd1306::prelude::DisplaySize::Display128x32 as DisplaySize;
use ssd1306::properties::DisplayProperties;
use embedded_graphics::{image::Image, pixelcolor::BinaryColor, prelude::*};

use crate::hal::{
    prelude::*,
    stm32,
    i2c::I2c,
    delay::Delay
};

#[entry]
fn main() -> ! {

    if let (Some(mut p), Some(cp)) = (stm32::Peripherals::take(),cortex_m::peripheral::Peripherals::take()) {
        
        cortex_m::interrupt::free(move |cs| {

        let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);
        
        let gpioa = p.GPIOA.split(&mut rcc);
        let scl = gpioa.pa9.into_alternate_af4(cs);
        let sda = gpioa.pa10.into_alternate_af4(cs);
        let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), &mut rcc);
        
        // Get delay provider
        let mut delay = Delay::new(cp.SYST, &rcc);

        // Set up the display
        
        

        
        let mut DATA: &[u8; 512] = &[85; 512];


        let mut disp: GraphicsMode<_> = SSD1306Builder::new().size(DisplaySize).connect_i2c(i2c).into();
        
        disp.init().unwrap();

        for a in 0..256 {
        
        let DATA  = &[(a as u8); 512];

        let im: Image<BinaryColor> = Image::new(DATA, 128, 32);
        
        im.draw(&mut disp);

        disp.flush().unwrap();

        }
       
    });
    
}

    loop {continue;}
    
}

