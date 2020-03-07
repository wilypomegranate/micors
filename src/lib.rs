#![deny(unsafe_code)]
#![no_std]
#![allow(dead_code)]

use stm32f1xx_hal::{gpio, gpio::gpioc, pac, prelude::*, rcc::Clocks};

use core::marker::PhantomData;

pub struct Pin {}

pub struct Stm32Board {
    // pc13: Pin,
    // dp: pac::Peripherals,
    // gpioc: Option<gpioc::Parts>,
    // rcc: Option<rcc:Rcc::Parts>,
    // flash: Option<Parts>,
    // rcc: Rcc,
    // gpioc: gpioc::Parts,
    pc13: gpioc::PC13<gpio::Input<gpio::Floating>>,
    pc9: gpioc::PC9<gpio::Input<gpio::Floating>>,
    crh: gpioc::CRH,
    clocks: Clocks,
}

impl Stm32Board {
    pub fn new() -> Stm32Board {
        cortex_m::Peripherals::take().unwrap();
        let dp = pac::Peripherals::take().unwrap();
        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();
        let gpioc = dp.GPIOC.split(&mut rcc.apb2);
        Stm32Board {
            // pc13: Pin{},
            // gpioc: gpioc,
            // rcc: None,
            // flash: dp.FLASH.constrain(),
            // rcc: dp.RCC.constrain(),
            pc13: gpioc.pc13,
            pc9: gpioc.pc9,
            crh: gpioc.crh,
            clocks: rcc.cfgr.freeze(&mut flash.acr),
        }
    }

    pub fn digital_out(mut self) -> impl FnMut() -> () {
        let pc13 = self.pc13;
        pc13.into_push_pull_output(&mut self.crh);
        let mut new = pc13.into_pull_down_input(&mut self.crh);
        // pc13.into_pull_down_input(&mut self.crh);
        // self.gpioc.pc13.into_push_pull_output(&mut self.gpioc.crh);
        // match self.flash {
        //     None => {
        //         // let mut rcc = self.dp.RCC.constrain();
        //         // self.rcc = Some(self.dp.RCC.constrain());
        //         // self.flash = Some(self.dp.FLASH.constrain());
        //         // let mut flash = self.dp.FLASH.constrain();
        //         // self.flash = Some(&mut self.dp.FLASH.constrain());
        //         self.flash = Some(self.dp.FLASH.constrain())
        //         // self.gpioc = Some(self.dp.GPIOC.split(&mut rcc.apb2));
        //     }

        //     _ => {}
        // }

        // DigitalOut::new(|| self.flash = Some(self.dp.FLASH.constrain()))
        // self.dp.FLASH.constrain();
        || {}
    }
}

// trait DigiOutTrait<F, G>
// where
//     F: FnMut(),
//     G: FnMut(),
// {
//     fn high(&self);
//     fn low(&self);
// }

pub struct DigitalOut<F>
where
    F: FnMut(),
{
    init: PhantomData<F>, // This HAL type should really encapsulate the pin in question too.
                          // wrapped_internal_hal_representation: HAL_TYPE,
                          // gpioc: &'a mut gpioc::Parts,
                          // flash: &'b mut Parts
}

impl<F> DigitalOut<F>
where
    F: FnMut(),
{
    // pub fn new(init: F) -> Self {
    //     init();
    //     DigitalOut { init: PhantomData }
    // }
}
