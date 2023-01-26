#![doc = "Peripheral access API for ESP32-C3 microcontrollers (generated using svd2rust v0.28.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next] svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
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
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {
    fn WIFI_MAC();
    fn WIFI_MAC_NMI();
    fn WIFI_PWR();
    fn WIFI_BB();
    fn BT_MAC();
    fn BT_BB();
    fn BT_BB_NMI();
    fn RWBT();
    fn RWBLE();
    fn RWBT_NMI();
    fn RWBLE_NMI();
    fn I2C_MASTER();
    fn APB_CTRL();
    fn UHCI0();
    fn GPIO();
    fn GPIO_NMI();
    fn SPI1();
    fn SPI2();
    fn I2S();
    fn UART0();
    fn UART1();
    fn LEDC();
    fn EFUSE();
    fn TWAI();
    fn USB_SERIAL_JTAG();
    fn RTC_CORE();
    fn RMT();
    fn I2C_EXT0();
    fn TIMER1();
    fn TIMER2();
    fn TG0_T0_LEVEL();
    fn TG0_WDT_LEVEL();
    fn TG1_T0_LEVEL();
    fn TG1_WDT_LEVEL();
    fn CACHE_IA();
    fn SYSTIMER_TARGET0();
    fn SYSTIMER_TARGET1();
    fn SYSTIMER_TARGET2();
    fn SPI_MEM_REJECT_CACHE();
    fn ICACHE_PRELOAD0();
    fn ICACHE_SYNC0();
    fn APB_ADC();
    fn DMA_CH0();
    fn DMA_CH1();
    fn DMA_CH2();
    fn RSA();
    fn AES();
    fn SHA();
    fn FROM_CPU_INTR0();
    fn FROM_CPU_INTR1();
    fn FROM_CPU_INTR2();
    fn FROM_CPU_INTR3();
    fn ASSIST_DEBUG();
    fn DMA_APBPERI_PMS();
    fn CORE0_IRAM0_PMS();
    fn CORE0_DRAM0_PMS();
    fn CORE0_PIF_PMS();
    fn CORE0_PIF_PMS_SIZE();
    fn BAK_PMS_VIOLATE();
    fn CACHE_CORE0_ACS();
}
#[doc(hidden)]
pub union Vector {
    pub _handler: unsafe extern "C" fn(),
    pub _reserved: usize,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[no_mangle]
pub static __EXTERNAL_INTERRUPTS: [Vector; 62] = [
    Vector { _handler: WIFI_MAC },
    Vector {
        _handler: WIFI_MAC_NMI,
    },
    Vector { _handler: WIFI_PWR },
    Vector { _handler: WIFI_BB },
    Vector { _handler: BT_MAC },
    Vector { _handler: BT_BB },
    Vector {
        _handler: BT_BB_NMI,
    },
    Vector { _handler: RWBT },
    Vector { _handler: RWBLE },
    Vector { _handler: RWBT_NMI },
    Vector {
        _handler: RWBLE_NMI,
    },
    Vector {
        _handler: I2C_MASTER,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: APB_CTRL },
    Vector { _handler: UHCI0 },
    Vector { _handler: GPIO },
    Vector { _handler: GPIO_NMI },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: I2S },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: LEDC },
    Vector { _handler: EFUSE },
    Vector { _handler: TWAI },
    Vector {
        _handler: USB_SERIAL_JTAG,
    },
    Vector { _handler: RTC_CORE },
    Vector { _handler: RMT },
    Vector { _handler: I2C_EXT0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector {
        _handler: TG0_T0_LEVEL,
    },
    Vector {
        _handler: TG0_WDT_LEVEL,
    },
    Vector {
        _handler: TG1_T0_LEVEL,
    },
    Vector {
        _handler: TG1_WDT_LEVEL,
    },
    Vector { _handler: CACHE_IA },
    Vector {
        _handler: SYSTIMER_TARGET0,
    },
    Vector {
        _handler: SYSTIMER_TARGET1,
    },
    Vector {
        _handler: SYSTIMER_TARGET2,
    },
    Vector {
        _handler: SPI_MEM_REJECT_CACHE,
    },
    Vector {
        _handler: ICACHE_PRELOAD0,
    },
    Vector {
        _handler: ICACHE_SYNC0,
    },
    Vector { _handler: APB_ADC },
    Vector { _handler: DMA_CH0 },
    Vector { _handler: DMA_CH1 },
    Vector { _handler: DMA_CH2 },
    Vector { _handler: RSA },
    Vector { _handler: AES },
    Vector { _handler: SHA },
    Vector {
        _handler: FROM_CPU_INTR0,
    },
    Vector {
        _handler: FROM_CPU_INTR1,
    },
    Vector {
        _handler: FROM_CPU_INTR2,
    },
    Vector {
        _handler: FROM_CPU_INTR3,
    },
    Vector {
        _handler: ASSIST_DEBUG,
    },
    Vector {
        _handler: DMA_APBPERI_PMS,
    },
    Vector {
        _handler: CORE0_IRAM0_PMS,
    },
    Vector {
        _handler: CORE0_DRAM0_PMS,
    },
    Vector {
        _handler: CORE0_PIF_PMS,
    },
    Vector {
        _handler: CORE0_PIF_PMS_SIZE,
    },
    Vector {
        _handler: BAK_PMS_VIOLATE,
    },
    Vector {
        _handler: CACHE_CORE0_ACS,
    },
];
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[doc = "AES (Advanced Encryption Standard) Accelerator"]
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const aes::RegisterBlock = 0x6003_a000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aes::RegisterBlock {
        Self::PTR
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for AES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AES").finish()
    }
}
#[doc = "AES (Advanced Encryption Standard) Accelerator"]
pub mod aes;
#[doc = "Advanced Peripheral Bus Controller"]
pub struct APB_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for APB_CTRL {}
impl APB_CTRL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const apb_ctrl::RegisterBlock = 0x6002_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const apb_ctrl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for APB_CTRL {
    type Target = apb_ctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for APB_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_CTRL").finish()
    }
}
#[doc = "Advanced Peripheral Bus Controller"]
pub mod apb_ctrl;
#[doc = "Successive Approximation Register Analog to Digital Converter"]
pub struct APB_SARADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for APB_SARADC {}
impl APB_SARADC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const apb_saradc::RegisterBlock = 0x6004_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const apb_saradc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for APB_SARADC {
    type Target = apb_saradc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for APB_SARADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_SARADC").finish()
    }
}
#[doc = "Successive Approximation Register Analog to Digital Converter"]
pub mod apb_saradc;
#[doc = "Debug Assist"]
pub struct ASSIST_DEBUG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ASSIST_DEBUG {}
impl ASSIST_DEBUG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const assist_debug::RegisterBlock = 0x600c_e000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const assist_debug::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ASSIST_DEBUG {
    type Target = assist_debug::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ASSIST_DEBUG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASSIST_DEBUG").finish()
    }
}
#[doc = "Debug Assist"]
pub mod assist_debug;
#[doc = "DMA (Direct Memory Access) Controller"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dma::RegisterBlock = 0x6003_f000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DMA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA").finish()
    }
}
#[doc = "DMA (Direct Memory Access) Controller"]
pub mod dma;
#[doc = "Digital Signature"]
pub struct DS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DS {}
impl DS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ds::RegisterBlock = 0x6003_d000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ds::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DS {
    type Target = ds::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DS").finish()
    }
}
#[doc = "Digital Signature"]
pub mod ds;
#[doc = "eFuse Controller"]
pub struct EFUSE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFUSE {}
impl EFUSE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const efuse::RegisterBlock = 0x6000_8800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const efuse::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EFUSE {
    type Target = efuse::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EFUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSE").finish()
    }
}
#[doc = "eFuse Controller"]
pub mod efuse;
#[doc = "External Memory"]
pub struct EXTMEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTMEM {}
impl EXTMEM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const extmem::RegisterBlock = 0x600c_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const extmem::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EXTMEM {
    type Target = extmem::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EXTMEM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTMEM").finish()
    }
}
#[doc = "External Memory"]
pub mod extmem;
#[doc = "General Purpose Input/Output"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpio::RegisterBlock = 0x6000_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO").finish()
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpio;
#[doc = "Sigma-Delta Modulation"]
pub struct GPIOSD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOSD {}
impl GPIOSD {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpiosd::RegisterBlock = 0x6000_4f00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiosd::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIOSD {
    type Target = gpiosd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIOSD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOSD").finish()
    }
}
#[doc = "Sigma-Delta Modulation"]
pub mod gpiosd;
#[doc = "HMAC (Hash-based Message Authentication Code) Accelerator"]
pub struct HMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HMAC {}
impl HMAC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const hmac::RegisterBlock = 0x6003_e000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hmac::RegisterBlock {
        Self::PTR
    }
}
impl Deref for HMAC {
    type Target = hmac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for HMAC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HMAC").finish()
    }
}
#[doc = "HMAC (Hash-based Message Authentication Code) Accelerator"]
pub mod hmac;
#[doc = "I2C (Inter-Integrated Circuit) Controller"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c0::RegisterBlock = 0x6001_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0").finish()
    }
}
#[doc = "I2C (Inter-Integrated Circuit) Controller"]
pub mod i2c0;
#[doc = "I2S (Inter-IC Sound) Controller"]
pub struct I2S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S {}
impl I2S {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2s::RegisterBlock = 0x6002_d000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2S {
    type Target = i2s::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2S {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S").finish()
    }
}
#[doc = "I2S (Inter-IC Sound) Controller"]
pub mod i2s;
#[doc = "Interrupt Core"]
pub struct INTERRUPT_CORE0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for INTERRUPT_CORE0 {}
impl INTERRUPT_CORE0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const interrupt_core0::RegisterBlock = 0x600c_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const interrupt_core0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for INTERRUPT_CORE0 {
    type Target = interrupt_core0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for INTERRUPT_CORE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT_CORE0").finish()
    }
}
#[doc = "Interrupt Core"]
pub mod interrupt_core0;
#[doc = "Input/Output Multiplexer"]
pub struct IO_MUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IO_MUX {}
impl IO_MUX {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const io_mux::RegisterBlock = 0x6000_9000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const io_mux::RegisterBlock {
        Self::PTR
    }
}
impl Deref for IO_MUX {
    type Target = io_mux::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for IO_MUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_MUX").finish()
    }
}
#[doc = "Input/Output Multiplexer"]
pub mod io_mux;
#[doc = "LED Control PWM (Pulse Width Modulation)"]
pub struct LEDC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEDC {}
impl LEDC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ledc::RegisterBlock = 0x6001_9000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ledc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LEDC {
    type Target = ledc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LEDC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LEDC").finish()
    }
}
#[doc = "LED Control PWM (Pulse Width Modulation)"]
pub mod ledc;
#[doc = "Remote Control Peripheral"]
pub struct RMT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RMT {}
impl RMT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rmt::RegisterBlock = 0x6001_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rmt::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RMT {
    type Target = rmt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RMT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMT").finish()
    }
}
#[doc = "Remote Control Peripheral"]
pub mod rmt;
#[doc = "Hardware random number generator"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rng::RegisterBlock = 0x6002_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RNG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG").finish()
    }
}
#[doc = "Hardware random number generator"]
pub mod rng;
#[doc = "RSA (Rivest Shamir Adleman) Accelerator"]
pub struct RSA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSA {}
impl RSA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rsa::RegisterBlock = 0x6003_c000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rsa::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RSA {
    type Target = rsa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RSA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSA").finish()
    }
}
#[doc = "RSA (Rivest Shamir Adleman) Accelerator"]
pub mod rsa;
#[doc = "Real-Time Clock Control"]
pub struct RTC_CNTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC_CNTL {}
impl RTC_CNTL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc_cntl::RegisterBlock = 0x6000_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc_cntl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTC_CNTL {
    type Target = rtc_cntl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC_CNTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_CNTL").finish()
    }
}
#[doc = "Real-Time Clock Control"]
pub mod rtc_cntl;
#[doc = "Sensitive"]
pub struct SENSITIVE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SENSITIVE {}
impl SENSITIVE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sensitive::RegisterBlock = 0x600c_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sensitive::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SENSITIVE {
    type Target = sensitive::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SENSITIVE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SENSITIVE").finish()
    }
}
#[doc = "Sensitive"]
pub mod sensitive;
#[doc = "SHA (Secure Hash Algorithm) Accelerator"]
pub struct SHA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SHA {}
impl SHA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sha::RegisterBlock = 0x6003_b000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sha::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SHA {
    type Target = sha::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SHA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHA").finish()
    }
}
#[doc = "SHA (Secure Hash Algorithm) Accelerator"]
pub mod sha;
#[doc = "SPI (Serial Peripheral Interface) Controller"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi0::RegisterBlock = 0x6000_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI0").finish()
    }
}
#[doc = "SPI (Serial Peripheral Interface) Controller"]
pub mod spi0;
#[doc = "SPI (Serial Peripheral Interface) Controller"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi1::RegisterBlock = 0x6000_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI1").finish()
    }
}
#[doc = "SPI (Serial Peripheral Interface) Controller"]
pub mod spi1;
#[doc = "SPI (Serial Peripheral Interface) Controller"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi2::RegisterBlock = 0x6002_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI2 {
    type Target = spi2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI2").finish()
    }
}
#[doc = "SPI (Serial Peripheral Interface) Controller"]
pub mod spi2;
#[doc = "System"]
pub struct SYSTEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEM {}
impl SYSTEM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const system::RegisterBlock = 0x600c_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const system::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYSTEM {
    type Target = system::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYSTEM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTEM").finish()
    }
}
#[doc = "System"]
pub mod system;
#[doc = "System Timer"]
pub struct SYSTIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTIMER {}
impl SYSTIMER {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const systimer::RegisterBlock = 0x6002_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const systimer::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYSTIMER {
    type Target = systimer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYSTIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTIMER").finish()
    }
}
#[doc = "System Timer"]
pub mod systimer;
#[doc = "Timer Group"]
pub struct TIMG0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMG0 {}
impl TIMG0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timg0::RegisterBlock = 0x6001_f000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timg0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMG0 {
    type Target = timg0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMG0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMG0").finish()
    }
}
#[doc = "Timer Group"]
pub mod timg0;
#[doc = "Timer Group"]
pub struct TIMG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMG1 {}
impl TIMG1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timg0::RegisterBlock = 0x6002_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timg0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMG1 {
    type Target = timg0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMG1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMG1").finish()
    }
}
#[doc = "Timer Group"]
pub use self::timg0 as timg1;
#[doc = "Two-Wire Automotive Interface"]
pub struct TWAI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWAI {}
impl TWAI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const twai::RegisterBlock = 0x6002_b000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twai::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TWAI {
    type Target = twai::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TWAI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWAI").finish()
    }
}
#[doc = "Two-Wire Automotive Interface"]
pub mod twai;
#[doc = "UART (Universal Asynchronous Receiver-Transmitter) Controller"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart0::RegisterBlock = 0x6000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0").finish()
    }
}
#[doc = "UART (Universal Asynchronous Receiver-Transmitter) Controller"]
pub mod uart0;
#[doc = "UART (Universal Asynchronous Receiver-Transmitter) Controller"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart0::RegisterBlock = 0x6001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART1").finish()
    }
}
#[doc = "UART (Universal Asynchronous Receiver-Transmitter) Controller"]
pub use self::uart0 as uart1;
#[doc = "Universal Host Controller Interface"]
pub struct UHCI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UHCI0 {}
impl UHCI0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uhci0::RegisterBlock = 0x6001_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uhci0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UHCI0 {
    type Target = uhci0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UHCI0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UHCI0").finish()
    }
}
#[doc = "Universal Host Controller Interface"]
pub mod uhci0;
#[doc = "Universal Host Controller Interface"]
pub struct UHCI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UHCI1 {}
impl UHCI1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uhci0::RegisterBlock = 0x6000_c000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uhci0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UHCI1 {
    type Target = uhci0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UHCI1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UHCI1").finish()
    }
}
#[doc = "Universal Host Controller Interface"]
pub use self::uhci0 as uhci1;
#[doc = "Full-speed USB Serial/JTAG Controller"]
pub struct USB_DEVICE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB_DEVICE {}
impl USB_DEVICE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usb_device::RegisterBlock = 0x6004_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb_device::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USB_DEVICE {
    type Target = usb_device::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USB_DEVICE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_DEVICE").finish()
    }
}
#[doc = "Full-speed USB Serial/JTAG Controller"]
pub mod usb_device;
#[doc = "XTS-AES-128 Flash Encryption"]
pub struct XTS_AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XTS_AES {}
impl XTS_AES {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const xts_aes::RegisterBlock = 0x600c_c000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const xts_aes::RegisterBlock {
        Self::PTR
    }
}
impl Deref for XTS_AES {
    type Target = xts_aes::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for XTS_AES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTS_AES").finish()
    }
}
#[doc = "XTS-AES-128 Flash Encryption"]
pub mod xts_aes;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "APB_CTRL"]
    pub APB_CTRL: APB_CTRL,
    #[doc = "APB_SARADC"]
    pub APB_SARADC: APB_SARADC,
    #[doc = "ASSIST_DEBUG"]
    pub ASSIST_DEBUG: ASSIST_DEBUG,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "DS"]
    pub DS: DS,
    #[doc = "EFUSE"]
    pub EFUSE: EFUSE,
    #[doc = "EXTMEM"]
    pub EXTMEM: EXTMEM,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "GPIOSD"]
    pub GPIOSD: GPIOSD,
    #[doc = "HMAC"]
    pub HMAC: HMAC,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2S"]
    pub I2S: I2S,
    #[doc = "INTERRUPT_CORE0"]
    pub INTERRUPT_CORE0: INTERRUPT_CORE0,
    #[doc = "IO_MUX"]
    pub IO_MUX: IO_MUX,
    #[doc = "LEDC"]
    pub LEDC: LEDC,
    #[doc = "RMT"]
    pub RMT: RMT,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "RSA"]
    pub RSA: RSA,
    #[doc = "RTC_CNTL"]
    pub RTC_CNTL: RTC_CNTL,
    #[doc = "SENSITIVE"]
    pub SENSITIVE: SENSITIVE,
    #[doc = "SHA"]
    pub SHA: SHA,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SYSTEM"]
    pub SYSTEM: SYSTEM,
    #[doc = "SYSTIMER"]
    pub SYSTIMER: SYSTIMER,
    #[doc = "TIMG0"]
    pub TIMG0: TIMG0,
    #[doc = "TIMG1"]
    pub TIMG1: TIMG1,
    #[doc = "TWAI"]
    pub TWAI: TWAI,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UHCI0"]
    pub UHCI0: UHCI0,
    #[doc = "UHCI1"]
    pub UHCI1: UHCI1,
    #[doc = "USB_DEVICE"]
    pub USB_DEVICE: USB_DEVICE,
    #[doc = "XTS_AES"]
    pub XTS_AES: XTS_AES,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AES: AES {
                _marker: PhantomData,
            },
            APB_CTRL: APB_CTRL {
                _marker: PhantomData,
            },
            APB_SARADC: APB_SARADC {
                _marker: PhantomData,
            },
            ASSIST_DEBUG: ASSIST_DEBUG {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            DS: DS {
                _marker: PhantomData,
            },
            EFUSE: EFUSE {
                _marker: PhantomData,
            },
            EXTMEM: EXTMEM {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            GPIOSD: GPIOSD {
                _marker: PhantomData,
            },
            HMAC: HMAC {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2S: I2S {
                _marker: PhantomData,
            },
            INTERRUPT_CORE0: INTERRUPT_CORE0 {
                _marker: PhantomData,
            },
            IO_MUX: IO_MUX {
                _marker: PhantomData,
            },
            LEDC: LEDC {
                _marker: PhantomData,
            },
            RMT: RMT {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            RSA: RSA {
                _marker: PhantomData,
            },
            RTC_CNTL: RTC_CNTL {
                _marker: PhantomData,
            },
            SENSITIVE: SENSITIVE {
                _marker: PhantomData,
            },
            SHA: SHA {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SYSTEM: SYSTEM {
                _marker: PhantomData,
            },
            SYSTIMER: SYSTIMER {
                _marker: PhantomData,
            },
            TIMG0: TIMG0 {
                _marker: PhantomData,
            },
            TIMG1: TIMG1 {
                _marker: PhantomData,
            },
            TWAI: TWAI {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            UHCI0: UHCI0 {
                _marker: PhantomData,
            },
            UHCI1: UHCI1 {
                _marker: PhantomData,
            },
            USB_DEVICE: USB_DEVICE {
                _marker: PhantomData,
            },
            XTS_AES: XTS_AES {
                _marker: PhantomData,
            },
        }
    }
}
