#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel0 Configuration Register"]
    pub ch0_cfr: CH0_CFR,
    #[doc = "0x04 - Channel1 Configuration Register"]
    pub ch1_cfr: CH1_CFR,
    #[doc = "0x08 - Channel2 Configuration Register"]
    pub ch2_cfr: CH2_CFR,
    #[doc = "0x0c - Channel3 Configuration Register"]
    pub ch3_cfr: CH3_CFR,
    _reserved4: [u8; 8usize],
    #[doc = "0x18 - Control Register"]
    pub cr: CR,
    #[doc = "0x1c - Clock Configuration Register"]
    pub ckr: CKR,
    #[doc = "0x20 - DMA and Interrupt Register"]
    pub di: DI,
    _reserved7: [u8; 8usize],
    #[doc = "0x2c - Channel0 Delay Register"]
    pub ch0_dr: CH0_DR,
    #[doc = "0x30 - Channel1 Delay Register"]
    pub ch1_dr: CH1_DR,
    #[doc = "0x34 - Channel2 Delay Register"]
    pub ch2_dr: CH2_DR,
    #[doc = "0x38 - Channel3 Delay Register"]
    pub ch3_dr: CH3_DR,
    _reserved11: [u8; 8usize],
    #[doc = "0x44 - Channel0 Result Register"]
    pub ch0_rr: CH0_RR,
    #[doc = "0x48 - Channel1 Result Register"]
    pub ch1_rr: CH1_RR,
    #[doc = "0x4c - Channel2 Result Register"]
    pub ch2_rr: CH2_RR,
    #[doc = "0x50 - Channel3 Result Register"]
    pub ch3_rr: CH3_RR,
    _reserved15: [u8; 8usize],
    #[doc = "0x5c - Status Register"]
    pub sr: SR,
}
#[doc = "Channel0 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_cfr](ch0_cfr) module"]
pub type CH0_CFR = crate::Reg<u32, _CH0_CFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_CFR;
#[doc = "`read()` method returns [ch0_cfr::R](ch0_cfr::R) reader structure"]
impl crate::Readable for CH0_CFR {}
#[doc = "`write(|w| ..)` method takes [ch0_cfr::W](ch0_cfr::W) writer structure"]
impl crate::Writable for CH0_CFR {}
#[doc = "Channel0 Configuration Register"]
pub mod ch0_cfr;
#[doc = "Channel1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_cfr](ch1_cfr) module"]
pub type CH1_CFR = crate::Reg<u32, _CH1_CFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_CFR;
#[doc = "`read()` method returns [ch1_cfr::R](ch1_cfr::R) reader structure"]
impl crate::Readable for CH1_CFR {}
#[doc = "`write(|w| ..)` method takes [ch1_cfr::W](ch1_cfr::W) writer structure"]
impl crate::Writable for CH1_CFR {}
#[doc = "Channel1 Configuration Register"]
pub mod ch1_cfr;
#[doc = "Channel2 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_cfr](ch2_cfr) module"]
pub type CH2_CFR = crate::Reg<u32, _CH2_CFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_CFR;
#[doc = "`read()` method returns [ch2_cfr::R](ch2_cfr::R) reader structure"]
impl crate::Readable for CH2_CFR {}
#[doc = "`write(|w| ..)` method takes [ch2_cfr::W](ch2_cfr::W) writer structure"]
impl crate::Writable for CH2_CFR {}
#[doc = "Channel2 Configuration Register"]
pub mod ch2_cfr;
#[doc = "Channel3 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_cfr](ch3_cfr) module"]
pub type CH3_CFR = crate::Reg<u32, _CH3_CFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_CFR;
#[doc = "`read()` method returns [ch3_cfr::R](ch3_cfr::R) reader structure"]
impl crate::Readable for CH3_CFR {}
#[doc = "`write(|w| ..)` method takes [ch3_cfr::W](ch3_cfr::W) writer structure"]
impl crate::Writable for CH3_CFR {}
#[doc = "Channel3 Configuration Register"]
pub mod ch3_cfr;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckr](ckr) module"]
pub type CKR = crate::Reg<u32, _CKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKR;
#[doc = "`read()` method returns [ckr::R](ckr::R) reader structure"]
impl crate::Readable for CKR {}
#[doc = "`write(|w| ..)` method takes [ckr::W](ckr::W) writer structure"]
impl crate::Writable for CKR {}
#[doc = "Clock Configuration Register"]
pub mod ckr;
#[doc = "DMA and Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [di](di) module"]
pub type DI = crate::Reg<u32, _DI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DI;
#[doc = "`read()` method returns [di::R](di::R) reader structure"]
impl crate::Readable for DI {}
#[doc = "`write(|w| ..)` method takes [di::W](di::W) writer structure"]
impl crate::Writable for DI {}
#[doc = "DMA and Interrupt Register"]
pub mod di;
#[doc = "Channel0 Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_dr](ch0_dr) module"]
pub type CH0_DR = crate::Reg<u32, _CH0_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_DR;
#[doc = "`read()` method returns [ch0_dr::R](ch0_dr::R) reader structure"]
impl crate::Readable for CH0_DR {}
#[doc = "`write(|w| ..)` method takes [ch0_dr::W](ch0_dr::W) writer structure"]
impl crate::Writable for CH0_DR {}
#[doc = "Channel0 Delay Register"]
pub mod ch0_dr;
#[doc = "Channel1 Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_dr](ch1_dr) module"]
pub type CH1_DR = crate::Reg<u32, _CH1_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_DR;
#[doc = "`read()` method returns [ch1_dr::R](ch1_dr::R) reader structure"]
impl crate::Readable for CH1_DR {}
#[doc = "`write(|w| ..)` method takes [ch1_dr::W](ch1_dr::W) writer structure"]
impl crate::Writable for CH1_DR {}
#[doc = "Channel1 Delay Register"]
pub mod ch1_dr;
#[doc = "Channel2 Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_dr](ch2_dr) module"]
pub type CH2_DR = crate::Reg<u32, _CH2_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_DR;
#[doc = "`read()` method returns [ch2_dr::R](ch2_dr::R) reader structure"]
impl crate::Readable for CH2_DR {}
#[doc = "`write(|w| ..)` method takes [ch2_dr::W](ch2_dr::W) writer structure"]
impl crate::Writable for CH2_DR {}
#[doc = "Channel2 Delay Register"]
pub mod ch2_dr;
#[doc = "Channel3 Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_dr](ch3_dr) module"]
pub type CH3_DR = crate::Reg<u32, _CH3_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_DR;
#[doc = "`read()` method returns [ch3_dr::R](ch3_dr::R) reader structure"]
impl crate::Readable for CH3_DR {}
#[doc = "`write(|w| ..)` method takes [ch3_dr::W](ch3_dr::W) writer structure"]
impl crate::Writable for CH3_DR {}
#[doc = "Channel3 Delay Register"]
pub mod ch3_dr;
#[doc = "Channel0 Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_rr](ch0_rr) module"]
pub type CH0_RR = crate::Reg<u32, _CH0_RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_RR;
#[doc = "`read()` method returns [ch0_rr::R](ch0_rr::R) reader structure"]
impl crate::Readable for CH0_RR {}
#[doc = "Channel0 Result Register"]
pub mod ch0_rr;
#[doc = "Channel1 Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_rr](ch1_rr) module"]
pub type CH1_RR = crate::Reg<u32, _CH1_RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_RR;
#[doc = "`read()` method returns [ch1_rr::R](ch1_rr::R) reader structure"]
impl crate::Readable for CH1_RR {}
#[doc = "Channel1 Result Register"]
pub mod ch1_rr;
#[doc = "Channel2 Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_rr](ch2_rr) module"]
pub type CH2_RR = crate::Reg<u32, _CH2_RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_RR;
#[doc = "`read()` method returns [ch2_rr::R](ch2_rr::R) reader structure"]
impl crate::Readable for CH2_RR {}
#[doc = "Channel2 Result Register"]
pub mod ch2_rr;
#[doc = "Channel3 Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_rr](ch3_rr) module"]
pub type CH3_RR = crate::Reg<u32, _CH3_RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_RR;
#[doc = "`read()` method returns [ch3_rr::R](ch3_rr::R) reader structure"]
impl crate::Readable for CH3_RR {}
#[doc = "Channel3 Result Register"]
pub mod ch3_rr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register"]
pub mod sr;
