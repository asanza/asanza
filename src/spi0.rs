#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Status Register"]
    pub s: S,
    #[doc = "0x01 - SPI Baud Rate Register"]
    pub br: BR,
    #[doc = "0x02 - SPI Control Register 2"]
    pub c2: C2,
    #[doc = "0x03 - SPI Control Register 1"]
    pub c1: C1,
    #[doc = "0x04 - SPI Match Register low"]
    pub ml: ML,
    #[doc = "0x05 - SPI match register high"]
    pub mh: MH,
    #[doc = "0x06 - SPI Data Register low"]
    pub dl: DL,
    #[doc = "0x07 - SPI data register high"]
    pub dh: DH,
}
#[doc = "SPI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s](s) module"]
pub type S = crate::Reg<u8, _S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S;
#[doc = "`read()` method returns [s::R](s::R) reader structure"]
impl crate::Readable for S {}
#[doc = "`write(|w| ..)` method takes [s::W](s::W) writer structure"]
impl crate::Writable for S {}
#[doc = "SPI Status Register"]
pub mod s;
#[doc = "SPI Baud Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [br](br) module"]
pub type BR = crate::Reg<u8, _BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BR;
#[doc = "`read()` method returns [br::R](br::R) reader structure"]
impl crate::Readable for BR {}
#[doc = "`write(|w| ..)` method takes [br::W](br::W) writer structure"]
impl crate::Writable for BR {}
#[doc = "SPI Baud Rate Register"]
pub mod br;
#[doc = "SPI Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](c2) module"]
pub type C2 = crate::Reg<u8, _C2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2;
#[doc = "`read()` method returns [c2::R](c2::R) reader structure"]
impl crate::Readable for C2 {}
#[doc = "`write(|w| ..)` method takes [c2::W](c2::W) writer structure"]
impl crate::Writable for C2 {}
#[doc = "SPI Control Register 2"]
pub mod c2;
#[doc = "SPI Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1](c1) module"]
pub type C1 = crate::Reg<u8, _C1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1;
#[doc = "`read()` method returns [c1::R](c1::R) reader structure"]
impl crate::Readable for C1 {}
#[doc = "`write(|w| ..)` method takes [c1::W](c1::W) writer structure"]
impl crate::Writable for C1 {}
#[doc = "SPI Control Register 1"]
pub mod c1;
#[doc = "SPI Match Register low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ml](ml) module"]
pub type ML = crate::Reg<u8, _ML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ML;
#[doc = "`read()` method returns [ml::R](ml::R) reader structure"]
impl crate::Readable for ML {}
#[doc = "`write(|w| ..)` method takes [ml::W](ml::W) writer structure"]
impl crate::Writable for ML {}
#[doc = "SPI Match Register low"]
pub mod ml;
#[doc = "SPI match register high\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mh](mh) module"]
pub type MH = crate::Reg<u8, _MH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MH;
#[doc = "`read()` method returns [mh::R](mh::R) reader structure"]
impl crate::Readable for MH {}
#[doc = "`write(|w| ..)` method takes [mh::W](mh::W) writer structure"]
impl crate::Writable for MH {}
#[doc = "SPI match register high"]
pub mod mh;
#[doc = "SPI Data Register low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dl](dl) module"]
pub type DL = crate::Reg<u8, _DL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DL;
#[doc = "`read()` method returns [dl::R](dl::R) reader structure"]
impl crate::Readable for DL {}
#[doc = "`write(|w| ..)` method takes [dl::W](dl::W) writer structure"]
impl crate::Writable for DL {}
#[doc = "SPI Data Register low"]
pub mod dl;
#[doc = "SPI data register high\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dh](dh) module"]
pub type DH = crate::Reg<u8, _DH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DH;
#[doc = "`read()` method returns [dh::R](dh::R) reader structure"]
impl crate::Readable for DH {}
#[doc = "`write(|w| ..)` method takes [dh::W](dh::W) writer structure"]
impl crate::Writable for DH {}
#[doc = "SPI data register high"]
pub mod dh;
