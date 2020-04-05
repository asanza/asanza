#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LCD General Control Register"]
    pub gcr: GCR,
    #[doc = "0x04 - LCD Auxiliary Register"]
    pub ar: AR,
    #[doc = "0x08 - LCD Fault Detect Control Register"]
    pub fdcr: FDCR,
    #[doc = "0x0c - LCD Fault Detect Status Register"]
    pub fdsr: FDSR,
    #[doc = "0x10 - LCD Pin Enable register"]
    pub penl: PEN,
    #[doc = "0x14 - LCD Pin Enable register"]
    pub penh: PEN,
    #[doc = "0x18 - LCD Back Plane Enable register"]
    pub bpenl: BPEN,
    #[doc = "0x1c - LCD Back Plane Enable register"]
    pub bpenh: BPEN,
    _reserved_8_wf0: [u8; 4usize],
    _reserved_9_wf4: [u8; 4usize],
    _reserved_10_wf8: [u8; 4usize],
    _reserved_11_wf1: [u8; 4usize],
    _reserved_12_wf1: [u8; 4usize],
    _reserved_13_wf2: [u8; 4usize],
    _reserved_14_wf2: [u8; 4usize],
    _reserved_15_wf28: [u8; 4usize],
    _reserved_16_wf3: [u8; 4usize],
    _reserved_17_wf3: [u8; 4usize],
    _reserved_18_wf4: [u8; 4usize],
    _reserved_19_wf4: [u8; 4usize],
    _reserved_20_wf48: [u8; 4usize],
    _reserved_21_wf5: [u8; 4usize],
    _reserved_22_wf5: [u8; 4usize],
    _reserved_23_wf6: [u8; 4usize],
}
impl RegisterBlock {
    #[doc = "0x20 - LCD Waveform Register 0."]
    #[inline(always)]
    pub fn wf0(&self) -> &WF0 {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const WF0) }
    }
    #[doc = "0x20 - LCD Waveform Register 0."]
    #[inline(always)]
    pub fn wf0_mut(&self) -> &mut WF0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut WF0) }
    }
    #[doc = "0x20 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf3to0(&self) -> &WF3TO0 {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const WF3TO0) }
    }
    #[doc = "0x20 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf3to0_mut(&self) -> &mut WF3TO0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut WF3TO0) }
    }
    #[doc = "0x21 - LCD Waveform Register 1."]
    #[inline(always)]
    pub fn wf1(&self) -> &WF1 {
        unsafe { &*(((self as *const Self) as *const u8).add(33usize) as *const WF1) }
    }
    #[doc = "0x21 - LCD Waveform Register 1."]
    #[inline(always)]
    pub fn wf1_mut(&self) -> &mut WF1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(33usize) as *mut WF1) }
    }
    #[doc = "0x22 - LCD Waveform Register 2."]
    #[inline(always)]
    pub fn wf2(&self) -> &WF2 {
        unsafe { &*(((self as *const Self) as *const u8).add(34usize) as *const WF2) }
    }
    #[doc = "0x22 - LCD Waveform Register 2."]
    #[inline(always)]
    pub fn wf2_mut(&self) -> &mut WF2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(34usize) as *mut WF2) }
    }
    #[doc = "0x23 - LCD Waveform Register 3."]
    #[inline(always)]
    pub fn wf3(&self) -> &WF3 {
        unsafe { &*(((self as *const Self) as *const u8).add(35usize) as *const WF3) }
    }
    #[doc = "0x23 - LCD Waveform Register 3."]
    #[inline(always)]
    pub fn wf3_mut(&self) -> &mut WF3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(35usize) as *mut WF3) }
    }
    #[doc = "0x24 - LCD Waveform Register 4."]
    #[inline(always)]
    pub fn wf4(&self) -> &WF4 {
        unsafe { &*(((self as *const Self) as *const u8).add(36usize) as *const WF4) }
    }
    #[doc = "0x24 - LCD Waveform Register 4."]
    #[inline(always)]
    pub fn wf4_mut(&self) -> &mut WF4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(36usize) as *mut WF4) }
    }
    #[doc = "0x24 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf7to4(&self) -> &WF7TO4 {
        unsafe { &*(((self as *const Self) as *const u8).add(36usize) as *const WF7TO4) }
    }
    #[doc = "0x24 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf7to4_mut(&self) -> &mut WF7TO4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(36usize) as *mut WF7TO4) }
    }
    #[doc = "0x25 - LCD Waveform Register 5."]
    #[inline(always)]
    pub fn wf5(&self) -> &WF5 {
        unsafe { &*(((self as *const Self) as *const u8).add(37usize) as *const WF5) }
    }
    #[doc = "0x25 - LCD Waveform Register 5."]
    #[inline(always)]
    pub fn wf5_mut(&self) -> &mut WF5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(37usize) as *mut WF5) }
    }
    #[doc = "0x26 - LCD Waveform Register 6."]
    #[inline(always)]
    pub fn wf6(&self) -> &WF6 {
        unsafe { &*(((self as *const Self) as *const u8).add(38usize) as *const WF6) }
    }
    #[doc = "0x26 - LCD Waveform Register 6."]
    #[inline(always)]
    pub fn wf6_mut(&self) -> &mut WF6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(38usize) as *mut WF6) }
    }
    #[doc = "0x27 - LCD Waveform Register 7."]
    #[inline(always)]
    pub fn wf7(&self) -> &WF7 {
        unsafe { &*(((self as *const Self) as *const u8).add(39usize) as *const WF7) }
    }
    #[doc = "0x27 - LCD Waveform Register 7."]
    #[inline(always)]
    pub fn wf7_mut(&self) -> &mut WF7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(39usize) as *mut WF7) }
    }
    #[doc = "0x28 - LCD Waveform Register 8."]
    #[inline(always)]
    pub fn wf8(&self) -> &WF8 {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const WF8) }
    }
    #[doc = "0x28 - LCD Waveform Register 8."]
    #[inline(always)]
    pub fn wf8_mut(&self) -> &mut WF8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut WF8) }
    }
    #[doc = "0x28 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf11to8(&self) -> &WF11TO8 {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const WF11TO8) }
    }
    #[doc = "0x28 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf11to8_mut(&self) -> &mut WF11TO8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut WF11TO8) }
    }
    #[doc = "0x29 - LCD Waveform Register 9."]
    #[inline(always)]
    pub fn wf9(&self) -> &WF9 {
        unsafe { &*(((self as *const Self) as *const u8).add(41usize) as *const WF9) }
    }
    #[doc = "0x29 - LCD Waveform Register 9."]
    #[inline(always)]
    pub fn wf9_mut(&self) -> &mut WF9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(41usize) as *mut WF9) }
    }
    #[doc = "0x2a - LCD Waveform Register 10."]
    #[inline(always)]
    pub fn wf10(&self) -> &WF10 {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const WF10) }
    }
    #[doc = "0x2a - LCD Waveform Register 10."]
    #[inline(always)]
    pub fn wf10_mut(&self) -> &mut WF10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(42usize) as *mut WF10) }
    }
    #[doc = "0x2b - LCD Waveform Register 11."]
    #[inline(always)]
    pub fn wf11(&self) -> &WF11 {
        unsafe { &*(((self as *const Self) as *const u8).add(43usize) as *const WF11) }
    }
    #[doc = "0x2b - LCD Waveform Register 11."]
    #[inline(always)]
    pub fn wf11_mut(&self) -> &mut WF11 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(43usize) as *mut WF11) }
    }
    #[doc = "0x2c - LCD Waveform Register 12."]
    #[inline(always)]
    pub fn wf12(&self) -> &WF12 {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const WF12) }
    }
    #[doc = "0x2c - LCD Waveform Register 12."]
    #[inline(always)]
    pub fn wf12_mut(&self) -> &mut WF12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut WF12) }
    }
    #[doc = "0x2c - LCD Waveform register"]
    #[inline(always)]
    pub fn wf15to12(&self) -> &WF15TO12 {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const WF15TO12) }
    }
    #[doc = "0x2c - LCD Waveform register"]
    #[inline(always)]
    pub fn wf15to12_mut(&self) -> &mut WF15TO12 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut WF15TO12) }
    }
    #[doc = "0x2d - LCD Waveform Register 13."]
    #[inline(always)]
    pub fn wf13(&self) -> &WF13 {
        unsafe { &*(((self as *const Self) as *const u8).add(45usize) as *const WF13) }
    }
    #[doc = "0x2d - LCD Waveform Register 13."]
    #[inline(always)]
    pub fn wf13_mut(&self) -> &mut WF13 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(45usize) as *mut WF13) }
    }
    #[doc = "0x2e - LCD Waveform Register 14."]
    #[inline(always)]
    pub fn wf14(&self) -> &WF14 {
        unsafe { &*(((self as *const Self) as *const u8).add(46usize) as *const WF14) }
    }
    #[doc = "0x2e - LCD Waveform Register 14."]
    #[inline(always)]
    pub fn wf14_mut(&self) -> &mut WF14 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(46usize) as *mut WF14) }
    }
    #[doc = "0x2f - LCD Waveform Register 15."]
    #[inline(always)]
    pub fn wf15(&self) -> &WF15 {
        unsafe { &*(((self as *const Self) as *const u8).add(47usize) as *const WF15) }
    }
    #[doc = "0x2f - LCD Waveform Register 15."]
    #[inline(always)]
    pub fn wf15_mut(&self) -> &mut WF15 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(47usize) as *mut WF15) }
    }
    #[doc = "0x30 - LCD Waveform Register 16."]
    #[inline(always)]
    pub fn wf16(&self) -> &WF16 {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const WF16) }
    }
    #[doc = "0x30 - LCD Waveform Register 16."]
    #[inline(always)]
    pub fn wf16_mut(&self) -> &mut WF16 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(48usize) as *mut WF16) }
    }
    #[doc = "0x30 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf19to16(&self) -> &WF19TO16 {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const WF19TO16) }
    }
    #[doc = "0x30 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf19to16_mut(&self) -> &mut WF19TO16 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(48usize) as *mut WF19TO16) }
    }
    #[doc = "0x31 - LCD Waveform Register 17."]
    #[inline(always)]
    pub fn wf17(&self) -> &WF17 {
        unsafe { &*(((self as *const Self) as *const u8).add(49usize) as *const WF17) }
    }
    #[doc = "0x31 - LCD Waveform Register 17."]
    #[inline(always)]
    pub fn wf17_mut(&self) -> &mut WF17 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(49usize) as *mut WF17) }
    }
    #[doc = "0x32 - LCD Waveform Register 18."]
    #[inline(always)]
    pub fn wf18(&self) -> &WF18 {
        unsafe { &*(((self as *const Self) as *const u8).add(50usize) as *const WF18) }
    }
    #[doc = "0x32 - LCD Waveform Register 18."]
    #[inline(always)]
    pub fn wf18_mut(&self) -> &mut WF18 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(50usize) as *mut WF18) }
    }
    #[doc = "0x33 - LCD Waveform Register 19."]
    #[inline(always)]
    pub fn wf19(&self) -> &WF19 {
        unsafe { &*(((self as *const Self) as *const u8).add(51usize) as *const WF19) }
    }
    #[doc = "0x33 - LCD Waveform Register 19."]
    #[inline(always)]
    pub fn wf19_mut(&self) -> &mut WF19 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(51usize) as *mut WF19) }
    }
    #[doc = "0x34 - LCD Waveform Register 20."]
    #[inline(always)]
    pub fn wf20(&self) -> &WF20 {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const WF20) }
    }
    #[doc = "0x34 - LCD Waveform Register 20."]
    #[inline(always)]
    pub fn wf20_mut(&self) -> &mut WF20 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut WF20) }
    }
    #[doc = "0x34 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf23to20(&self) -> &WF23TO20 {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const WF23TO20) }
    }
    #[doc = "0x34 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf23to20_mut(&self) -> &mut WF23TO20 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut WF23TO20) }
    }
    #[doc = "0x35 - LCD Waveform Register 21."]
    #[inline(always)]
    pub fn wf21(&self) -> &WF21 {
        unsafe { &*(((self as *const Self) as *const u8).add(53usize) as *const WF21) }
    }
    #[doc = "0x35 - LCD Waveform Register 21."]
    #[inline(always)]
    pub fn wf21_mut(&self) -> &mut WF21 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(53usize) as *mut WF21) }
    }
    #[doc = "0x36 - LCD Waveform Register 22."]
    #[inline(always)]
    pub fn wf22(&self) -> &WF22 {
        unsafe { &*(((self as *const Self) as *const u8).add(54usize) as *const WF22) }
    }
    #[doc = "0x36 - LCD Waveform Register 22."]
    #[inline(always)]
    pub fn wf22_mut(&self) -> &mut WF22 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(54usize) as *mut WF22) }
    }
    #[doc = "0x37 - LCD Waveform Register 23."]
    #[inline(always)]
    pub fn wf23(&self) -> &WF23 {
        unsafe { &*(((self as *const Self) as *const u8).add(55usize) as *const WF23) }
    }
    #[doc = "0x37 - LCD Waveform Register 23."]
    #[inline(always)]
    pub fn wf23_mut(&self) -> &mut WF23 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(55usize) as *mut WF23) }
    }
    #[doc = "0x38 - LCD Waveform Register 24."]
    #[inline(always)]
    pub fn wf24(&self) -> &WF24 {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const WF24) }
    }
    #[doc = "0x38 - LCD Waveform Register 24."]
    #[inline(always)]
    pub fn wf24_mut(&self) -> &mut WF24 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(56usize) as *mut WF24) }
    }
    #[doc = "0x38 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf27to24(&self) -> &WF27TO24 {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const WF27TO24) }
    }
    #[doc = "0x38 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf27to24_mut(&self) -> &mut WF27TO24 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(56usize) as *mut WF27TO24) }
    }
    #[doc = "0x39 - LCD Waveform Register 25."]
    #[inline(always)]
    pub fn wf25(&self) -> &WF25 {
        unsafe { &*(((self as *const Self) as *const u8).add(57usize) as *const WF25) }
    }
    #[doc = "0x39 - LCD Waveform Register 25."]
    #[inline(always)]
    pub fn wf25_mut(&self) -> &mut WF25 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(57usize) as *mut WF25) }
    }
    #[doc = "0x3a - LCD Waveform Register 26."]
    #[inline(always)]
    pub fn wf26(&self) -> &WF26 {
        unsafe { &*(((self as *const Self) as *const u8).add(58usize) as *const WF26) }
    }
    #[doc = "0x3a - LCD Waveform Register 26."]
    #[inline(always)]
    pub fn wf26_mut(&self) -> &mut WF26 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(58usize) as *mut WF26) }
    }
    #[doc = "0x3b - LCD Waveform Register 27."]
    #[inline(always)]
    pub fn wf27(&self) -> &WF27 {
        unsafe { &*(((self as *const Self) as *const u8).add(59usize) as *const WF27) }
    }
    #[doc = "0x3b - LCD Waveform Register 27."]
    #[inline(always)]
    pub fn wf27_mut(&self) -> &mut WF27 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(59usize) as *mut WF27) }
    }
    #[doc = "0x3c - LCD Waveform Register 28."]
    #[inline(always)]
    pub fn wf28(&self) -> &WF28 {
        unsafe { &*(((self as *const Self) as *const u8).add(60usize) as *const WF28) }
    }
    #[doc = "0x3c - LCD Waveform Register 28."]
    #[inline(always)]
    pub fn wf28_mut(&self) -> &mut WF28 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(60usize) as *mut WF28) }
    }
    #[doc = "0x3c - LCD Waveform register"]
    #[inline(always)]
    pub fn wf31to28(&self) -> &WF31TO28 {
        unsafe { &*(((self as *const Self) as *const u8).add(60usize) as *const WF31TO28) }
    }
    #[doc = "0x3c - LCD Waveform register"]
    #[inline(always)]
    pub fn wf31to28_mut(&self) -> &mut WF31TO28 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(60usize) as *mut WF31TO28) }
    }
    #[doc = "0x3d - LCD Waveform Register 29."]
    #[inline(always)]
    pub fn wf29(&self) -> &WF29 {
        unsafe { &*(((self as *const Self) as *const u8).add(61usize) as *const WF29) }
    }
    #[doc = "0x3d - LCD Waveform Register 29."]
    #[inline(always)]
    pub fn wf29_mut(&self) -> &mut WF29 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(61usize) as *mut WF29) }
    }
    #[doc = "0x3e - LCD Waveform Register 30."]
    #[inline(always)]
    pub fn wf30(&self) -> &WF30 {
        unsafe { &*(((self as *const Self) as *const u8).add(62usize) as *const WF30) }
    }
    #[doc = "0x3e - LCD Waveform Register 30."]
    #[inline(always)]
    pub fn wf30_mut(&self) -> &mut WF30 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(62usize) as *mut WF30) }
    }
    #[doc = "0x3f - LCD Waveform Register 31."]
    #[inline(always)]
    pub fn wf31(&self) -> &WF31 {
        unsafe { &*(((self as *const Self) as *const u8).add(63usize) as *const WF31) }
    }
    #[doc = "0x3f - LCD Waveform Register 31."]
    #[inline(always)]
    pub fn wf31_mut(&self) -> &mut WF31 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(63usize) as *mut WF31) }
    }
    #[doc = "0x40 - LCD Waveform Register 32."]
    #[inline(always)]
    pub fn wf32(&self) -> &WF32 {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const WF32) }
    }
    #[doc = "0x40 - LCD Waveform Register 32."]
    #[inline(always)]
    pub fn wf32_mut(&self) -> &mut WF32 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut WF32) }
    }
    #[doc = "0x40 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf35to32(&self) -> &WF35TO32 {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const WF35TO32) }
    }
    #[doc = "0x40 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf35to32_mut(&self) -> &mut WF35TO32 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut WF35TO32) }
    }
    #[doc = "0x41 - LCD Waveform Register 33."]
    #[inline(always)]
    pub fn wf33(&self) -> &WF33 {
        unsafe { &*(((self as *const Self) as *const u8).add(65usize) as *const WF33) }
    }
    #[doc = "0x41 - LCD Waveform Register 33."]
    #[inline(always)]
    pub fn wf33_mut(&self) -> &mut WF33 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(65usize) as *mut WF33) }
    }
    #[doc = "0x42 - LCD Waveform Register 34."]
    #[inline(always)]
    pub fn wf34(&self) -> &WF34 {
        unsafe { &*(((self as *const Self) as *const u8).add(66usize) as *const WF34) }
    }
    #[doc = "0x42 - LCD Waveform Register 34."]
    #[inline(always)]
    pub fn wf34_mut(&self) -> &mut WF34 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(66usize) as *mut WF34) }
    }
    #[doc = "0x43 - LCD Waveform Register 35."]
    #[inline(always)]
    pub fn wf35(&self) -> &WF35 {
        unsafe { &*(((self as *const Self) as *const u8).add(67usize) as *const WF35) }
    }
    #[doc = "0x43 - LCD Waveform Register 35."]
    #[inline(always)]
    pub fn wf35_mut(&self) -> &mut WF35 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(67usize) as *mut WF35) }
    }
    #[doc = "0x44 - LCD Waveform Register 36."]
    #[inline(always)]
    pub fn wf36(&self) -> &WF36 {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const WF36) }
    }
    #[doc = "0x44 - LCD Waveform Register 36."]
    #[inline(always)]
    pub fn wf36_mut(&self) -> &mut WF36 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut WF36) }
    }
    #[doc = "0x44 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf39to36(&self) -> &WF39TO36 {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const WF39TO36) }
    }
    #[doc = "0x44 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf39to36_mut(&self) -> &mut WF39TO36 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(68usize) as *mut WF39TO36) }
    }
    #[doc = "0x45 - LCD Waveform Register 37."]
    #[inline(always)]
    pub fn wf37(&self) -> &WF37 {
        unsafe { &*(((self as *const Self) as *const u8).add(69usize) as *const WF37) }
    }
    #[doc = "0x45 - LCD Waveform Register 37."]
    #[inline(always)]
    pub fn wf37_mut(&self) -> &mut WF37 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(69usize) as *mut WF37) }
    }
    #[doc = "0x46 - LCD Waveform Register 38."]
    #[inline(always)]
    pub fn wf38(&self) -> &WF38 {
        unsafe { &*(((self as *const Self) as *const u8).add(70usize) as *const WF38) }
    }
    #[doc = "0x46 - LCD Waveform Register 38."]
    #[inline(always)]
    pub fn wf38_mut(&self) -> &mut WF38 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(70usize) as *mut WF38) }
    }
    #[doc = "0x47 - LCD Waveform Register 39."]
    #[inline(always)]
    pub fn wf39(&self) -> &WF39 {
        unsafe { &*(((self as *const Self) as *const u8).add(71usize) as *const WF39) }
    }
    #[doc = "0x47 - LCD Waveform Register 39."]
    #[inline(always)]
    pub fn wf39_mut(&self) -> &mut WF39 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(71usize) as *mut WF39) }
    }
    #[doc = "0x48 - LCD Waveform Register 40."]
    #[inline(always)]
    pub fn wf40(&self) -> &WF40 {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const WF40) }
    }
    #[doc = "0x48 - LCD Waveform Register 40."]
    #[inline(always)]
    pub fn wf40_mut(&self) -> &mut WF40 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut WF40) }
    }
    #[doc = "0x48 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf43to40(&self) -> &WF43TO40 {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const WF43TO40) }
    }
    #[doc = "0x48 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf43to40_mut(&self) -> &mut WF43TO40 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut WF43TO40) }
    }
    #[doc = "0x49 - LCD Waveform Register 41."]
    #[inline(always)]
    pub fn wf41(&self) -> &WF41 {
        unsafe { &*(((self as *const Self) as *const u8).add(73usize) as *const WF41) }
    }
    #[doc = "0x49 - LCD Waveform Register 41."]
    #[inline(always)]
    pub fn wf41_mut(&self) -> &mut WF41 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(73usize) as *mut WF41) }
    }
    #[doc = "0x4a - LCD Waveform Register 42."]
    #[inline(always)]
    pub fn wf42(&self) -> &WF42 {
        unsafe { &*(((self as *const Self) as *const u8).add(74usize) as *const WF42) }
    }
    #[doc = "0x4a - LCD Waveform Register 42."]
    #[inline(always)]
    pub fn wf42_mut(&self) -> &mut WF42 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(74usize) as *mut WF42) }
    }
    #[doc = "0x4b - LCD Waveform Register 43."]
    #[inline(always)]
    pub fn wf43(&self) -> &WF43 {
        unsafe { &*(((self as *const Self) as *const u8).add(75usize) as *const WF43) }
    }
    #[doc = "0x4b - LCD Waveform Register 43."]
    #[inline(always)]
    pub fn wf43_mut(&self) -> &mut WF43 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(75usize) as *mut WF43) }
    }
    #[doc = "0x4c - LCD Waveform Register 44."]
    #[inline(always)]
    pub fn wf44(&self) -> &WF44 {
        unsafe { &*(((self as *const Self) as *const u8).add(76usize) as *const WF44) }
    }
    #[doc = "0x4c - LCD Waveform Register 44."]
    #[inline(always)]
    pub fn wf44_mut(&self) -> &mut WF44 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(76usize) as *mut WF44) }
    }
    #[doc = "0x4c - LCD Waveform register"]
    #[inline(always)]
    pub fn wf47to44(&self) -> &WF47TO44 {
        unsafe { &*(((self as *const Self) as *const u8).add(76usize) as *const WF47TO44) }
    }
    #[doc = "0x4c - LCD Waveform register"]
    #[inline(always)]
    pub fn wf47to44_mut(&self) -> &mut WF47TO44 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(76usize) as *mut WF47TO44) }
    }
    #[doc = "0x4d - LCD Waveform Register 45."]
    #[inline(always)]
    pub fn wf45(&self) -> &WF45 {
        unsafe { &*(((self as *const Self) as *const u8).add(77usize) as *const WF45) }
    }
    #[doc = "0x4d - LCD Waveform Register 45."]
    #[inline(always)]
    pub fn wf45_mut(&self) -> &mut WF45 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(77usize) as *mut WF45) }
    }
    #[doc = "0x4e - LCD Waveform Register 46."]
    #[inline(always)]
    pub fn wf46(&self) -> &WF46 {
        unsafe { &*(((self as *const Self) as *const u8).add(78usize) as *const WF46) }
    }
    #[doc = "0x4e - LCD Waveform Register 46."]
    #[inline(always)]
    pub fn wf46_mut(&self) -> &mut WF46 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(78usize) as *mut WF46) }
    }
    #[doc = "0x4f - LCD Waveform Register 47."]
    #[inline(always)]
    pub fn wf47(&self) -> &WF47 {
        unsafe { &*(((self as *const Self) as *const u8).add(79usize) as *const WF47) }
    }
    #[doc = "0x4f - LCD Waveform Register 47."]
    #[inline(always)]
    pub fn wf47_mut(&self) -> &mut WF47 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(79usize) as *mut WF47) }
    }
    #[doc = "0x50 - LCD Waveform Register 48."]
    #[inline(always)]
    pub fn wf48(&self) -> &WF48 {
        unsafe { &*(((self as *const Self) as *const u8).add(80usize) as *const WF48) }
    }
    #[doc = "0x50 - LCD Waveform Register 48."]
    #[inline(always)]
    pub fn wf48_mut(&self) -> &mut WF48 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(80usize) as *mut WF48) }
    }
    #[doc = "0x50 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf51to48(&self) -> &WF51TO48 {
        unsafe { &*(((self as *const Self) as *const u8).add(80usize) as *const WF51TO48) }
    }
    #[doc = "0x50 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf51to48_mut(&self) -> &mut WF51TO48 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(80usize) as *mut WF51TO48) }
    }
    #[doc = "0x51 - LCD Waveform Register 49."]
    #[inline(always)]
    pub fn wf49(&self) -> &WF49 {
        unsafe { &*(((self as *const Self) as *const u8).add(81usize) as *const WF49) }
    }
    #[doc = "0x51 - LCD Waveform Register 49."]
    #[inline(always)]
    pub fn wf49_mut(&self) -> &mut WF49 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(81usize) as *mut WF49) }
    }
    #[doc = "0x52 - LCD Waveform Register 50."]
    #[inline(always)]
    pub fn wf50(&self) -> &WF50 {
        unsafe { &*(((self as *const Self) as *const u8).add(82usize) as *const WF50) }
    }
    #[doc = "0x52 - LCD Waveform Register 50."]
    #[inline(always)]
    pub fn wf50_mut(&self) -> &mut WF50 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(82usize) as *mut WF50) }
    }
    #[doc = "0x53 - LCD Waveform Register 51."]
    #[inline(always)]
    pub fn wf51(&self) -> &WF51 {
        unsafe { &*(((self as *const Self) as *const u8).add(83usize) as *const WF51) }
    }
    #[doc = "0x53 - LCD Waveform Register 51."]
    #[inline(always)]
    pub fn wf51_mut(&self) -> &mut WF51 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(83usize) as *mut WF51) }
    }
    #[doc = "0x54 - LCD Waveform Register 52."]
    #[inline(always)]
    pub fn wf52(&self) -> &WF52 {
        unsafe { &*(((self as *const Self) as *const u8).add(84usize) as *const WF52) }
    }
    #[doc = "0x54 - LCD Waveform Register 52."]
    #[inline(always)]
    pub fn wf52_mut(&self) -> &mut WF52 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(84usize) as *mut WF52) }
    }
    #[doc = "0x54 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf55to52(&self) -> &WF55TO52 {
        unsafe { &*(((self as *const Self) as *const u8).add(84usize) as *const WF55TO52) }
    }
    #[doc = "0x54 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf55to52_mut(&self) -> &mut WF55TO52 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(84usize) as *mut WF55TO52) }
    }
    #[doc = "0x55 - LCD Waveform Register 53."]
    #[inline(always)]
    pub fn wf53(&self) -> &WF53 {
        unsafe { &*(((self as *const Self) as *const u8).add(85usize) as *const WF53) }
    }
    #[doc = "0x55 - LCD Waveform Register 53."]
    #[inline(always)]
    pub fn wf53_mut(&self) -> &mut WF53 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(85usize) as *mut WF53) }
    }
    #[doc = "0x56 - LCD Waveform Register 54."]
    #[inline(always)]
    pub fn wf54(&self) -> &WF54 {
        unsafe { &*(((self as *const Self) as *const u8).add(86usize) as *const WF54) }
    }
    #[doc = "0x56 - LCD Waveform Register 54."]
    #[inline(always)]
    pub fn wf54_mut(&self) -> &mut WF54 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(86usize) as *mut WF54) }
    }
    #[doc = "0x57 - LCD Waveform Register 55."]
    #[inline(always)]
    pub fn wf55(&self) -> &WF55 {
        unsafe { &*(((self as *const Self) as *const u8).add(87usize) as *const WF55) }
    }
    #[doc = "0x57 - LCD Waveform Register 55."]
    #[inline(always)]
    pub fn wf55_mut(&self) -> &mut WF55 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(87usize) as *mut WF55) }
    }
    #[doc = "0x58 - LCD Waveform Register 56."]
    #[inline(always)]
    pub fn wf56(&self) -> &WF56 {
        unsafe { &*(((self as *const Self) as *const u8).add(88usize) as *const WF56) }
    }
    #[doc = "0x58 - LCD Waveform Register 56."]
    #[inline(always)]
    pub fn wf56_mut(&self) -> &mut WF56 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(88usize) as *mut WF56) }
    }
    #[doc = "0x58 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf59to56(&self) -> &WF59TO56 {
        unsafe { &*(((self as *const Self) as *const u8).add(88usize) as *const WF59TO56) }
    }
    #[doc = "0x58 - LCD Waveform register"]
    #[inline(always)]
    pub fn wf59to56_mut(&self) -> &mut WF59TO56 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(88usize) as *mut WF59TO56) }
    }
    #[doc = "0x59 - LCD Waveform Register 57."]
    #[inline(always)]
    pub fn wf57(&self) -> &WF57 {
        unsafe { &*(((self as *const Self) as *const u8).add(89usize) as *const WF57) }
    }
    #[doc = "0x59 - LCD Waveform Register 57."]
    #[inline(always)]
    pub fn wf57_mut(&self) -> &mut WF57 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(89usize) as *mut WF57) }
    }
    #[doc = "0x5a - LCD Waveform Register 58."]
    #[inline(always)]
    pub fn wf58(&self) -> &WF58 {
        unsafe { &*(((self as *const Self) as *const u8).add(90usize) as *const WF58) }
    }
    #[doc = "0x5a - LCD Waveform Register 58."]
    #[inline(always)]
    pub fn wf58_mut(&self) -> &mut WF58 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(90usize) as *mut WF58) }
    }
    #[doc = "0x5b - LCD Waveform Register 59."]
    #[inline(always)]
    pub fn wf59(&self) -> &WF59 {
        unsafe { &*(((self as *const Self) as *const u8).add(91usize) as *const WF59) }
    }
    #[doc = "0x5b - LCD Waveform Register 59."]
    #[inline(always)]
    pub fn wf59_mut(&self) -> &mut WF59 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(91usize) as *mut WF59) }
    }
    #[doc = "0x5c - LCD Waveform Register 60."]
    #[inline(always)]
    pub fn wf60(&self) -> &WF60 {
        unsafe { &*(((self as *const Self) as *const u8).add(92usize) as *const WF60) }
    }
    #[doc = "0x5c - LCD Waveform Register 60."]
    #[inline(always)]
    pub fn wf60_mut(&self) -> &mut WF60 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(92usize) as *mut WF60) }
    }
    #[doc = "0x5c - LCD Waveform register"]
    #[inline(always)]
    pub fn wf63to60(&self) -> &WF63TO60 {
        unsafe { &*(((self as *const Self) as *const u8).add(92usize) as *const WF63TO60) }
    }
    #[doc = "0x5c - LCD Waveform register"]
    #[inline(always)]
    pub fn wf63to60_mut(&self) -> &mut WF63TO60 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(92usize) as *mut WF63TO60) }
    }
    #[doc = "0x5d - LCD Waveform Register 61."]
    #[inline(always)]
    pub fn wf61(&self) -> &WF61 {
        unsafe { &*(((self as *const Self) as *const u8).add(93usize) as *const WF61) }
    }
    #[doc = "0x5d - LCD Waveform Register 61."]
    #[inline(always)]
    pub fn wf61_mut(&self) -> &mut WF61 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(93usize) as *mut WF61) }
    }
    #[doc = "0x5e - LCD Waveform Register 62."]
    #[inline(always)]
    pub fn wf62(&self) -> &WF62 {
        unsafe { &*(((self as *const Self) as *const u8).add(94usize) as *const WF62) }
    }
    #[doc = "0x5e - LCD Waveform Register 62."]
    #[inline(always)]
    pub fn wf62_mut(&self) -> &mut WF62 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(94usize) as *mut WF62) }
    }
    #[doc = "0x5f - LCD Waveform Register 63."]
    #[inline(always)]
    pub fn wf63(&self) -> &WF63 {
        unsafe { &*(((self as *const Self) as *const u8).add(95usize) as *const WF63) }
    }
    #[doc = "0x5f - LCD Waveform Register 63."]
    #[inline(always)]
    pub fn wf63_mut(&self) -> &mut WF63 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(95usize) as *mut WF63) }
    }
}
#[doc = "LCD General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcr](gcr) module"]
pub type GCR = crate::Reg<u32, _GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCR;
#[doc = "`read()` method returns [gcr::R](gcr::R) reader structure"]
impl crate::Readable for GCR {}
#[doc = "`write(|w| ..)` method takes [gcr::W](gcr::W) writer structure"]
impl crate::Writable for GCR {}
#[doc = "LCD General Control Register"]
pub mod gcr;
#[doc = "LCD Auxiliary Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ar](ar) module"]
pub type AR = crate::Reg<u32, _AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AR;
#[doc = "`read()` method returns [ar::R](ar::R) reader structure"]
impl crate::Readable for AR {}
#[doc = "`write(|w| ..)` method takes [ar::W](ar::W) writer structure"]
impl crate::Writable for AR {}
#[doc = "LCD Auxiliary Register"]
pub mod ar;
#[doc = "LCD Fault Detect Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcr](fdcr) module"]
pub type FDCR = crate::Reg<u32, _FDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCR;
#[doc = "`read()` method returns [fdcr::R](fdcr::R) reader structure"]
impl crate::Readable for FDCR {}
#[doc = "`write(|w| ..)` method takes [fdcr::W](fdcr::W) writer structure"]
impl crate::Writable for FDCR {}
#[doc = "LCD Fault Detect Control Register"]
pub mod fdcr;
#[doc = "LCD Fault Detect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdsr](fdsr) module"]
pub type FDSR = crate::Reg<u32, _FDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDSR;
#[doc = "`read()` method returns [fdsr::R](fdsr::R) reader structure"]
impl crate::Readable for FDSR {}
#[doc = "`write(|w| ..)` method takes [fdsr::W](fdsr::W) writer structure"]
impl crate::Writable for FDSR {}
#[doc = "LCD Fault Detect Status Register"]
pub mod fdsr;
#[doc = "LCD Pin Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pen](pen) module"]
pub type PEN = crate::Reg<u32, _PEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEN;
#[doc = "`read()` method returns [pen::R](pen::R) reader structure"]
impl crate::Readable for PEN {}
#[doc = "`write(|w| ..)` method takes [pen::W](pen::W) writer structure"]
impl crate::Writable for PEN {}
#[doc = "LCD Pin Enable register"]
pub mod pen;
#[doc = "LCD Back Plane Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpen](bpen) module"]
pub type BPEN = crate::Reg<u32, _BPEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BPEN;
#[doc = "`read()` method returns [bpen::R](bpen::R) reader structure"]
impl crate::Readable for BPEN {}
#[doc = "`write(|w| ..)` method takes [bpen::W](bpen::W) writer structure"]
impl crate::Writable for BPEN {}
#[doc = "LCD Back Plane Enable register"]
pub mod bpen;
#[doc = "LCD Waveform register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf3to0](wf3to0) module"]
pub type WF3TO0 = crate::Reg<u32, _WF3TO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF3TO0;
#[doc = "`read()` method returns [wf3to0::R](wf3to0::R) reader structure"]
impl crate::Readable for WF3TO0 {}
#[doc = "`write(|w| ..)` method takes [wf3to0::W](wf3to0::W) writer structure"]
impl crate::Writable for WF3TO0 {}
#[doc = "LCD Waveform register"]
pub mod wf3to0;
#[doc = "LCD Waveform Register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf0](wf0) module"]
pub type WF0 = crate::Reg<u8, _WF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF0;
#[doc = "`read()` method returns [wf0::R](wf0::R) reader structure"]
impl crate::Readable for WF0 {}
#[doc = "`write(|w| ..)` method takes [wf0::W](wf0::W) writer structure"]
impl crate::Writable for WF0 {}
#[doc = "LCD Waveform Register 0."]
pub mod wf0;
#[doc = "LCD Waveform Register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf1](wf1) module"]
pub type WF1 = crate::Reg<u8, _WF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF1;
#[doc = "`read()` method returns [wf1::R](wf1::R) reader structure"]
impl crate::Readable for WF1 {}
#[doc = "`write(|w| ..)` method takes [wf1::W](wf1::W) writer structure"]
impl crate::Writable for WF1 {}
#[doc = "LCD Waveform Register 1."]
pub mod wf1;
#[doc = "LCD Waveform Register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf2](wf2) module"]
pub type WF2 = crate::Reg<u8, _WF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF2;
#[doc = "`read()` method returns [wf2::R](wf2::R) reader structure"]
impl crate::Readable for WF2 {}
#[doc = "`write(|w| ..)` method takes [wf2::W](wf2::W) writer structure"]
impl crate::Writable for WF2 {}
#[doc = "LCD Waveform Register 2."]
pub mod wf2;
#[doc = "LCD Waveform Register 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf3](wf3) module"]
pub type WF3 = crate::Reg<u8, _WF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF3;
#[doc = "`read()` method returns [wf3::R](wf3::R) reader structure"]
impl crate::Readable for WF3 {}
#[doc = "`write(|w| ..)` method takes [wf3::W](wf3::W) writer structure"]
impl crate::Writable for WF3 {}
#[doc = "LCD Waveform Register 3."]
pub mod wf3;
#[doc = "LCD Waveform register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf7to4](wf7to4) module"]
pub type WF7TO4 = crate::Reg<u32, _WF7TO4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF7TO4;
#[doc = "`read()` method returns [wf7to4::R](wf7to4::R) reader structure"]
impl crate::Readable for WF7TO4 {}
#[doc = "`write(|w| ..)` method takes [wf7to4::W](wf7to4::W) writer structure"]
impl crate::Writable for WF7TO4 {}
#[doc = "LCD Waveform register"]
pub mod wf7to4;
#[doc = "LCD Waveform Register 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf4](wf4) module"]
pub type WF4 = crate::Reg<u8, _WF4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF4;
#[doc = "`read()` method returns [wf4::R](wf4::R) reader structure"]
impl crate::Readable for WF4 {}
#[doc = "`write(|w| ..)` method takes [wf4::W](wf4::W) writer structure"]
impl crate::Writable for WF4 {}
#[doc = "LCD Waveform Register 4."]
pub mod wf4;
#[doc = "LCD Waveform Register 5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf5](wf5) module"]
pub type WF5 = crate::Reg<u8, _WF5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF5;
#[doc = "`read()` method returns [wf5::R](wf5::R) reader structure"]
impl crate::Readable for WF5 {}
#[doc = "`write(|w| ..)` method takes [wf5::W](wf5::W) writer structure"]
impl crate::Writable for WF5 {}
#[doc = "LCD Waveform Register 5."]
pub mod wf5;
#[doc = "LCD Waveform Register 6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf6](wf6) module"]
pub type WF6 = crate::Reg<u8, _WF6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF6;
#[doc = "`read()` method returns [wf6::R](wf6::R) reader structure"]
impl crate::Readable for WF6 {}
#[doc = "`write(|w| ..)` method takes [wf6::W](wf6::W) writer structure"]
impl crate::Writable for WF6 {}
#[doc = "LCD Waveform Register 6."]
pub mod wf6;
#[doc = "LCD Waveform Register 7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf7](wf7) module"]
pub type WF7 = crate::Reg<u8, _WF7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF7;
#[doc = "`read()` method returns [wf7::R](wf7::R) reader structure"]
impl crate::Readable for WF7 {}
#[doc = "`write(|w| ..)` method takes [wf7::W](wf7::W) writer structure"]
impl crate::Writable for WF7 {}
#[doc = "LCD Waveform Register 7."]
pub mod wf7;
#[doc = "LCD Waveform register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf11to8](wf11to8) module"]
pub type WF11TO8 = crate::Reg<u32, _WF11TO8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF11TO8;
#[doc = "`read()` method returns [wf11to8::R](wf11to8::R) reader structure"]
impl crate::Readable for WF11TO8 {}
#[doc = "`write(|w| ..)` method takes [wf11to8::W](wf11to8::W) writer structure"]
impl crate::Writable for WF11TO8 {}
#[doc = "LCD Waveform register"]
pub mod wf11to8;
#[doc = "LCD Waveform Register 8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf8](wf8) module"]
pub type WF8 = crate::Reg<u8, _WF8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF8;
#[doc = "`read()` method returns [wf8::R](wf8::R) reader structure"]
impl crate::Readable for WF8 {}
#[doc = "`write(|w| ..)` method takes [wf8::W](wf8::W) writer structure"]
impl crate::Writable for WF8 {}
#[doc = "LCD Waveform Register 8."]
pub mod wf8;
#[doc = "LCD Waveform Register 9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf9](wf9) module"]
pub type WF9 = crate::Reg<u8, _WF9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF9;
#[doc = "`read()` method returns [wf9::R](wf9::R) reader structure"]
impl crate::Readable for WF9 {}
#[doc = "`write(|w| ..)` method takes [wf9::W](wf9::W) writer structure"]
impl crate::Writable for WF9 {}
#[doc = "LCD Waveform Register 9."]
pub mod wf9;
#[doc = "LCD Waveform Register 10.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf10](wf10) module"]
pub type WF10 = crate::Reg<u8, _WF10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF10;
#[doc = "`read()` method returns [wf10::R](wf10::R) reader structure"]
impl crate::Readable for WF10 {}
#[doc = "`write(|w| ..)` method takes [wf10::W](wf10::W) writer structure"]
impl crate::Writable for WF10 {}
#[doc = "LCD Waveform Register 10."]
pub mod wf10;
#[doc = "LCD Waveform Register 11.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf11](wf11) module"]
pub type WF11 = crate::Reg<u8, _WF11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF11;
#[doc = "`read()` method returns [wf11::R](wf11::R) reader structure"]
impl crate::Readable for WF11 {}
#[doc = "`write(|w| ..)` method takes [wf11::W](wf11::W) writer structure"]
impl crate::Writable for WF11 {}
#[doc = "LCD Waveform Register 11."]
pub mod wf11;
#[doc = "LCD Waveform register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf15to12](wf15to12) module"]
pub type WF15TO12 = crate::Reg<u32, _WF15TO12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF15TO12;
#[doc = "`read()` method returns [wf15to12::R](wf15to12::R) reader structure"]
impl crate::Readable for WF15TO12 {}
#[doc = "`write(|w| ..)` method takes [wf15to12::W](wf15to12::W) writer structure"]
impl crate::Writable for WF15TO12 {}
#[doc = "LCD Waveform register"]
pub mod wf15to12;
#[doc = "LCD Waveform Register 12.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf12](wf12) module"]
pub type WF12 = crate::Reg<u8, _WF12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF12;
#[doc = "`read()` method returns [wf12::R](wf12::R) reader structure"]
impl crate::Readable for WF12 {}
#[doc = "`write(|w| ..)` method takes [wf12::W](wf12::W) writer structure"]
impl crate::Writable for WF12 {}
#[doc = "LCD Waveform Register 12."]
pub mod wf12;
#[doc = "LCD Waveform Register 13.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf13](wf13) module"]
pub type WF13 = crate::Reg<u8, _WF13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF13;
#[doc = "`read()` method returns [wf13::R](wf13::R) reader structure"]
impl crate::Readable for WF13 {}
#[doc = "`write(|w| ..)` method takes [wf13::W](wf13::W) writer structure"]
impl crate::Writable for WF13 {}
#[doc = "LCD Waveform Register 13."]
pub mod wf13;
#[doc = "LCD Waveform Register 14.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf14](wf14) module"]
pub type WF14 = crate::Reg<u8, _WF14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF14;
#[doc = "`read()` method returns [wf14::R](wf14::R) reader structure"]
impl crate::Readable for WF14 {}
#[doc = "`write(|w| ..)` method takes [wf14::W](wf14::W) writer structure"]
impl crate::Writable for WF14 {}
#[doc = "LCD Waveform Register 14."]
pub mod wf14;
#[doc = "LCD Waveform Register 15.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf15](wf15) module"]
pub type WF15 = crate::Reg<u8, _WF15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF15;
#[doc = "`read()` method returns [wf15::R](wf15::R) reader structure"]
impl crate::Readable for WF15 {}
#[doc = "`write(|w| ..)` method takes [wf15::W](wf15::W) writer structure"]
impl crate::Writable for WF15 {}
#[doc = "LCD Waveform Register 15."]
pub mod wf15;
#[doc = "LCD Waveform register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf19to16](wf19to16) module"]
pub type WF19TO16 = crate::Reg<u32, _WF19TO16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF19TO16;
#[doc = "`read()` method returns [wf19to16::R](wf19to16::R) reader structure"]
impl crate::Readable for WF19TO16 {}
#[doc = "`write(|w| ..)` method takes [wf19to16::W](wf19to16::W) writer structure"]
impl crate::Writable for WF19TO16 {}
#[doc = "LCD Waveform register"]
pub mod wf19to16;
#[doc = "LCD Waveform Register 16.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf16](wf16) module"]
pub type WF16 = crate::Reg<u8, _WF16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF16;
#[doc = "`read()` method returns [wf16::R](wf16::R) reader structure"]
impl crate::Readable for WF16 {}
#[doc = "`write(|w| ..)` method takes [wf16::W](wf16::W) writer structure"]
impl crate::Writable for WF16 {}
#[doc = "LCD Waveform Register 16."]
pub mod wf16;
#[doc = "LCD Waveform Register 17.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf17](wf17) module"]
pub type WF17 = crate::Reg<u8, _WF17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF17;
#[doc = "`read()` method returns [wf17::R](wf17::R) reader structure"]
impl crate::Readable for WF17 {}
#[doc = "`write(|w| ..)` method takes [wf17::W](wf17::W) writer structure"]
impl crate::Writable for WF17 {}
#[doc = "LCD Waveform Register 17."]
pub mod wf17;
#[doc = "LCD Waveform Register 18.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf18](wf18) module"]
pub type WF18 = crate::Reg<u8, _WF18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF18;
#[doc = "`read()` method returns [wf18::R](wf18::R) reader structure"]
impl crate::Readable for WF18 {}
#[doc = "`write(|w| ..)` method takes [wf18::W](wf18::W) writer structure"]
impl crate::Writable for WF18 {}
#[doc = "LCD Waveform Register 18."]
pub mod wf18;
#[doc = "LCD Waveform Register 19.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf19](wf19) module"]
pub type WF19 = crate::Reg<u8, _WF19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF19;
#[doc = "`read()` method returns [wf19::R](wf19::R) reader structure"]
impl crate::Readable for WF19 {}
#[doc = "`write(|w| ..)` method takes [wf19::W](wf19::W) writer structure"]
impl crate::Writable for WF19 {}
#[doc = "LCD Waveform Register 19."]
pub mod wf19;
#[doc = "LCD Waveform register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf23to20](wf23to20) module"]
pub type WF23TO20 = crate::Reg<u32, _WF23TO20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF23TO20;
#[doc = "`read()` method returns [wf23to20::R](wf23to20::R) reader structure"]
impl crate::Readable for WF23TO20 {}
#[doc = "`write(|w| ..)` method takes [wf23to20::W](wf23to20::W) writer structure"]
impl crate::Writable for WF23TO20 {}
#[doc = "LCD Waveform register"]
pub mod wf23to20;
#[doc = "LCD Waveform Register 20.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf20](wf20) module"]
pub type WF20 = crate::Reg<u8, _WF20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF20;
#[doc = "`read()` method returns [wf20::R](wf20::R) reader structure"]
impl crate::Readable for WF20 {}
#[doc = "`write(|w| ..)` method takes [wf20::W](wf20::W) writer structure"]
impl crate::Writable for WF20 {}
#[doc = "LCD Waveform Register 20."]
pub mod wf20;
#[doc = "LCD Waveform Register 21.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf21](wf21) module"]
pub type WF21 = crate::Reg<u8, _WF21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF21;
#[doc = "`read()` method returns [wf21::R](wf21::R) reader structure"]
impl crate::Readable for WF21 {}
#[doc = "`write(|w| ..)` method takes [wf21::W](wf21::W) writer structure"]
impl crate::Writable for WF21 {}
#[doc = "LCD Waveform Register 21."]
pub mod wf21;
#[doc = "LCD Waveform Register 22.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf22](wf22) module"]
pub type WF22 = crate::Reg<u8, _WF22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF22;
#[doc = "`read()` method returns [wf22::R](wf22::R) reader structure"]
impl crate::Readable for WF22 {}
#[doc = "`write(|w| ..)` method takes [wf22::W](wf22::W) writer structure"]
impl crate::Writable for WF22 {}
#[doc = "LCD Waveform Register 22."]
pub mod wf22;
#[doc = "LCD Waveform Register 23.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf23](wf23) module"]
pub type WF23 = crate::Reg<u8, _WF23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF23;
#[doc = "`read()` method returns [wf23::R](wf23::R) reader structure"]
impl crate::Readable for WF23 {}
#[doc = "`write(|w| ..)` method takes [wf23::W](wf23::W) writer structure"]
impl crate::Writable for WF23 {}
#[doc = "LCD Waveform Register 23."]
pub mod wf23;
#[doc = "LCD Waveform register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf27to24](wf27to24) module"]
pub type WF27TO24 = crate::Reg<u32, _WF27TO24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF27TO24;
#[doc = "`read()` method returns [wf27to24::R](wf27to24::R) reader structure"]
impl crate::Readable for WF27TO24 {}
#[doc = "`write(|w| ..)` method takes [wf27to24::W](wf27to24::W) writer structure"]
impl crate::Writable for WF27TO24 {}
#[doc = "LCD Waveform register"]
pub mod wf27to24;
#[doc = "LCD Waveform Register 24.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf24](wf24) module"]
pub type WF24 = crate::Reg<u8, _WF24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF24;
#[doc = "`read()` method returns [wf24::R](wf24::R) reader structure"]
impl crate::Readable for WF24 {}
#[doc = "`write(|w| ..)` method takes [wf24::W](wf24::W) writer structure"]
impl crate::Writable for WF24 {}
#[doc = "LCD Waveform Register 24."]
pub mod wf24;
#[doc = "LCD Waveform Register 25.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf25](wf25) module"]
pub type WF25 = crate::Reg<u8, _WF25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF25;
#[doc = "`read()` method returns [wf25::R](wf25::R) reader structure"]
impl crate::Readable for WF25 {}
#[doc = "`write(|w| ..)` method takes [wf25::W](wf25::W) writer structure"]
impl crate::Writable for WF25 {}
#[doc = "LCD Waveform Register 25."]
pub mod wf25;
#[doc = "LCD Waveform Register 26.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf26](wf26) module"]
pub type WF26 = crate::Reg<u8, _WF26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF26;
#[doc = "`read()` method returns [wf26::R](wf26::R) reader structure"]
impl crate::Readable for WF26 {}
#[doc = "`write(|w| ..)` method takes [wf26::W](wf26::W) writer structure"]
impl crate::Writable for WF26 {}
#[doc = "LCD Waveform Register 26."]
pub mod wf26;
#[doc = "LCD Waveform Register 27.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf27](wf27) module"]
pub type WF27 = crate::Reg<u8, _WF27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF27;
#[doc = "`read()` method returns [wf27::R](wf27::R) reader structure"]
impl crate::Readable for WF27 {}
#[doc = "`write(|w| ..)` method takes [wf27::W](wf27::W) writer structure"]
impl crate::Writable for WF27 {}
#[doc = "LCD Waveform Register 27."]
pub mod wf27;
#[doc = "LCD Waveform register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf31to28](wf31to28) module"]
pub type WF31TO28 = crate::Reg<u32, _WF31TO28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF31TO28;
#[doc = "`read()` method returns [wf31to28::R](wf31to28::R) reader structure"]
impl crate::Readable for WF31TO28 {}
#[doc = "`write(|w| ..)` method takes [wf31to28::W](wf31to28::W) writer structure"]
impl crate::Writable for WF31TO28 {}
#[doc = "LCD Waveform register"]
pub mod wf31to28;
#[doc = "LCD Waveform Register 28.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf28](wf28) module"]
pub type WF28 = crate::Reg<u8, _WF28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF28;
#[doc = "`read()` method returns [wf28::R](wf28::R) reader structure"]
impl crate::Readable for WF28 {}
#[doc = "`write(|w| ..)` method takes [wf28::W](wf28::W) writer structure"]
impl crate::Writable for WF28 {}
#[doc = "LCD Waveform Register 28."]
pub mod wf28;
#[doc = "LCD Waveform Register 29.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf29](wf29) module"]
pub type WF29 = crate::Reg<u8, _WF29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF29;
#[doc = "`read()` method returns [wf29::R](wf29::R) reader structure"]
impl crate::Readable for WF29 {}
#[doc = "`write(|w| ..)` method takes [wf29::W](wf29::W) writer structure"]
impl crate::Writable for WF29 {}
#[doc = "LCD Waveform Register 29."]
pub mod wf29;
#[doc = "LCD Waveform Register 30.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf30](wf30) module"]
pub type WF30 = crate::Reg<u8, _WF30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF30;
#[doc = "`read()` method returns [wf30::R](wf30::R) reader structure"]
impl crate::Readable for WF30 {}
#[doc = "`write(|w| ..)` method takes [wf30::W](wf30::W) writer structure"]
impl crate::Writable for WF30 {}
#[doc = "LCD Waveform Register 30."]
pub mod wf30;
#[doc = "LCD Waveform Register 31.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf31](wf31) module"]
pub type WF31 = crate::Reg<u8, _WF31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF31;
#[doc = "`read()` method returns [wf31::R](wf31::R) reader structure"]
impl crate::Readable for WF31 {}
#[doc = "`write(|w| ..)` method takes [wf31::W](wf31::W) writer structure"]
impl crate::Writable for WF31 {}
#[doc = "LCD Waveform Register 31."]
pub mod wf31;
#[doc = "LCD Waveform register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf35to32](wf35to32) module"]
pub type WF35TO32 = crate::Reg<u32, _WF35TO32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF35TO32;
#[doc = "`read()` method returns [wf35to32::R](wf35to32::R) reader structure"]
impl crate::Readable for WF35TO32 {}
#[doc = "`write(|w| ..)` method takes [wf35to32::W](wf35to32::W) writer structure"]
impl crate::Writable for WF35TO32 {}
#[doc = "LCD Waveform register"]
pub mod wf35to32;
#[doc = "LCD Waveform Register 32.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf32](wf32) module"]
pub type WF32 = crate::Reg<u8, _WF32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF32;
#[doc = "`read()` method returns [wf32::R](wf32::R) reader structure"]
impl crate::Readable for WF32 {}
#[doc = "`write(|w| ..)` method takes [wf32::W](wf32::W) writer structure"]
impl crate::Writable for WF32 {}
#[doc = "LCD Waveform Register 32."]
pub mod wf32;
#[doc = "LCD Waveform Register 33.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf33](wf33) module"]
pub type WF33 = crate::Reg<u8, _WF33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF33;
#[doc = "`read()` method returns [wf33::R](wf33::R) reader structure"]
impl crate::Readable for WF33 {}
#[doc = "`write(|w| ..)` method takes [wf33::W](wf33::W) writer structure"]
impl crate::Writable for WF33 {}
#[doc = "LCD Waveform Register 33."]
pub mod wf33;
#[doc = "LCD Waveform Register 34.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf34](wf34) module"]
pub type WF34 = crate::Reg<u8, _WF34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF34;
#[doc = "`read()` method returns [wf34::R](wf34::R) reader structure"]
impl crate::Readable for WF34 {}
#[doc = "`write(|w| ..)` method takes [wf34::W](wf34::W) writer structure"]
impl crate::Writable for WF34 {}
#[doc = "LCD Waveform Register 34."]
pub mod wf34;
#[doc = "LCD Waveform Register 35.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf35](wf35) module"]
pub type WF35 = crate::Reg<u8, _WF35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF35;
#[doc = "`read()` method returns [wf35::R](wf35::R) reader structure"]
impl crate::Readable for WF35 {}
#[doc = "`write(|w| ..)` method takes [wf35::W](wf35::W) writer structure"]
impl crate::Writable for WF35 {}
#[doc = "LCD Waveform Register 35."]
pub mod wf35;
#[doc = "LCD Waveform register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf39to36](wf39to36) module"]
pub type WF39TO36 = crate::Reg<u32, _WF39TO36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF39TO36;
#[doc = "`read()` method returns [wf39to36::R](wf39to36::R) reader structure"]
impl crate::Readable for WF39TO36 {}
#[doc = "`write(|w| ..)` method takes [wf39to36::W](wf39to36::W) writer structure"]
impl crate::Writable for WF39TO36 {}
#[doc = "LCD Waveform register"]
pub mod wf39to36;
#[doc = "LCD Waveform Register 36.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf36](wf36) module"]
pub type WF36 = crate::Reg<u8, _WF36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF36;
#[doc = "`read()` method returns [wf36::R](wf36::R) reader structure"]
impl crate::Readable for WF36 {}
#[doc = "`write(|w| ..)` method takes [wf36::W](wf36::W) writer structure"]
impl crate::Writable for WF36 {}
#[doc = "LCD Waveform Register 36."]
pub mod wf36;
#[doc = "LCD Waveform Register 37.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf37](wf37) module"]
pub type WF37 = crate::Reg<u8, _WF37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF37;
#[doc = "`read()` method returns [wf37::R](wf37::R) reader structure"]
impl crate::Readable for WF37 {}
#[doc = "`write(|w| ..)` method takes [wf37::W](wf37::W) writer structure"]
impl crate::Writable for WF37 {}
#[doc = "LCD Waveform Register 37."]
pub mod wf37;
#[doc = "LCD Waveform Register 38.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf38](wf38) module"]
pub type WF38 = crate::Reg<u8, _WF38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF38;
#[doc = "`read()` method returns [wf38::R](wf38::R) reader structure"]
impl crate::Readable for WF38 {}
#[doc = "`write(|w| ..)` method takes [wf38::W](wf38::W) writer structure"]
impl crate::Writable for WF38 {}
#[doc = "LCD Waveform Register 38."]
pub mod wf38;
#[doc = "LCD Waveform Register 39.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf39](wf39) module"]
pub type WF39 = crate::Reg<u8, _WF39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF39;
#[doc = "`read()` method returns [wf39::R](wf39::R) reader structure"]
impl crate::Readable for WF39 {}
#[doc = "`write(|w| ..)` method takes [wf39::W](wf39::W) writer structure"]
impl crate::Writable for WF39 {}
#[doc = "LCD Waveform Register 39."]
pub mod wf39;
#[doc = "LCD Waveform register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf43to40](wf43to40) module"]
pub type WF43TO40 = crate::Reg<u32, _WF43TO40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF43TO40;
#[doc = "`read()` method returns [wf43to40::R](wf43to40::R) reader structure"]
impl crate::Readable for WF43TO40 {}
#[doc = "`write(|w| ..)` method takes [wf43to40::W](wf43to40::W) writer structure"]
impl crate::Writable for WF43TO40 {}
#[doc = "LCD Waveform register"]
pub mod wf43to40;
#[doc = "LCD Waveform Register 40.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf40](wf40) module"]
pub type WF40 = crate::Reg<u8, _WF40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF40;
#[doc = "`read()` method returns [wf40::R](wf40::R) reader structure"]
impl crate::Readable for WF40 {}
#[doc = "`write(|w| ..)` method takes [wf40::W](wf40::W) writer structure"]
impl crate::Writable for WF40 {}
#[doc = "LCD Waveform Register 40."]
pub mod wf40;
#[doc = "LCD Waveform Register 41.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf41](wf41) module"]
pub type WF41 = crate::Reg<u8, _WF41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF41;
#[doc = "`read()` method returns [wf41::R](wf41::R) reader structure"]
impl crate::Readable for WF41 {}
#[doc = "`write(|w| ..)` method takes [wf41::W](wf41::W) writer structure"]
impl crate::Writable for WF41 {}
#[doc = "LCD Waveform Register 41."]
pub mod wf41;
#[doc = "LCD Waveform Register 42.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf42](wf42) module"]
pub type WF42 = crate::Reg<u8, _WF42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF42;
#[doc = "`read()` method returns [wf42::R](wf42::R) reader structure"]
impl crate::Readable for WF42 {}
#[doc = "`write(|w| ..)` method takes [wf42::W](wf42::W) writer structure"]
impl crate::Writable for WF42 {}
#[doc = "LCD Waveform Register 42."]
pub mod wf42;
#[doc = "LCD Waveform Register 43.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf43](wf43) module"]
pub type WF43 = crate::Reg<u8, _WF43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF43;
#[doc = "`read()` method returns [wf43::R](wf43::R) reader structure"]
impl crate::Readable for WF43 {}
#[doc = "`write(|w| ..)` method takes [wf43::W](wf43::W) writer structure"]
impl crate::Writable for WF43 {}
#[doc = "LCD Waveform Register 43."]
pub mod wf43;
#[doc = "LCD Waveform register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf47to44](wf47to44) module"]
pub type WF47TO44 = crate::Reg<u32, _WF47TO44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF47TO44;
#[doc = "`read()` method returns [wf47to44::R](wf47to44::R) reader structure"]
impl crate::Readable for WF47TO44 {}
#[doc = "`write(|w| ..)` method takes [wf47to44::W](wf47to44::W) writer structure"]
impl crate::Writable for WF47TO44 {}
#[doc = "LCD Waveform register"]
pub mod wf47to44;
#[doc = "LCD Waveform Register 44.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf44](wf44) module"]
pub type WF44 = crate::Reg<u8, _WF44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF44;
#[doc = "`read()` method returns [wf44::R](wf44::R) reader structure"]
impl crate::Readable for WF44 {}
#[doc = "`write(|w| ..)` method takes [wf44::W](wf44::W) writer structure"]
impl crate::Writable for WF44 {}
#[doc = "LCD Waveform Register 44."]
pub mod wf44;
#[doc = "LCD Waveform Register 45.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf45](wf45) module"]
pub type WF45 = crate::Reg<u8, _WF45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF45;
#[doc = "`read()` method returns [wf45::R](wf45::R) reader structure"]
impl crate::Readable for WF45 {}
#[doc = "`write(|w| ..)` method takes [wf45::W](wf45::W) writer structure"]
impl crate::Writable for WF45 {}
#[doc = "LCD Waveform Register 45."]
pub mod wf45;
#[doc = "LCD Waveform Register 46.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf46](wf46) module"]
pub type WF46 = crate::Reg<u8, _WF46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF46;
#[doc = "`read()` method returns [wf46::R](wf46::R) reader structure"]
impl crate::Readable for WF46 {}
#[doc = "`write(|w| ..)` method takes [wf46::W](wf46::W) writer structure"]
impl crate::Writable for WF46 {}
#[doc = "LCD Waveform Register 46."]
pub mod wf46;
#[doc = "LCD Waveform Register 47.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf47](wf47) module"]
pub type WF47 = crate::Reg<u8, _WF47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF47;
#[doc = "`read()` method returns [wf47::R](wf47::R) reader structure"]
impl crate::Readable for WF47 {}
#[doc = "`write(|w| ..)` method takes [wf47::W](wf47::W) writer structure"]
impl crate::Writable for WF47 {}
#[doc = "LCD Waveform Register 47."]
pub mod wf47;
#[doc = "LCD Waveform register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf51to48](wf51to48) module"]
pub type WF51TO48 = crate::Reg<u32, _WF51TO48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF51TO48;
#[doc = "`read()` method returns [wf51to48::R](wf51to48::R) reader structure"]
impl crate::Readable for WF51TO48 {}
#[doc = "`write(|w| ..)` method takes [wf51to48::W](wf51to48::W) writer structure"]
impl crate::Writable for WF51TO48 {}
#[doc = "LCD Waveform register"]
pub mod wf51to48;
#[doc = "LCD Waveform Register 48.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf48](wf48) module"]
pub type WF48 = crate::Reg<u8, _WF48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF48;
#[doc = "`read()` method returns [wf48::R](wf48::R) reader structure"]
impl crate::Readable for WF48 {}
#[doc = "`write(|w| ..)` method takes [wf48::W](wf48::W) writer structure"]
impl crate::Writable for WF48 {}
#[doc = "LCD Waveform Register 48."]
pub mod wf48;
#[doc = "LCD Waveform Register 49.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf49](wf49) module"]
pub type WF49 = crate::Reg<u8, _WF49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF49;
#[doc = "`read()` method returns [wf49::R](wf49::R) reader structure"]
impl crate::Readable for WF49 {}
#[doc = "`write(|w| ..)` method takes [wf49::W](wf49::W) writer structure"]
impl crate::Writable for WF49 {}
#[doc = "LCD Waveform Register 49."]
pub mod wf49;
#[doc = "LCD Waveform Register 50.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf50](wf50) module"]
pub type WF50 = crate::Reg<u8, _WF50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF50;
#[doc = "`read()` method returns [wf50::R](wf50::R) reader structure"]
impl crate::Readable for WF50 {}
#[doc = "`write(|w| ..)` method takes [wf50::W](wf50::W) writer structure"]
impl crate::Writable for WF50 {}
#[doc = "LCD Waveform Register 50."]
pub mod wf50;
#[doc = "LCD Waveform Register 51.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf51](wf51) module"]
pub type WF51 = crate::Reg<u8, _WF51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF51;
#[doc = "`read()` method returns [wf51::R](wf51::R) reader structure"]
impl crate::Readable for WF51 {}
#[doc = "`write(|w| ..)` method takes [wf51::W](wf51::W) writer structure"]
impl crate::Writable for WF51 {}
#[doc = "LCD Waveform Register 51."]
pub mod wf51;
#[doc = "LCD Waveform register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf55to52](wf55to52) module"]
pub type WF55TO52 = crate::Reg<u32, _WF55TO52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF55TO52;
#[doc = "`read()` method returns [wf55to52::R](wf55to52::R) reader structure"]
impl crate::Readable for WF55TO52 {}
#[doc = "`write(|w| ..)` method takes [wf55to52::W](wf55to52::W) writer structure"]
impl crate::Writable for WF55TO52 {}
#[doc = "LCD Waveform register"]
pub mod wf55to52;
#[doc = "LCD Waveform Register 52.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf52](wf52) module"]
pub type WF52 = crate::Reg<u8, _WF52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF52;
#[doc = "`read()` method returns [wf52::R](wf52::R) reader structure"]
impl crate::Readable for WF52 {}
#[doc = "`write(|w| ..)` method takes [wf52::W](wf52::W) writer structure"]
impl crate::Writable for WF52 {}
#[doc = "LCD Waveform Register 52."]
pub mod wf52;
#[doc = "LCD Waveform Register 53.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf53](wf53) module"]
pub type WF53 = crate::Reg<u8, _WF53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF53;
#[doc = "`read()` method returns [wf53::R](wf53::R) reader structure"]
impl crate::Readable for WF53 {}
#[doc = "`write(|w| ..)` method takes [wf53::W](wf53::W) writer structure"]
impl crate::Writable for WF53 {}
#[doc = "LCD Waveform Register 53."]
pub mod wf53;
#[doc = "LCD Waveform Register 54.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf54](wf54) module"]
pub type WF54 = crate::Reg<u8, _WF54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF54;
#[doc = "`read()` method returns [wf54::R](wf54::R) reader structure"]
impl crate::Readable for WF54 {}
#[doc = "`write(|w| ..)` method takes [wf54::W](wf54::W) writer structure"]
impl crate::Writable for WF54 {}
#[doc = "LCD Waveform Register 54."]
pub mod wf54;
#[doc = "LCD Waveform Register 55.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf55](wf55) module"]
pub type WF55 = crate::Reg<u8, _WF55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF55;
#[doc = "`read()` method returns [wf55::R](wf55::R) reader structure"]
impl crate::Readable for WF55 {}
#[doc = "`write(|w| ..)` method takes [wf55::W](wf55::W) writer structure"]
impl crate::Writable for WF55 {}
#[doc = "LCD Waveform Register 55."]
pub mod wf55;
#[doc = "LCD Waveform register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf59to56](wf59to56) module"]
pub type WF59TO56 = crate::Reg<u32, _WF59TO56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF59TO56;
#[doc = "`read()` method returns [wf59to56::R](wf59to56::R) reader structure"]
impl crate::Readable for WF59TO56 {}
#[doc = "`write(|w| ..)` method takes [wf59to56::W](wf59to56::W) writer structure"]
impl crate::Writable for WF59TO56 {}
#[doc = "LCD Waveform register"]
pub mod wf59to56;
#[doc = "LCD Waveform Register 56.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf56](wf56) module"]
pub type WF56 = crate::Reg<u8, _WF56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF56;
#[doc = "`read()` method returns [wf56::R](wf56::R) reader structure"]
impl crate::Readable for WF56 {}
#[doc = "`write(|w| ..)` method takes [wf56::W](wf56::W) writer structure"]
impl crate::Writable for WF56 {}
#[doc = "LCD Waveform Register 56."]
pub mod wf56;
#[doc = "LCD Waveform Register 57.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf57](wf57) module"]
pub type WF57 = crate::Reg<u8, _WF57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF57;
#[doc = "`read()` method returns [wf57::R](wf57::R) reader structure"]
impl crate::Readable for WF57 {}
#[doc = "`write(|w| ..)` method takes [wf57::W](wf57::W) writer structure"]
impl crate::Writable for WF57 {}
#[doc = "LCD Waveform Register 57."]
pub mod wf57;
#[doc = "LCD Waveform Register 58.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf58](wf58) module"]
pub type WF58 = crate::Reg<u8, _WF58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF58;
#[doc = "`read()` method returns [wf58::R](wf58::R) reader structure"]
impl crate::Readable for WF58 {}
#[doc = "`write(|w| ..)` method takes [wf58::W](wf58::W) writer structure"]
impl crate::Writable for WF58 {}
#[doc = "LCD Waveform Register 58."]
pub mod wf58;
#[doc = "LCD Waveform Register 59.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf59](wf59) module"]
pub type WF59 = crate::Reg<u8, _WF59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF59;
#[doc = "`read()` method returns [wf59::R](wf59::R) reader structure"]
impl crate::Readable for WF59 {}
#[doc = "`write(|w| ..)` method takes [wf59::W](wf59::W) writer structure"]
impl crate::Writable for WF59 {}
#[doc = "LCD Waveform Register 59."]
pub mod wf59;
#[doc = "LCD Waveform register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf63to60](wf63to60) module"]
pub type WF63TO60 = crate::Reg<u32, _WF63TO60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF63TO60;
#[doc = "`read()` method returns [wf63to60::R](wf63to60::R) reader structure"]
impl crate::Readable for WF63TO60 {}
#[doc = "`write(|w| ..)` method takes [wf63to60::W](wf63to60::W) writer structure"]
impl crate::Writable for WF63TO60 {}
#[doc = "LCD Waveform register"]
pub mod wf63to60;
#[doc = "LCD Waveform Register 60.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf60](wf60) module"]
pub type WF60 = crate::Reg<u8, _WF60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF60;
#[doc = "`read()` method returns [wf60::R](wf60::R) reader structure"]
impl crate::Readable for WF60 {}
#[doc = "`write(|w| ..)` method takes [wf60::W](wf60::W) writer structure"]
impl crate::Writable for WF60 {}
#[doc = "LCD Waveform Register 60."]
pub mod wf60;
#[doc = "LCD Waveform Register 61.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf61](wf61) module"]
pub type WF61 = crate::Reg<u8, _WF61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF61;
#[doc = "`read()` method returns [wf61::R](wf61::R) reader structure"]
impl crate::Readable for WF61 {}
#[doc = "`write(|w| ..)` method takes [wf61::W](wf61::W) writer structure"]
impl crate::Writable for WF61 {}
#[doc = "LCD Waveform Register 61."]
pub mod wf61;
#[doc = "LCD Waveform Register 62.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf62](wf62) module"]
pub type WF62 = crate::Reg<u8, _WF62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF62;
#[doc = "`read()` method returns [wf62::R](wf62::R) reader structure"]
impl crate::Readable for WF62 {}
#[doc = "`write(|w| ..)` method takes [wf62::W](wf62::W) writer structure"]
impl crate::Writable for WF62 {}
#[doc = "LCD Waveform Register 62."]
pub mod wf62;
#[doc = "LCD Waveform Register 63.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf63](wf63) module"]
pub type WF63 = crate::Reg<u8, _WF63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF63;
#[doc = "`read()` method returns [wf63::R](wf63::R) reader structure"]
impl crate::Readable for WF63 {}
#[doc = "`write(|w| ..)` method takes [wf63::W](wf63::W) writer structure"]
impl crate::Writable for WF63 {}
#[doc = "LCD Waveform Register 63."]
pub mod wf63;
