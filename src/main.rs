#![no_std]
#![no_main]

mod config;
mod can;

use rtic::app;
use cortex_m::asm::delay;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m::interrupt::{free as disable_interrupts, CriticalSection};
use stm32l4xx_hal::time::Hertz;
use stm32l4xx_hal::{
    prelude::*,
    stm32,
    stm32::RCC
};
use stm32l4xx_hal::gpio::GpioExt;

#[app(device = stm32l4xx_hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        can: can::Can,
    }
    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        rtt_init_print!();

        let p = ctx.device;

        let mut flash = p.FLASH.constrain();
        let mut rcc_reg = p.RCC;
        rcc_reg.apb1enr1.modify(|_,w|w.can1en().set_bit());

        let mut rcc = rcc_reg.constrain();

        let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);
        let mut pwr = p.PWR.constrain(&mut rcc.apb1r1);


        // clock configuration (clocks run at nearly the maximum frequency)
        let clocks = rcc
            .cfgr
            .sysclk(80.mhz())
            .pclk1(80.mhz())
            .pclk2(80.mhz())
            .freeze(&mut flash.acr, &mut pwr);


        //unsafe{p.RCC.apb1enr1.modify(|_,w|w.can1en().set_bit());};

        let can_rx: config::CAN_RX_PIN = gpioa.pa11.into_af9(&mut gpioa.moder, &mut gpioa.afrh);
        let can_tx: config::CAN_TX_PIN = gpioa.pa12.into_af9(&mut gpioa.moder, &mut gpioa.afrh);
        //let can_rx: config::CAN_RX_PIN = gpioa.pa11.into_alternate_af4(&cs);
        //let can_tx: config::CAN_TX_PIN = gpioa.pa12.into_alternate_af4(&cs);
        let can_params: can::CanParams = can::CanParams{
            work_mode: can::CanMode::LoopBackMode,
            automatic_retransmission: can::AutomaticRetransmission::Enabled,
            automatic_busoff_management: can::AutomaticBussOffManagement::Enabled,
            auto_wake_up: can::AutomaticWakeUpMode::Enabled,
            pclk_Hz: clocks.pclk2(),
            bitrate: can::BitRate::_1Mbs,
        };

        let filter: can::Filter = can::Filter{
            mode: can::FilterMode::ListMode,
            scale_config: can::FilterScaleConfiguration::_32BitSingleConfig,
            id_or_mask: 0x1234567,
            enable: true,
            id_type: can::IdType::Extended,
            rtr: false
        };

        let can  = can::Can::new(
            can_tx,
            can_rx,
            p.CAN1,
            can_params,
            &[]
        );

        init::LateResources {
            can
        }
    }
    #[idle(resources = [can])]
    fn idle(ctx: idle::Context) -> ! {
        let mut can = ctx.resources.can;
        loop {
            delay(6_000_000);
            rprintln!("*************************");
            can.lock(|can|{
               can.write_to_mailbox(can::IdType::Extended, 0x1111_0000, &[0x01,0x02]);
            });
            delay(6_000_000);
        }

    }

    #[task(binds = CAN1_RX0, priority = 3 , resources = [can])]
    fn can_irq3(ctx: can_irq3::Context){
        rprintln!("CAN1_RX0");
        let can: &mut can::Can = ctx.resources.can;
        can.irq_state_machine(|id, data|{
            rprintln!("CAN_IRQ: id: {:x}; Data: {:?}", id, data);
        });
    }

};


use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use core::borrow::{BorrowMut, Borrow};


#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("Panic: {:?}", info);
    loop {
       // atomic::compiler_fence(Ordering::SeqCst);
    }
}