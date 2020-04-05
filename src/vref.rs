#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - VREF Trim Register"]
    pub vrefh_trm: VREFH_TRM,
    #[doc = "0x01 - VREF Status and Control Register"]
    pub vrefh_sc: VREFH_SC,
    _reserved2: [u8; 3usize],
    #[doc = "0x05 - VREFL TRIM Register"]
    pub vrefl_trm: VREFL_TRM,
}
#[doc = "VREF Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vrefh_trm](vrefh_trm) module"]
pub type VREFH_TRM = crate::Reg<u8, _VREFH_TRM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREFH_TRM;
#[doc = "`read()` method returns [vrefh_trm::R](vrefh_trm::R) reader structure"]
impl crate::Readable for VREFH_TRM {}
#[doc = "`write(|w| ..)` method takes [vrefh_trm::W](vrefh_trm::W) writer structure"]
impl crate::Writable for VREFH_TRM {}
#[doc = "VREF Trim Register"]
pub mod vrefh_trm;
#[doc = "VREF Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vrefh_sc](vrefh_sc) module"]
pub type VREFH_SC = crate::Reg<u8, _VREFH_SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREFH_SC;
#[doc = "`read()` method returns [vrefh_sc::R](vrefh_sc::R) reader structure"]
impl crate::Readable for VREFH_SC {}
#[doc = "`write(|w| ..)` method takes [vrefh_sc::W](vrefh_sc::W) writer structure"]
impl crate::Writable for VREFH_SC {}
#[doc = "VREF Status and Control Register"]
pub mod vrefh_sc;
#[doc = "VREFL TRIM Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vrefl_trm](vrefl_trm) module"]
pub type VREFL_TRM = crate::Reg<u8, _VREFL_TRM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREFL_TRM;
#[doc = "`read()` method returns [vrefl_trm::R](vrefl_trm::R) reader structure"]
impl crate::Readable for VREFL_TRM {}
#[doc = "`write(|w| ..)` method takes [vrefl_trm::W](vrefl_trm::W) writer structure"]
impl crate::Writable for VREFL_TRM {}
#[doc = "VREFL TRIM Register"]
pub mod vrefl_trm;
