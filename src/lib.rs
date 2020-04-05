#![doc = "Peripheral access API for MKM34Z7 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
extern "C" {
    fn DMA0();
    fn DMA1();
    fn DMA2();
    fn DMA3();
    fn SPI0_SPI1();
    fn PDB0();
    fn PMC();
    fn TMR0();
    fn TMR1();
    fn TMR2();
    fn TMR3();
    fn PIT0_PIT1();
    fn LLWU();
    fn FTFA();
    fn CMP0_CMP1_CMP2();
    fn LCD();
    fn ADC0();
    fn PTX();
    fn RNGA();
    fn UART0_UART1_UART2_UART3();
    fn MMAU();
    fn AFE_CH0();
    fn AFE_CH1();
    fn AFE_CH2();
    fn AFE_CH3();
    fn RTC();
    fn I2C0_I2C1();
    fn LPUART0();
    fn MCG();
    fn WDOG_EWM();
    fn LPTMR0();
    fn XBAR();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _handler: DMA0 },
    Vector { _handler: DMA1 },
    Vector { _handler: DMA2 },
    Vector { _handler: DMA3 },
    Vector {
        _handler: SPI0_SPI1,
    },
    Vector { _handler: PDB0 },
    Vector { _handler: PMC },
    Vector { _handler: TMR0 },
    Vector { _handler: TMR1 },
    Vector { _handler: TMR2 },
    Vector { _handler: TMR3 },
    Vector {
        _handler: PIT0_PIT1,
    },
    Vector { _handler: LLWU },
    Vector { _handler: FTFA },
    Vector {
        _handler: CMP0_CMP1_CMP2,
    },
    Vector { _handler: LCD },
    Vector { _handler: ADC0 },
    Vector { _handler: PTX },
    Vector { _handler: RNGA },
    Vector {
        _handler: UART0_UART1_UART2_UART3,
    },
    Vector { _handler: MMAU },
    Vector { _handler: AFE_CH0 },
    Vector { _handler: AFE_CH1 },
    Vector { _handler: AFE_CH2 },
    Vector { _handler: AFE_CH3 },
    Vector { _handler: RTC },
    Vector {
        _handler: I2C0_I2C1,
    },
    Vector { _handler: LPUART0 },
    Vector { _handler: MCG },
    Vector { _handler: WDOG_EWM },
    Vector { _handler: LPTMR0 },
    Vector { _handler: XBAR },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - DMA0"]
    DMA0 = 0,
    #[doc = "1 - DMA1"]
    DMA1 = 1,
    #[doc = "2 - DMA2"]
    DMA2 = 2,
    #[doc = "3 - DMA3"]
    DMA3 = 3,
    #[doc = "4 - SPI0_SPI1"]
    SPI0_SPI1 = 4,
    #[doc = "5 - PDB0"]
    PDB0 = 5,
    #[doc = "6 - PMC"]
    PMC = 6,
    #[doc = "7 - TMR0"]
    TMR0 = 7,
    #[doc = "8 - TMR1"]
    TMR1 = 8,
    #[doc = "9 - TMR2"]
    TMR2 = 9,
    #[doc = "10 - TMR3"]
    TMR3 = 10,
    #[doc = "11 - PIT0_PIT1"]
    PIT0_PIT1 = 11,
    #[doc = "12 - LLWU"]
    LLWU = 12,
    #[doc = "13 - FTFA"]
    FTFA = 13,
    #[doc = "14 - CMP0_CMP1_CMP2"]
    CMP0_CMP1_CMP2 = 14,
    #[doc = "15 - LCD"]
    LCD = 15,
    #[doc = "16 - ADC0"]
    ADC0 = 16,
    #[doc = "17 - PTx"]
    PTX = 17,
    #[doc = "18 - RNGA"]
    RNGA = 18,
    #[doc = "19 - UART0_UART1_UART2_UART3"]
    UART0_UART1_UART2_UART3 = 19,
    #[doc = "20 - MMAU"]
    MMAU = 20,
    #[doc = "21 - AFE_CH0"]
    AFE_CH0 = 21,
    #[doc = "22 - AFE_CH1"]
    AFE_CH1 = 22,
    #[doc = "23 - AFE_CH2"]
    AFE_CH2 = 23,
    #[doc = "24 - AFE_CH3"]
    AFE_CH3 = 24,
    #[doc = "25 - RTC"]
    RTC = 25,
    #[doc = "26 - I2C0_I2C1"]
    I2C0_I2C1 = 26,
    #[doc = "27 - LPUART0"]
    LPUART0 = 27,
    #[doc = "28 - MCG"]
    MCG = 28,
    #[doc = "29 - WDOG_EWM"]
    WDOG_EWM = 29,
    #[doc = "30 - LPTMR0"]
    LPTMR0 = 30,
    #[doc = "31 - XBAR"]
    XBAR = 31,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Flash configuration field"]
pub struct FTFA_FLASHCONFIG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTFA_FLASHCONFIG {}
impl FTFA_FLASHCONFIG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ftfa_flash_config::RegisterBlock {
        0x0400 as *const _
    }
}
impl Deref for FTFA_FLASHCONFIG {
    type Target = ftfa_flash_config::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FTFA_FLASHCONFIG::ptr() }
    }
}
#[doc = "Flash configuration field"]
pub mod ftfa_flash_config;
#[doc = "AIPS-Lite Bridge"]
pub struct AIPS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AIPS {}
impl AIPS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aips::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for AIPS {
    type Target = aips::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AIPS::ptr() }
    }
}
#[doc = "AIPS-Lite Bridge"]
pub mod aips;
#[doc = "DMA Controller"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "DMA Controller"]
pub mod dma;
#[doc = "Memory protection unit"]
pub struct SYSMPU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSMPU {}
impl SYSMPU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sysmpu::RegisterBlock {
        0x4000_a000 as *const _
    }
}
impl Deref for SYSMPU {
    type Target = sysmpu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSMPU::ptr() }
    }
}
#[doc = "Memory protection unit"]
pub mod sysmpu;
#[doc = "Flash Memory Interface"]
pub struct FTFA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTFA {}
impl FTFA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ftfa::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for FTFA {
    type Target = ftfa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FTFA::ptr() }
    }
}
#[doc = "Flash Memory Interface"]
pub mod ftfa;
#[doc = "DMA channel multiplexor"]
pub struct DMAMUX0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX0 {}
impl DMAMUX0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmamux0::RegisterBlock {
        0x4002_1000 as *const _
    }
}
impl Deref for DMAMUX0 {
    type Target = dmamux0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAMUX0::ptr() }
    }
}
#[doc = "DMA channel multiplexor"]
pub mod dmamux0;
#[doc = "Random Number Generator Accelerator"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng::RegisterBlock {
        0x4002_9000 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RNG::ptr() }
    }
}
#[doc = "Random Number Generator Accelerator"]
pub mod rng;
#[doc = "Low Power Universal Asynchronous Receiver/Transmitter"]
pub struct LPUART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART0 {}
impl LPUART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpuart0::RegisterBlock {
        0x4002_a000 as *const _
    }
}
impl Deref for LPUART0 {
    type Target = lpuart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPUART0::ptr() }
    }
}
#[doc = "Low Power Universal Asynchronous Receiver/Transmitter"]
pub mod lpuart0;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        0x4002_b000 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc0;
#[doc = "Periodic Interrupt Timer"]
pub struct PIT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIT0 {}
impl PIT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pit0::RegisterBlock {
        0x4002_d000 as *const _
    }
}
impl Deref for PIT0 {
    type Target = pit0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIT0::ptr() }
    }
}
#[doc = "Periodic Interrupt Timer"]
pub mod pit0;
#[doc = "Periodic Interrupt Timer"]
pub struct PIT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIT1 {}
impl PIT1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pit1::RegisterBlock {
        0x4002_e000 as *const _
    }
}
impl Deref for PIT1 {
    type Target = pit1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIT1::ptr() }
    }
}
#[doc = "Periodic Interrupt Timer"]
pub mod pit1;
#[doc = "Analog Front End"]
pub struct AFE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AFE {}
impl AFE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const afe::RegisterBlock {
        0x4003_0000 as *const _
    }
}
impl Deref for AFE {
    type Target = afe::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AFE::ptr() }
    }
}
#[doc = "Analog Front End"]
pub mod afe;
#[doc = "Cyclic Redundancy Check"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        0x4003_4000 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cyclic Redundancy Check"]
pub mod crc;
#[doc = "Programmable Delay Block"]
pub struct PDB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDB0 {}
impl PDB0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdb0::RegisterBlock {
        0x4003_6000 as *const _
    }
}
impl Deref for PDB0 {
    type Target = pdb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDB0::ptr() }
    }
}
#[doc = "Programmable Delay Block"]
pub mod pdb0;
#[doc = "Pin Control and Interrupts"]
pub struct PORTA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTA {}
impl PORTA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const porta::RegisterBlock {
        0x4004_6000 as *const _
    }
}
impl Deref for PORTA {
    type Target = porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTA::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod porta;
#[doc = "Pin Control and Interrupts"]
pub struct PORTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTB {}
impl PORTB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const portb::RegisterBlock {
        0x4004_7000 as *const _
    }
}
impl Deref for PORTB {
    type Target = portb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTB::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portb;
#[doc = "Pin Control and Interrupts"]
pub struct PORTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTC {}
impl PORTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const portc::RegisterBlock {
        0x4004_8000 as *const _
    }
}
impl Deref for PORTC {
    type Target = portc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTC::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portc;
#[doc = "Pin Control and Interrupts"]
pub struct PORTD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTD {}
impl PORTD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const portd::RegisterBlock {
        0x4004_9000 as *const _
    }
}
impl Deref for PORTD {
    type Target = portd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTD::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portd;
#[doc = "Pin Control and Interrupts"]
pub struct PORTE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTE {}
impl PORTE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const porte::RegisterBlock {
        0x4004_a000 as *const _
    }
}
impl Deref for PORTE {
    type Target = porte::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTE::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod porte;
#[doc = "Pin Control and Interrupts"]
pub struct PORTF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTF {}
impl PORTF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const portf::RegisterBlock {
        0x4004_b000 as *const _
    }
}
impl Deref for PORTF {
    type Target = portf::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTF::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portf;
#[doc = "Pin Control and Interrupts"]
pub struct PORTG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTG {}
impl PORTG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const portg::RegisterBlock {
        0x4004_c000 as *const _
    }
}
impl Deref for PORTG {
    type Target = portg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTG::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portg;
#[doc = "Pin Control and Interrupts"]
pub struct PORTH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTH {}
impl PORTH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const porth::RegisterBlock {
        0x4004_d000 as *const _
    }
}
impl Deref for PORTH {
    type Target = porth::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTH::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod porth;
#[doc = "Pin Control and Interrupts"]
pub struct PORTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTI {}
impl PORTI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const porti::RegisterBlock {
        0x4004_e000 as *const _
    }
}
impl Deref for PORTI {
    type Target = porti::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTI::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod porti;
#[doc = "Pin Control and Interrupts"]
pub struct PORTJ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTJ {}
impl PORTJ {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const portj::RegisterBlock {
        0x4003_7000 as *const _
    }
}
impl Deref for PORTJ {
    type Target = portj::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTJ::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portj;
#[doc = "Pin Control and Interrupts"]
pub struct PORTK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTK {}
impl PORTK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const portk::RegisterBlock {
        0x4003_8000 as *const _
    }
}
impl Deref for PORTK {
    type Target = portk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTK::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portk;
#[doc = "Pin Control and Interrupts"]
pub struct PORTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTL {}
impl PORTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const portl::RegisterBlock {
        0x4003_9000 as *const _
    }
}
impl Deref for PORTL {
    type Target = portl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTL::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portl;
#[doc = "Pin Control and Interrupts"]
pub struct PORTM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTM {}
impl PORTM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const portm::RegisterBlock {
        0x4003_a000 as *const _
    }
}
impl Deref for PORTM {
    type Target = portm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTM::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portm;
#[doc = "Low Power Timer"]
pub struct LPTMR0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTMR0 {}
impl LPTMR0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptmr0::RegisterBlock {
        0x4003_c000 as *const _
    }
}
impl Deref for LPTMR0 {
    type Target = lptmr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTMR0::ptr() }
    }
}
#[doc = "Low Power Timer"]
pub mod lptmr0;
#[doc = "System Integration Module"]
pub struct SIM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SIM {}
impl SIM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sim::RegisterBlock {
        0x4003_e000 as *const _
    }
}
impl Deref for SIM {
    type Target = sim::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SIM::ptr() }
    }
}
#[doc = "System Integration Module"]
pub mod sim;
#[doc = "Segment Liquid Crystal Display"]
pub struct LCD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LCD {}
impl LCD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lcd::RegisterBlock {
        0x4004_3000 as *const _
    }
}
impl Deref for LCD {
    type Target = lcd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LCD::ptr() }
    }
}
#[doc = "Segment Liquid Crystal Display"]
pub mod lcd;
#[doc = "Real Time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4005_0000 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real Time Clock"]
pub mod rtc;
#[doc = "Generation 2008 Watchdog Timer"]
pub struct WDOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG {}
impl WDOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdog::RegisterBlock {
        0x4005_3000 as *const _
    }
}
impl Deref for WDOG {
    type Target = wdog::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDOG::ptr() }
    }
}
#[doc = "Generation 2008 Watchdog Timer"]
pub mod wdog;
#[doc = "Crossbar Switch"]
pub struct XBAR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XBAR {}
impl XBAR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const xbar::RegisterBlock {
        0x4005_5000 as *const _
    }
}
impl Deref for XBAR {
    type Target = xbar::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*XBAR::ptr() }
    }
}
#[doc = "Crossbar Switch"]
pub mod xbar;
#[doc = "Quad Timer"]
pub struct TMR0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR0 {}
impl TMR0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr0::RegisterBlock {
        0x4005_7000 as *const _
    }
}
impl Deref for TMR0 {
    type Target = tmr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR0::ptr() }
    }
}
#[doc = "Quad Timer"]
pub mod tmr0;
#[doc = "Quad Timer"]
pub struct TMR1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR1 {}
impl TMR1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr1::RegisterBlock {
        0x4005_8000 as *const _
    }
}
impl Deref for TMR1 {
    type Target = tmr1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR1::ptr() }
    }
}
#[doc = "Quad Timer"]
pub mod tmr1;
#[doc = "Quad Timer"]
pub struct TMR2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR2 {}
impl TMR2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr2::RegisterBlock {
        0x4005_9000 as *const _
    }
}
impl Deref for TMR2 {
    type Target = tmr2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR2::ptr() }
    }
}
#[doc = "Quad Timer"]
pub mod tmr2;
#[doc = "Quad Timer"]
pub struct TMR3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR3 {}
impl TMR3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr3::RegisterBlock {
        0x4005_a000 as *const _
    }
}
impl Deref for TMR3 {
    type Target = tmr3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR3::ptr() }
    }
}
#[doc = "Quad Timer"]
pub mod tmr3;
#[doc = "External Watchdog Monitor"]
pub struct EWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EWM {}
impl EWM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ewm::RegisterBlock {
        0x4006_1000 as *const _
    }
}
impl Deref for EWM {
    type Target = ewm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EWM::ptr() }
    }
}
#[doc = "External Watchdog Monitor"]
pub mod ewm;
#[doc = "Multipurpose Clock Generator module"]
pub struct MCG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCG {}
impl MCG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mcg::RegisterBlock {
        0x4006_4000 as *const _
    }
}
impl Deref for MCG {
    type Target = mcg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MCG::ptr() }
    }
}
#[doc = "Multipurpose Clock Generator module"]
pub mod mcg;
#[doc = "Oscillator"]
pub struct OSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSC {}
impl OSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const osc::RegisterBlock {
        0x4006_6000 as *const _
    }
}
impl Deref for OSC {
    type Target = osc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OSC::ptr() }
    }
}
#[doc = "Oscillator"]
pub mod osc;
#[doc = "Inter-Integrated Circuit"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4006_7000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit"]
pub mod i2c0;
#[doc = "Inter-Integrated Circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4006_8000 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit"]
pub mod i2c1;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4006_a000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub mod uart0;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        0x4006_b000 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub mod uart1;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart2::RegisterBlock {
        0x4006_c000 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub mod uart2;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct UART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART3 {}
impl UART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart3::RegisterBlock {
        0x4006_d000 as *const _
    }
}
impl Deref for UART3 {
    type Target = uart3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART3::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub mod uart3;
#[doc = "Voltage Reference"]
pub struct VREF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREF {}
impl VREF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vref::RegisterBlock {
        0x4006_f000 as *const _
    }
}
impl Deref for VREF {
    type Target = vref::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*VREF::ptr() }
    }
}
#[doc = "Voltage Reference"]
pub mod vref;
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP0 {}
impl CMP0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp0::RegisterBlock {
        0x4007_2000 as *const _
    }
}
impl Deref for CMP0 {
    type Target = cmp0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP0::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub mod cmp0;
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP1 {}
impl CMP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp1::RegisterBlock {
        0x4007_2008 as *const _
    }
}
impl Deref for CMP1 {
    type Target = cmp1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP1::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub mod cmp1;
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP2 {}
impl CMP2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp2::RegisterBlock {
        0x4007_2010 as *const _
    }
}
impl Deref for CMP2 {
    type Target = cmp2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP2::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub mod cmp2;
#[doc = "Serial Peripheral Interface"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4007_5000 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi0;
#[doc = "Serial Peripheral Interface"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4007_6000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi1;
#[doc = "Reset Control Module"]
pub struct RCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCM {}
impl RCM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rcm::RegisterBlock {
        0x4007_b000 as *const _
    }
}
impl Deref for RCM {
    type Target = rcm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RCM::ptr() }
    }
}
#[doc = "Reset Control Module"]
pub mod rcm;
#[doc = "Low leakage wakeup unit"]
pub struct LLWU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LLWU {}
impl LLWU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const llwu::RegisterBlock {
        0x4007_c000 as *const _
    }
}
impl Deref for LLWU {
    type Target = llwu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LLWU::ptr() }
    }
}
#[doc = "Low leakage wakeup unit"]
pub mod llwu;
#[doc = "Power Management Controller"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmc::RegisterBlock {
        0x4007_d000 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "Power Management Controller"]
pub mod pmc;
#[doc = "System Mode Controller"]
pub struct SMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMC {}
impl SMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smc::RegisterBlock {
        0x4007_e000 as *const _
    }
}
impl Deref for SMC {
    type Target = smc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SMC::ptr() }
    }
}
#[doc = "System Mode Controller"]
pub mod smc;
#[doc = "General Purpose Input/Output"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x400f_f000 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioa;
#[doc = "General Purpose Input/Output"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        0x400f_f001 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpiob;
#[doc = "General Purpose Input/Output"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x400f_f002 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioc;
#[doc = "General Purpose Input/Output"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiod::RegisterBlock {
        0x400f_f003 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpiod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpiod;
#[doc = "General Purpose Input/Output"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioe::RegisterBlock {
        0x400f_f040 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioe::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioe;
#[doc = "General Purpose Input/Output"]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiof::RegisterBlock {
        0x400f_f041 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpiof::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOF::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpiof;
#[doc = "General Purpose Input/Output"]
pub struct GPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOG {}
impl GPIOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiog::RegisterBlock {
        0x400f_f042 as *const _
    }
}
impl Deref for GPIOG {
    type Target = gpiog::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOG::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpiog;
#[doc = "General Purpose Input/Output"]
pub struct GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOH {}
impl GPIOH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioh::RegisterBlock {
        0x400f_f043 as *const _
    }
}
impl Deref for GPIOH {
    type Target = gpioh::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOH::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioh;
#[doc = "General Purpose Input/Output"]
pub struct GPIOI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOI {}
impl GPIOI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioi::RegisterBlock {
        0x400f_f080 as *const _
    }
}
impl Deref for GPIOI {
    type Target = gpioi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOI::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioi;
#[doc = "General Purpose Input/Output"]
pub struct GPIOJ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOJ {}
impl GPIOJ {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioj::RegisterBlock {
        0x400f_f081 as *const _
    }
}
impl Deref for GPIOJ {
    type Target = gpioj::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOJ::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioj;
#[doc = "General Purpose Input/Output"]
pub struct GPIOK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOK {}
impl GPIOK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiok::RegisterBlock {
        0x400f_f082 as *const _
    }
}
impl Deref for GPIOK {
    type Target = gpiok::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOK::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpiok;
#[doc = "General Purpose Input/Output"]
pub struct GPIOL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOL {}
impl GPIOL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiol::RegisterBlock {
        0x400f_f083 as *const _
    }
}
impl Deref for GPIOL {
    type Target = gpiol::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOL::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpiol;
#[doc = "General Purpose Input/Output"]
pub struct GPIOM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOM {}
impl GPIOM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiom::RegisterBlock {
        0x400f_f0c0 as *const _
    }
}
impl Deref for GPIOM {
    type Target = gpiom::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOM::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpiom;
#[doc = "System Control Block"]
pub struct SYSTEMCONTROL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEMCONTROL {}
impl SYSTEMCONTROL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const system_control::RegisterBlock {
        0xe000_e000 as *const _
    }
}
impl Deref for SYSTEMCONTROL {
    type Target = system_control::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTEMCONTROL::ptr() }
    }
}
#[doc = "System Control Block"]
pub mod system_control;
#[doc = "System timer"]
pub struct SYSTICK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTICK {}
impl SYSTICK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys_tick::RegisterBlock {
        0xe000_e010 as *const _
    }
}
impl Deref for SYSTICK {
    type Target = sys_tick::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTICK::ptr() }
    }
}
#[doc = "System timer"]
pub mod sys_tick;
#[doc = "Micro Trace Buffer"]
pub struct MTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTB {}
impl MTB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mtb::RegisterBlock {
        0xf000_0000 as *const _
    }
}
impl Deref for MTB {
    type Target = mtb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MTB::ptr() }
    }
}
#[doc = "Micro Trace Buffer"]
pub mod mtb;
#[doc = "MTB data watchpoint and trace"]
pub struct MTBDWT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTBDWT {}
impl MTBDWT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mtbdwt::RegisterBlock {
        0xf000_1000 as *const _
    }
}
impl Deref for MTBDWT {
    type Target = mtbdwt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MTBDWT::ptr() }
    }
}
#[doc = "MTB data watchpoint and trace"]
pub mod mtbdwt;
#[doc = "System ROM"]
pub struct ROM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ROM {}
impl ROM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rom::RegisterBlock {
        0xf000_2000 as *const _
    }
}
impl Deref for ROM {
    type Target = rom::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ROM::ptr() }
    }
}
#[doc = "System ROM"]
pub mod rom;
#[doc = "Core Platform Miscellaneous Control Module"]
pub struct MCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCM {}
impl MCM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mcm::RegisterBlock {
        0xf000_3000 as *const _
    }
}
impl Deref for MCM {
    type Target = mcm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MCM::ptr() }
    }
}
#[doc = "Core Platform Miscellaneous Control Module"]
pub mod mcm;
#[doc = "Memory-Mapped Arithmetic Unit"]
pub struct MMAU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MMAU {}
impl MMAU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mmau::RegisterBlock {
        0xf000_4000 as *const _
    }
}
impl Deref for MMAU {
    type Target = mmau::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MMAU::ptr() }
    }
}
#[doc = "Memory-Mapped Arithmetic Unit"]
pub mod mmau;
#[doc = "Memory Mapped Cryptographic Acceleration Unit (MMCAU)"]
pub struct CAU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAU {}
impl CAU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cau::RegisterBlock {
        0xf000_5000 as *const _
    }
}
impl Deref for CAU {
    type Target = cau::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAU::ptr() }
    }
}
#[doc = "Memory Mapped Cryptographic Acceleration Unit (MMCAU)"]
pub mod cau;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOA {}
impl FGPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fgpioa::RegisterBlock {
        0xf800_0000 as *const _
    }
}
impl Deref for FGPIOA {
    type Target = fgpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FGPIOA::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpioa;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOB {}
impl FGPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fgpiob::RegisterBlock {
        0xf800_0001 as *const _
    }
}
impl Deref for FGPIOB {
    type Target = fgpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FGPIOB::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpiob;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOC {}
impl FGPIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fgpioc::RegisterBlock {
        0xf800_0002 as *const _
    }
}
impl Deref for FGPIOC {
    type Target = fgpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FGPIOC::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpioc;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOD {}
impl FGPIOD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fgpiod::RegisterBlock {
        0xf800_0003 as *const _
    }
}
impl Deref for FGPIOD {
    type Target = fgpiod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FGPIOD::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpiod;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOE {}
impl FGPIOE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fgpioe::RegisterBlock {
        0xf800_0040 as *const _
    }
}
impl Deref for FGPIOE {
    type Target = fgpioe::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FGPIOE::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpioe;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOF {}
impl FGPIOF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fgpiof::RegisterBlock {
        0xf800_0041 as *const _
    }
}
impl Deref for FGPIOF {
    type Target = fgpiof::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FGPIOF::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpiof;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOG {}
impl FGPIOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fgpiog::RegisterBlock {
        0xf800_0042 as *const _
    }
}
impl Deref for FGPIOG {
    type Target = fgpiog::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FGPIOG::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpiog;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOH {}
impl FGPIOH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fgpioh::RegisterBlock {
        0xf800_0043 as *const _
    }
}
impl Deref for FGPIOH {
    type Target = fgpioh::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FGPIOH::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpioh;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOI {}
impl FGPIOI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fgpioi::RegisterBlock {
        0xf800_0080 as *const _
    }
}
impl Deref for FGPIOI {
    type Target = fgpioi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FGPIOI::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpioi;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOJ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOJ {}
impl FGPIOJ {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fgpioj::RegisterBlock {
        0xf800_0081 as *const _
    }
}
impl Deref for FGPIOJ {
    type Target = fgpioj::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FGPIOJ::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpioj;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOK {}
impl FGPIOK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fgpiok::RegisterBlock {
        0xf800_0082 as *const _
    }
}
impl Deref for FGPIOK {
    type Target = fgpiok::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FGPIOK::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpiok;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOL {}
impl FGPIOL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fgpiol::RegisterBlock {
        0xf800_0083 as *const _
    }
}
impl Deref for FGPIOL {
    type Target = fgpiol::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FGPIOL::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpiol;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOM {}
impl FGPIOM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fgpiom::RegisterBlock {
        0xf800_00c0 as *const _
    }
}
impl Deref for FGPIOM {
    type Target = fgpiom::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FGPIOM::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpiom;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "FTFA_FLASHCONFIG"]
    pub FTFA_FLASHCONFIG: FTFA_FLASHCONFIG,
    #[doc = "AIPS"]
    pub AIPS: AIPS,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "SYSMPU"]
    pub SYSMPU: SYSMPU,
    #[doc = "FTFA"]
    pub FTFA: FTFA,
    #[doc = "DMAMUX0"]
    pub DMAMUX0: DMAMUX0,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "LPUART0"]
    pub LPUART0: LPUART0,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "PIT0"]
    pub PIT0: PIT0,
    #[doc = "PIT1"]
    pub PIT1: PIT1,
    #[doc = "AFE"]
    pub AFE: AFE,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "PDB0"]
    pub PDB0: PDB0,
    #[doc = "PORTA"]
    pub PORTA: PORTA,
    #[doc = "PORTB"]
    pub PORTB: PORTB,
    #[doc = "PORTC"]
    pub PORTC: PORTC,
    #[doc = "PORTD"]
    pub PORTD: PORTD,
    #[doc = "PORTE"]
    pub PORTE: PORTE,
    #[doc = "PORTF"]
    pub PORTF: PORTF,
    #[doc = "PORTG"]
    pub PORTG: PORTG,
    #[doc = "PORTH"]
    pub PORTH: PORTH,
    #[doc = "PORTI"]
    pub PORTI: PORTI,
    #[doc = "PORTJ"]
    pub PORTJ: PORTJ,
    #[doc = "PORTK"]
    pub PORTK: PORTK,
    #[doc = "PORTL"]
    pub PORTL: PORTL,
    #[doc = "PORTM"]
    pub PORTM: PORTM,
    #[doc = "LPTMR0"]
    pub LPTMR0: LPTMR0,
    #[doc = "SIM"]
    pub SIM: SIM,
    #[doc = "LCD"]
    pub LCD: LCD,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "WDOG"]
    pub WDOG: WDOG,
    #[doc = "XBAR"]
    pub XBAR: XBAR,
    #[doc = "TMR0"]
    pub TMR0: TMR0,
    #[doc = "TMR1"]
    pub TMR1: TMR1,
    #[doc = "TMR2"]
    pub TMR2: TMR2,
    #[doc = "TMR3"]
    pub TMR3: TMR3,
    #[doc = "EWM"]
    pub EWM: EWM,
    #[doc = "MCG"]
    pub MCG: MCG,
    #[doc = "OSC"]
    pub OSC: OSC,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "UART3"]
    pub UART3: UART3,
    #[doc = "VREF"]
    pub VREF: VREF,
    #[doc = "CMP0"]
    pub CMP0: CMP0,
    #[doc = "CMP1"]
    pub CMP1: CMP1,
    #[doc = "CMP2"]
    pub CMP2: CMP2,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "RCM"]
    pub RCM: RCM,
    #[doc = "LLWU"]
    pub LLWU: LLWU,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "SMC"]
    pub SMC: SMC,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "GPIOF"]
    pub GPIOF: GPIOF,
    #[doc = "GPIOG"]
    pub GPIOG: GPIOG,
    #[doc = "GPIOH"]
    pub GPIOH: GPIOH,
    #[doc = "GPIOI"]
    pub GPIOI: GPIOI,
    #[doc = "GPIOJ"]
    pub GPIOJ: GPIOJ,
    #[doc = "GPIOK"]
    pub GPIOK: GPIOK,
    #[doc = "GPIOL"]
    pub GPIOL: GPIOL,
    #[doc = "GPIOM"]
    pub GPIOM: GPIOM,
    #[doc = "SYSTEMCONTROL"]
    pub SYSTEMCONTROL: SYSTEMCONTROL,
    #[doc = "SYSTICK"]
    pub SYSTICK: SYSTICK,
    #[doc = "MTB"]
    pub MTB: MTB,
    #[doc = "MTBDWT"]
    pub MTBDWT: MTBDWT,
    #[doc = "ROM"]
    pub ROM: ROM,
    #[doc = "MCM"]
    pub MCM: MCM,
    #[doc = "MMAU"]
    pub MMAU: MMAU,
    #[doc = "CAU"]
    pub CAU: CAU,
    #[doc = "FGPIOA"]
    pub FGPIOA: FGPIOA,
    #[doc = "FGPIOB"]
    pub FGPIOB: FGPIOB,
    #[doc = "FGPIOC"]
    pub FGPIOC: FGPIOC,
    #[doc = "FGPIOD"]
    pub FGPIOD: FGPIOD,
    #[doc = "FGPIOE"]
    pub FGPIOE: FGPIOE,
    #[doc = "FGPIOF"]
    pub FGPIOF: FGPIOF,
    #[doc = "FGPIOG"]
    pub FGPIOG: FGPIOG,
    #[doc = "FGPIOH"]
    pub FGPIOH: FGPIOH,
    #[doc = "FGPIOI"]
    pub FGPIOI: FGPIOI,
    #[doc = "FGPIOJ"]
    pub FGPIOJ: FGPIOJ,
    #[doc = "FGPIOK"]
    pub FGPIOK: FGPIOK,
    #[doc = "FGPIOL"]
    pub FGPIOL: FGPIOL,
    #[doc = "FGPIOM"]
    pub FGPIOM: FGPIOM,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            FTFA_FLASHCONFIG: FTFA_FLASHCONFIG {
                _marker: PhantomData,
            },
            AIPS: AIPS {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            SYSMPU: SYSMPU {
                _marker: PhantomData,
            },
            FTFA: FTFA {
                _marker: PhantomData,
            },
            DMAMUX0: DMAMUX0 {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            LPUART0: LPUART0 {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            PIT0: PIT0 {
                _marker: PhantomData,
            },
            PIT1: PIT1 {
                _marker: PhantomData,
            },
            AFE: AFE {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            PDB0: PDB0 {
                _marker: PhantomData,
            },
            PORTA: PORTA {
                _marker: PhantomData,
            },
            PORTB: PORTB {
                _marker: PhantomData,
            },
            PORTC: PORTC {
                _marker: PhantomData,
            },
            PORTD: PORTD {
                _marker: PhantomData,
            },
            PORTE: PORTE {
                _marker: PhantomData,
            },
            PORTF: PORTF {
                _marker: PhantomData,
            },
            PORTG: PORTG {
                _marker: PhantomData,
            },
            PORTH: PORTH {
                _marker: PhantomData,
            },
            PORTI: PORTI {
                _marker: PhantomData,
            },
            PORTJ: PORTJ {
                _marker: PhantomData,
            },
            PORTK: PORTK {
                _marker: PhantomData,
            },
            PORTL: PORTL {
                _marker: PhantomData,
            },
            PORTM: PORTM {
                _marker: PhantomData,
            },
            LPTMR0: LPTMR0 {
                _marker: PhantomData,
            },
            SIM: SIM {
                _marker: PhantomData,
            },
            LCD: LCD {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            WDOG: WDOG {
                _marker: PhantomData,
            },
            XBAR: XBAR {
                _marker: PhantomData,
            },
            TMR0: TMR0 {
                _marker: PhantomData,
            },
            TMR1: TMR1 {
                _marker: PhantomData,
            },
            TMR2: TMR2 {
                _marker: PhantomData,
            },
            TMR3: TMR3 {
                _marker: PhantomData,
            },
            EWM: EWM {
                _marker: PhantomData,
            },
            MCG: MCG {
                _marker: PhantomData,
            },
            OSC: OSC {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            UART3: UART3 {
                _marker: PhantomData,
            },
            VREF: VREF {
                _marker: PhantomData,
            },
            CMP0: CMP0 {
                _marker: PhantomData,
            },
            CMP1: CMP1 {
                _marker: PhantomData,
            },
            CMP2: CMP2 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            RCM: RCM {
                _marker: PhantomData,
            },
            LLWU: LLWU {
                _marker: PhantomData,
            },
            PMC: PMC {
                _marker: PhantomData,
            },
            SMC: SMC {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            GPIOF: GPIOF {
                _marker: PhantomData,
            },
            GPIOG: GPIOG {
                _marker: PhantomData,
            },
            GPIOH: GPIOH {
                _marker: PhantomData,
            },
            GPIOI: GPIOI {
                _marker: PhantomData,
            },
            GPIOJ: GPIOJ {
                _marker: PhantomData,
            },
            GPIOK: GPIOK {
                _marker: PhantomData,
            },
            GPIOL: GPIOL {
                _marker: PhantomData,
            },
            GPIOM: GPIOM {
                _marker: PhantomData,
            },
            SYSTEMCONTROL: SYSTEMCONTROL {
                _marker: PhantomData,
            },
            SYSTICK: SYSTICK {
                _marker: PhantomData,
            },
            MTB: MTB {
                _marker: PhantomData,
            },
            MTBDWT: MTBDWT {
                _marker: PhantomData,
            },
            ROM: ROM {
                _marker: PhantomData,
            },
            MCM: MCM {
                _marker: PhantomData,
            },
            MMAU: MMAU {
                _marker: PhantomData,
            },
            CAU: CAU {
                _marker: PhantomData,
            },
            FGPIOA: FGPIOA {
                _marker: PhantomData,
            },
            FGPIOB: FGPIOB {
                _marker: PhantomData,
            },
            FGPIOC: FGPIOC {
                _marker: PhantomData,
            },
            FGPIOD: FGPIOD {
                _marker: PhantomData,
            },
            FGPIOE: FGPIOE {
                _marker: PhantomData,
            },
            FGPIOF: FGPIOF {
                _marker: PhantomData,
            },
            FGPIOG: FGPIOG {
                _marker: PhantomData,
            },
            FGPIOH: FGPIOH {
                _marker: PhantomData,
            },
            FGPIOI: FGPIOI {
                _marker: PhantomData,
            },
            FGPIOJ: FGPIOJ {
                _marker: PhantomData,
            },
            FGPIOK: FGPIOK {
                _marker: PhantomData,
            },
            FGPIOL: FGPIOL {
                _marker: PhantomData,
            },
            FGPIOM: FGPIOM {
                _marker: PhantomData,
            },
        }
    }
}
