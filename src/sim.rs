#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Options Register 1"]
    pub sopt1: SOPT1,
    #[doc = "0x04 - SOPT1 Configuration Register"]
    pub sopt1_cfg: SOPT1_CFG,
    _reserved2: [u8; 4092usize],
    #[doc = "0x1004 - System Control Register"]
    pub ctrl_reg: CTRL_REG,
    _reserved3: [u8; 28usize],
    #[doc = "0x1024 - System Device Identification Register"]
    pub sdid: SDID,
    _reserved4: [u8; 12usize],
    #[doc = "0x1034 - System Clock Gating Control Register 4"]
    pub scgc4: SCGC4,
    #[doc = "0x1038 - System Clock Gating Control Register 5"]
    pub scgc5: SCGC5,
    #[doc = "0x103c - System Clock Gating Control Register 6"]
    pub scgc6: SCGC6,
    #[doc = "0x1040 - System Clock Gating Control Register 7"]
    pub scgc7: SCGC7,
    #[doc = "0x1044 - System Clock Divider Register 1"]
    pub clkdiv1: CLKDIV1,
    _reserved9: [u8; 4usize],
    #[doc = "0x104c - Flash Configuration Register 1"]
    pub fcfg1: FCFG1,
    #[doc = "0x1050 - Flash Configuration Register 2"]
    pub fcfg2: FCFG2,
    #[doc = "0x1054 - Unique Identification Register High"]
    pub uidh: UIDH,
    #[doc = "0x1058 - Unique Identification Register Mid-High"]
    pub uidmh: UIDMH,
    #[doc = "0x105c - Unique Identification Register Mid-Low"]
    pub uidml: UIDML,
    #[doc = "0x1060 - Unique Identification Register Low"]
    pub uidl: UIDL,
    _reserved15: [u8; 8usize],
    #[doc = "0x106c - Miscellaneous Control Register"]
    pub misc_ctl: MISC_CTL,
    _reserved16: [u8; 88usize],
    #[doc = "0x10c8 - ADC Compensation Register 0"]
    pub adc_comp0: ADC_COMP0,
    #[doc = "0x10cc - ADC Compensation Register 1"]
    pub adc_comp1: ADC_COMP1,
}
#[doc = "System Options Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt1](sopt1) module"]
pub type SOPT1 = crate::Reg<u32, _SOPT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOPT1;
#[doc = "`read()` method returns [sopt1::R](sopt1::R) reader structure"]
impl crate::Readable for SOPT1 {}
#[doc = "`write(|w| ..)` method takes [sopt1::W](sopt1::W) writer structure"]
impl crate::Writable for SOPT1 {}
#[doc = "System Options Register 1"]
pub mod sopt1;
#[doc = "SOPT1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt1_cfg](sopt1_cfg) module"]
pub type SOPT1_CFG = crate::Reg<u32, _SOPT1_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOPT1_CFG;
#[doc = "`read()` method returns [sopt1_cfg::R](sopt1_cfg::R) reader structure"]
impl crate::Readable for SOPT1_CFG {}
#[doc = "`write(|w| ..)` method takes [sopt1_cfg::W](sopt1_cfg::W) writer structure"]
impl crate::Writable for SOPT1_CFG {}
#[doc = "SOPT1 Configuration Register"]
pub mod sopt1_cfg;
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_reg](ctrl_reg) module"]
pub type CTRL_REG = crate::Reg<u32, _CTRL_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL_REG;
#[doc = "`read()` method returns [ctrl_reg::R](ctrl_reg::R) reader structure"]
impl crate::Readable for CTRL_REG {}
#[doc = "`write(|w| ..)` method takes [ctrl_reg::W](ctrl_reg::W) writer structure"]
impl crate::Writable for CTRL_REG {}
#[doc = "System Control Register"]
pub mod ctrl_reg;
#[doc = "System Device Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdid](sdid) module"]
pub type SDID = crate::Reg<u32, _SDID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDID;
#[doc = "`read()` method returns [sdid::R](sdid::R) reader structure"]
impl crate::Readable for SDID {}
#[doc = "System Device Identification Register"]
pub mod sdid;
#[doc = "System Clock Gating Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc4](scgc4) module"]
pub type SCGC4 = crate::Reg<u32, _SCGC4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC4;
#[doc = "`read()` method returns [scgc4::R](scgc4::R) reader structure"]
impl crate::Readable for SCGC4 {}
#[doc = "`write(|w| ..)` method takes [scgc4::W](scgc4::W) writer structure"]
impl crate::Writable for SCGC4 {}
#[doc = "System Clock Gating Control Register 4"]
pub mod scgc4;
#[doc = "System Clock Gating Control Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc5](scgc5) module"]
pub type SCGC5 = crate::Reg<u32, _SCGC5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC5;
#[doc = "`read()` method returns [scgc5::R](scgc5::R) reader structure"]
impl crate::Readable for SCGC5 {}
#[doc = "`write(|w| ..)` method takes [scgc5::W](scgc5::W) writer structure"]
impl crate::Writable for SCGC5 {}
#[doc = "System Clock Gating Control Register 5"]
pub mod scgc5;
#[doc = "System Clock Gating Control Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc6](scgc6) module"]
pub type SCGC6 = crate::Reg<u32, _SCGC6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC6;
#[doc = "`read()` method returns [scgc6::R](scgc6::R) reader structure"]
impl crate::Readable for SCGC6 {}
#[doc = "`write(|w| ..)` method takes [scgc6::W](scgc6::W) writer structure"]
impl crate::Writable for SCGC6 {}
#[doc = "System Clock Gating Control Register 6"]
pub mod scgc6;
#[doc = "System Clock Gating Control Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc7](scgc7) module"]
pub type SCGC7 = crate::Reg<u32, _SCGC7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC7;
#[doc = "`read()` method returns [scgc7::R](scgc7::R) reader structure"]
impl crate::Readable for SCGC7 {}
#[doc = "`write(|w| ..)` method takes [scgc7::W](scgc7::W) writer structure"]
impl crate::Writable for SCGC7 {}
#[doc = "System Clock Gating Control Register 7"]
pub mod scgc7;
#[doc = "System Clock Divider Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv1](clkdiv1) module"]
pub type CLKDIV1 = crate::Reg<u32, _CLKDIV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKDIV1;
#[doc = "`read()` method returns [clkdiv1::R](clkdiv1::R) reader structure"]
impl crate::Readable for CLKDIV1 {}
#[doc = "`write(|w| ..)` method takes [clkdiv1::W](clkdiv1::W) writer structure"]
impl crate::Writable for CLKDIV1 {}
#[doc = "System Clock Divider Register 1"]
pub mod clkdiv1;
#[doc = "Flash Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg1](fcfg1) module"]
pub type FCFG1 = crate::Reg<u32, _FCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG1;
#[doc = "`read()` method returns [fcfg1::R](fcfg1::R) reader structure"]
impl crate::Readable for FCFG1 {}
#[doc = "`write(|w| ..)` method takes [fcfg1::W](fcfg1::W) writer structure"]
impl crate::Writable for FCFG1 {}
#[doc = "Flash Configuration Register 1"]
pub mod fcfg1;
#[doc = "Flash Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg2](fcfg2) module"]
pub type FCFG2 = crate::Reg<u32, _FCFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG2;
#[doc = "`read()` method returns [fcfg2::R](fcfg2::R) reader structure"]
impl crate::Readable for FCFG2 {}
#[doc = "Flash Configuration Register 2"]
pub mod fcfg2;
#[doc = "Unique Identification Register High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uidh](uidh) module"]
pub type UIDH = crate::Reg<u32, _UIDH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UIDH;
#[doc = "`read()` method returns [uidh::R](uidh::R) reader structure"]
impl crate::Readable for UIDH {}
#[doc = "Unique Identification Register High"]
pub mod uidh;
#[doc = "Unique Identification Register Mid-High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uidmh](uidmh) module"]
pub type UIDMH = crate::Reg<u32, _UIDMH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UIDMH;
#[doc = "`read()` method returns [uidmh::R](uidmh::R) reader structure"]
impl crate::Readable for UIDMH {}
#[doc = "Unique Identification Register Mid-High"]
pub mod uidmh;
#[doc = "Unique Identification Register Mid-Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uidml](uidml) module"]
pub type UIDML = crate::Reg<u32, _UIDML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UIDML;
#[doc = "`read()` method returns [uidml::R](uidml::R) reader structure"]
impl crate::Readable for UIDML {}
#[doc = "Unique Identification Register Mid-Low"]
pub mod uidml;
#[doc = "Unique Identification Register Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uidl](uidl) module"]
pub type UIDL = crate::Reg<u32, _UIDL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UIDL;
#[doc = "`read()` method returns [uidl::R](uidl::R) reader structure"]
impl crate::Readable for UIDL {}
#[doc = "Unique Identification Register Low"]
pub mod uidl;
#[doc = "Miscellaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_ctl](misc_ctl) module"]
pub type MISC_CTL = crate::Reg<u32, _MISC_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC_CTL;
#[doc = "`read()` method returns [misc_ctl::R](misc_ctl::R) reader structure"]
impl crate::Readable for MISC_CTL {}
#[doc = "`write(|w| ..)` method takes [misc_ctl::W](misc_ctl::W) writer structure"]
impl crate::Writable for MISC_CTL {}
#[doc = "Miscellaneous Control Register"]
pub mod misc_ctl;
#[doc = "ADC Compensation Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_comp0](adc_comp0) module"]
pub type ADC_COMP0 = crate::Reg<u32, _ADC_COMP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_COMP0;
#[doc = "`read()` method returns [adc_comp0::R](adc_comp0::R) reader structure"]
impl crate::Readable for ADC_COMP0 {}
#[doc = "ADC Compensation Register 0"]
pub mod adc_comp0;
#[doc = "ADC Compensation Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_comp1](adc_comp1) module"]
pub type ADC_COMP1 = crate::Reg<u32, _ADC_COMP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_COMP1;
#[doc = "`read()` method returns [adc_comp1::R](adc_comp1::R) reader structure"]
impl crate::Readable for ADC_COMP1 {}
#[doc = "ADC Compensation Register 1"]
pub mod adc_comp1;
