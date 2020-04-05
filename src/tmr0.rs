#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Channel Compare Register 1"]
    pub comp1: COMP1,
    #[doc = "0x02 - Timer Channel Compare Register 2"]
    pub comp2: COMP2,
    #[doc = "0x04 - Timer Channel Capture Register"]
    pub capt: CAPT,
    #[doc = "0x06 - Timer Channel Load Register"]
    pub load: LOAD,
    #[doc = "0x08 - Timer Channel Hold Register"]
    pub hold: HOLD,
    #[doc = "0x0a - Timer Channel Counter Register"]
    pub cntr: CNTR,
    #[doc = "0x0c - Timer Channel Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x0e - Timer Channel Status and Control Register"]
    pub sctrl: SCTRL,
    #[doc = "0x10 - Timer Channel Comparator Load Register 1"]
    pub cmpld1: CMPLD1,
    #[doc = "0x12 - Timer Channel Comparator Load Register 2"]
    pub cmpld2: CMPLD2,
    #[doc = "0x14 - Timer Channel Comparator Status and Control Register"]
    pub csctrl: CSCTRL,
    #[doc = "0x16 - Timer Channel Input Filter Register"]
    pub filt: FILT,
    _reserved12: [u8; 6usize],
    #[doc = "0x1e - Timer Channel Enable Register"]
    pub enbl: ENBL,
}
#[doc = "Timer Channel Compare Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1](comp1) module"]
pub type COMP1 = crate::Reg<u16, _COMP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP1;
#[doc = "`read()` method returns [comp1::R](comp1::R) reader structure"]
impl crate::Readable for COMP1 {}
#[doc = "`write(|w| ..)` method takes [comp1::W](comp1::W) writer structure"]
impl crate::Writable for COMP1 {}
#[doc = "Timer Channel Compare Register 1"]
pub mod comp1;
#[doc = "Timer Channel Compare Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2](comp2) module"]
pub type COMP2 = crate::Reg<u16, _COMP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP2;
#[doc = "`read()` method returns [comp2::R](comp2::R) reader structure"]
impl crate::Readable for COMP2 {}
#[doc = "`write(|w| ..)` method takes [comp2::W](comp2::W) writer structure"]
impl crate::Writable for COMP2 {}
#[doc = "Timer Channel Compare Register 2"]
pub mod comp2;
#[doc = "Timer Channel Capture Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capt](capt) module"]
pub type CAPT = crate::Reg<u16, _CAPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPT;
#[doc = "`read()` method returns [capt::R](capt::R) reader structure"]
impl crate::Readable for CAPT {}
#[doc = "`write(|w| ..)` method takes [capt::W](capt::W) writer structure"]
impl crate::Writable for CAPT {}
#[doc = "Timer Channel Capture Register"]
pub mod capt;
#[doc = "Timer Channel Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load](load) module"]
pub type LOAD = crate::Reg<u16, _LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOAD;
#[doc = "`read()` method returns [load::R](load::R) reader structure"]
impl crate::Readable for LOAD {}
#[doc = "`write(|w| ..)` method takes [load::W](load::W) writer structure"]
impl crate::Writable for LOAD {}
#[doc = "Timer Channel Load Register"]
pub mod load;
#[doc = "Timer Channel Hold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hold](hold) module"]
pub type HOLD = crate::Reg<u16, _HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOLD;
#[doc = "`read()` method returns [hold::R](hold::R) reader structure"]
impl crate::Readable for HOLD {}
#[doc = "`write(|w| ..)` method takes [hold::W](hold::W) writer structure"]
impl crate::Writable for HOLD {}
#[doc = "Timer Channel Hold Register"]
pub mod hold;
#[doc = "Timer Channel Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntr](cntr) module"]
pub type CNTR = crate::Reg<u16, _CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTR;
#[doc = "`read()` method returns [cntr::R](cntr::R) reader structure"]
impl crate::Readable for CNTR {}
#[doc = "`write(|w| ..)` method takes [cntr::W](cntr::W) writer structure"]
impl crate::Writable for CNTR {}
#[doc = "Timer Channel Counter Register"]
pub mod cntr;
#[doc = "Timer Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u16, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Timer Channel Control Register"]
pub mod ctrl;
#[doc = "Timer Channel Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctrl](sctrl) module"]
pub type SCTRL = crate::Reg<u16, _SCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTRL;
#[doc = "`read()` method returns [sctrl::R](sctrl::R) reader structure"]
impl crate::Readable for SCTRL {}
#[doc = "`write(|w| ..)` method takes [sctrl::W](sctrl::W) writer structure"]
impl crate::Writable for SCTRL {}
#[doc = "Timer Channel Status and Control Register"]
pub mod sctrl;
#[doc = "Timer Channel Comparator Load Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpld1](cmpld1) module"]
pub type CMPLD1 = crate::Reg<u16, _CMPLD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPLD1;
#[doc = "`read()` method returns [cmpld1::R](cmpld1::R) reader structure"]
impl crate::Readable for CMPLD1 {}
#[doc = "`write(|w| ..)` method takes [cmpld1::W](cmpld1::W) writer structure"]
impl crate::Writable for CMPLD1 {}
#[doc = "Timer Channel Comparator Load Register 1"]
pub mod cmpld1;
#[doc = "Timer Channel Comparator Load Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpld2](cmpld2) module"]
pub type CMPLD2 = crate::Reg<u16, _CMPLD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPLD2;
#[doc = "`read()` method returns [cmpld2::R](cmpld2::R) reader structure"]
impl crate::Readable for CMPLD2 {}
#[doc = "`write(|w| ..)` method takes [cmpld2::W](cmpld2::W) writer structure"]
impl crate::Writable for CMPLD2 {}
#[doc = "Timer Channel Comparator Load Register 2"]
pub mod cmpld2;
#[doc = "Timer Channel Comparator Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctrl](csctrl) module"]
pub type CSCTRL = crate::Reg<u16, _CSCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTRL;
#[doc = "`read()` method returns [csctrl::R](csctrl::R) reader structure"]
impl crate::Readable for CSCTRL {}
#[doc = "`write(|w| ..)` method takes [csctrl::W](csctrl::W) writer structure"]
impl crate::Writable for CSCTRL {}
#[doc = "Timer Channel Comparator Status and Control Register"]
pub mod csctrl;
#[doc = "Timer Channel Input Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filt](filt) module"]
pub type FILT = crate::Reg<u16, _FILT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILT;
#[doc = "`read()` method returns [filt::R](filt::R) reader structure"]
impl crate::Readable for FILT {}
#[doc = "`write(|w| ..)` method takes [filt::W](filt::W) writer structure"]
impl crate::Writable for FILT {}
#[doc = "Timer Channel Input Filter Register"]
pub mod filt;
#[doc = "Timer Channel Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enbl](enbl) module"]
pub type ENBL = crate::Reg<u16, _ENBL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENBL;
#[doc = "`read()` method returns [enbl::R](enbl::R) reader structure"]
impl crate::Readable for ENBL {}
#[doc = "`write(|w| ..)` method takes [enbl::W](enbl::W) writer structure"]
impl crate::Writable for ENBL {}
#[doc = "Timer Channel Enable Register"]
pub mod enbl;
