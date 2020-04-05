#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Operand Register X0"]
    pub x0: X0,
    #[doc = "0x04 - Operand Register X1"]
    pub x1: X1,
    #[doc = "0x08 - Operand Register X2"]
    pub x2: X2,
    #[doc = "0x0c - Operand Register X3"]
    pub x3: X3,
    #[doc = "0x10 - Accumulator Register A0"]
    pub a0: A0,
    #[doc = "0x14 - Accumulator Register A1"]
    pub a1: A1,
    #[doc = "0x18 - Control/Status Register"]
    pub csr: CSR,
    #[doc = "0x1c - CSR Interrupt Flags Clearance Register"]
    pub csr_if_clr: CSR_IF_CLR,
}
#[doc = "Operand Register X0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [x0](x0) module"]
pub type X0 = crate::Reg<u32, _X0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _X0;
#[doc = "`read()` method returns [x0::R](x0::R) reader structure"]
impl crate::Readable for X0 {}
#[doc = "`write(|w| ..)` method takes [x0::W](x0::W) writer structure"]
impl crate::Writable for X0 {}
#[doc = "Operand Register X0"]
pub mod x0;
#[doc = "Operand Register X1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [x1](x1) module"]
pub type X1 = crate::Reg<u32, _X1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _X1;
#[doc = "`read()` method returns [x1::R](x1::R) reader structure"]
impl crate::Readable for X1 {}
#[doc = "`write(|w| ..)` method takes [x1::W](x1::W) writer structure"]
impl crate::Writable for X1 {}
#[doc = "Operand Register X1"]
pub mod x1;
#[doc = "Operand Register X2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [x2](x2) module"]
pub type X2 = crate::Reg<u32, _X2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _X2;
#[doc = "`read()` method returns [x2::R](x2::R) reader structure"]
impl crate::Readable for X2 {}
#[doc = "`write(|w| ..)` method takes [x2::W](x2::W) writer structure"]
impl crate::Writable for X2 {}
#[doc = "Operand Register X2"]
pub mod x2;
#[doc = "Operand Register X3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [x3](x3) module"]
pub type X3 = crate::Reg<u32, _X3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _X3;
#[doc = "`read()` method returns [x3::R](x3::R) reader structure"]
impl crate::Readable for X3 {}
#[doc = "`write(|w| ..)` method takes [x3::W](x3::W) writer structure"]
impl crate::Writable for X3 {}
#[doc = "Operand Register X3"]
pub mod x3;
#[doc = "Accumulator Register A0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a0](a0) module"]
pub type A0 = crate::Reg<u32, _A0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _A0;
#[doc = "`read()` method returns [a0::R](a0::R) reader structure"]
impl crate::Readable for A0 {}
#[doc = "`write(|w| ..)` method takes [a0::W](a0::W) writer structure"]
impl crate::Writable for A0 {}
#[doc = "Accumulator Register A0"]
pub mod a0;
#[doc = "Accumulator Register A1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a1](a1) module"]
pub type A1 = crate::Reg<u32, _A1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _A1;
#[doc = "`read()` method returns [a1::R](a1::R) reader structure"]
impl crate::Readable for A1 {}
#[doc = "`write(|w| ..)` method takes [a1::W](a1::W) writer structure"]
impl crate::Writable for A1 {}
#[doc = "Accumulator Register A1"]
pub mod a1;
#[doc = "Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "Control/Status Register"]
pub mod csr;
#[doc = "CSR Interrupt Flags Clearance Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr_if_clr](csr_if_clr) module"]
pub type CSR_IF_CLR = crate::Reg<u32, _CSR_IF_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR_IF_CLR;
#[doc = "`read()` method returns [csr_if_clr::R](csr_if_clr::R) reader structure"]
impl crate::Readable for CSR_IF_CLR {}
#[doc = "`write(|w| ..)` method takes [csr_if_clr::W](csr_if_clr::W) writer structure"]
impl crate::Writable for CSR_IF_CLR {}
#[doc = "CSR Interrupt Flags Clearance Register"]
pub mod csr_if_clr;
