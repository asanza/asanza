#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin Control Register n"]
    pub pcr0: PCR0,
    #[doc = "0x04 - Pin Control Register n"]
    pub pcr1: PCR1,
    #[doc = "0x08 - Pin Control Register n"]
    pub pcr2: PCR2,
    #[doc = "0x0c - Pin Control Register n"]
    pub pcr3: PCR3,
    #[doc = "0x10 - Pin Control Register n"]
    pub pcr4: PCR4,
    #[doc = "0x14 - Pin Control Register n"]
    pub pcr5: PCR5,
    #[doc = "0x18 - Pin Control Register n"]
    pub pcr6: PCR6,
    #[doc = "0x1c - Pin Control Register n"]
    pub pcr7: PCR7,
    _reserved8: [u8; 96usize],
    #[doc = "0x80 - Global Pin Control Low Register"]
    pub gpclr: GPCLR,
    #[doc = "0x84 - Global Pin Control High Register"]
    pub gpchr: GPCHR,
    _reserved10: [u8; 24usize],
    #[doc = "0xa0 - Interrupt Status Flag Register"]
    pub isfr: ISFR,
}
#[doc = "Pin Control Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr0](pcr0) module"]
pub type PCR0 = crate::Reg<u32, _PCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR0;
#[doc = "`read()` method returns [pcr0::R](pcr0::R) reader structure"]
impl crate::Readable for PCR0 {}
#[doc = "`write(|w| ..)` method takes [pcr0::W](pcr0::W) writer structure"]
impl crate::Writable for PCR0 {}
#[doc = "Pin Control Register n"]
pub mod pcr0;
#[doc = "Pin Control Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr1](pcr1) module"]
pub type PCR1 = crate::Reg<u32, _PCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR1;
#[doc = "`read()` method returns [pcr1::R](pcr1::R) reader structure"]
impl crate::Readable for PCR1 {}
#[doc = "`write(|w| ..)` method takes [pcr1::W](pcr1::W) writer structure"]
impl crate::Writable for PCR1 {}
#[doc = "Pin Control Register n"]
pub mod pcr1;
#[doc = "Pin Control Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr2](pcr2) module"]
pub type PCR2 = crate::Reg<u32, _PCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR2;
#[doc = "`read()` method returns [pcr2::R](pcr2::R) reader structure"]
impl crate::Readable for PCR2 {}
#[doc = "`write(|w| ..)` method takes [pcr2::W](pcr2::W) writer structure"]
impl crate::Writable for PCR2 {}
#[doc = "Pin Control Register n"]
pub mod pcr2;
#[doc = "Pin Control Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr3](pcr3) module"]
pub type PCR3 = crate::Reg<u32, _PCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR3;
#[doc = "`read()` method returns [pcr3::R](pcr3::R) reader structure"]
impl crate::Readable for PCR3 {}
#[doc = "`write(|w| ..)` method takes [pcr3::W](pcr3::W) writer structure"]
impl crate::Writable for PCR3 {}
#[doc = "Pin Control Register n"]
pub mod pcr3;
#[doc = "Pin Control Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr4](pcr4) module"]
pub type PCR4 = crate::Reg<u32, _PCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR4;
#[doc = "`read()` method returns [pcr4::R](pcr4::R) reader structure"]
impl crate::Readable for PCR4 {}
#[doc = "`write(|w| ..)` method takes [pcr4::W](pcr4::W) writer structure"]
impl crate::Writable for PCR4 {}
#[doc = "Pin Control Register n"]
pub mod pcr4;
#[doc = "Pin Control Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr5](pcr5) module"]
pub type PCR5 = crate::Reg<u32, _PCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR5;
#[doc = "`read()` method returns [pcr5::R](pcr5::R) reader structure"]
impl crate::Readable for PCR5 {}
#[doc = "`write(|w| ..)` method takes [pcr5::W](pcr5::W) writer structure"]
impl crate::Writable for PCR5 {}
#[doc = "Pin Control Register n"]
pub mod pcr5;
#[doc = "Pin Control Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr6](pcr6) module"]
pub type PCR6 = crate::Reg<u32, _PCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR6;
#[doc = "`read()` method returns [pcr6::R](pcr6::R) reader structure"]
impl crate::Readable for PCR6 {}
#[doc = "`write(|w| ..)` method takes [pcr6::W](pcr6::W) writer structure"]
impl crate::Writable for PCR6 {}
#[doc = "Pin Control Register n"]
pub mod pcr6;
#[doc = "Pin Control Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr7](pcr7) module"]
pub type PCR7 = crate::Reg<u32, _PCR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR7;
#[doc = "`read()` method returns [pcr7::R](pcr7::R) reader structure"]
impl crate::Readable for PCR7 {}
#[doc = "`write(|w| ..)` method takes [pcr7::W](pcr7::W) writer structure"]
impl crate::Writable for PCR7 {}
#[doc = "Pin Control Register n"]
pub mod pcr7;
#[doc = "Global Pin Control Low Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpclr](gpclr) module"]
pub type GPCLR = crate::Reg<u32, _GPCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPCLR;
#[doc = "`write(|w| ..)` method takes [gpclr::W](gpclr::W) writer structure"]
impl crate::Writable for GPCLR {}
#[doc = "Global Pin Control Low Register"]
pub mod gpclr;
#[doc = "Global Pin Control High Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpchr](gpchr) module"]
pub type GPCHR = crate::Reg<u32, _GPCHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPCHR;
#[doc = "`write(|w| ..)` method takes [gpchr::W](gpchr::W) writer structure"]
impl crate::Writable for GPCHR {}
#[doc = "Global Pin Control High Register"]
pub mod gpchr;
#[doc = "Interrupt Status Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isfr](isfr) module"]
pub type ISFR = crate::Reg<u32, _ISFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISFR;
#[doc = "`read()` method returns [isfr::R](isfr::R) reader structure"]
impl crate::Readable for ISFR {}
#[doc = "`write(|w| ..)` method takes [isfr::W](isfr::W) writer structure"]
impl crate::Writable for ISFR {}
#[doc = "Interrupt Status Flag Register"]
pub mod isfr;
