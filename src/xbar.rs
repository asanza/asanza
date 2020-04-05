#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Crossbar Select Register 0"]
    pub sel0: SEL0,
    #[doc = "0x02 - Crossbar Select Register 1"]
    pub sel1: SEL1,
    #[doc = "0x04 - Crossbar Select Register 2"]
    pub sel2: SEL2,
    #[doc = "0x06 - Crossbar Select Register 3"]
    pub sel3: SEL3,
    #[doc = "0x08 - Crossbar Select Register 4"]
    pub sel4: SEL4,
    #[doc = "0x0a - Crossbar Select Register 5"]
    pub sel5: SEL5,
    #[doc = "0x0c - Crossbar Select Register 6"]
    pub sel6: SEL6,
    #[doc = "0x0e - Crossbar Select Register 7"]
    pub sel7: SEL7,
    #[doc = "0x10 - Crossbar Select Register 8"]
    pub sel8: SEL8,
    #[doc = "0x12 - Crossbar Select Register 9"]
    pub sel9: SEL9,
    #[doc = "0x14 - Crossbar Select Register 10"]
    pub sel10: SEL10,
    #[doc = "0x16 - Crossbar Select Register 11"]
    pub sel11: SEL11,
    #[doc = "0x18 - Crossbar Select Register 12"]
    pub sel12: SEL12,
    #[doc = "0x1a - Crossbar Select Register 13"]
    pub sel13: SEL13,
    #[doc = "0x1c - Crossbar Select Register 14"]
    pub sel14: SEL14,
    #[doc = "0x1e - Crossbar Select Register 15"]
    pub sel15: SEL15,
    #[doc = "0x20 - Crossbar Select Register 16"]
    pub sel16: SEL16,
    #[doc = "0x22 - Crossbar Select Register 17"]
    pub sel17: SEL17,
    #[doc = "0x24 - Crossbar Select Register 18"]
    pub sel18: SEL18,
    #[doc = "0x26 - Crossbar Select Register 19"]
    pub sel19: SEL19,
    #[doc = "0x28 - Crossbar Select Register 20"]
    pub sel20: SEL20,
    #[doc = "0x2a - Crossbar Select Register 21"]
    pub sel21: SEL21,
    #[doc = "0x2c - Crossbar Control Register 0"]
    pub ctrl0: CTRL0,
    #[doc = "0x2e - Crossbar Control Register 1"]
    pub ctrl1: CTRL1,
}
#[doc = "Crossbar Select Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel0](sel0) module"]
pub type SEL0 = crate::Reg<u16, _SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL0;
#[doc = "`read()` method returns [sel0::R](sel0::R) reader structure"]
impl crate::Readable for SEL0 {}
#[doc = "`write(|w| ..)` method takes [sel0::W](sel0::W) writer structure"]
impl crate::Writable for SEL0 {}
#[doc = "Crossbar Select Register 0"]
pub mod sel0;
#[doc = "Crossbar Select Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel1](sel1) module"]
pub type SEL1 = crate::Reg<u16, _SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL1;
#[doc = "`read()` method returns [sel1::R](sel1::R) reader structure"]
impl crate::Readable for SEL1 {}
#[doc = "`write(|w| ..)` method takes [sel1::W](sel1::W) writer structure"]
impl crate::Writable for SEL1 {}
#[doc = "Crossbar Select Register 1"]
pub mod sel1;
#[doc = "Crossbar Select Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel2](sel2) module"]
pub type SEL2 = crate::Reg<u16, _SEL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL2;
#[doc = "`read()` method returns [sel2::R](sel2::R) reader structure"]
impl crate::Readable for SEL2 {}
#[doc = "`write(|w| ..)` method takes [sel2::W](sel2::W) writer structure"]
impl crate::Writable for SEL2 {}
#[doc = "Crossbar Select Register 2"]
pub mod sel2;
#[doc = "Crossbar Select Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel3](sel3) module"]
pub type SEL3 = crate::Reg<u16, _SEL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL3;
#[doc = "`read()` method returns [sel3::R](sel3::R) reader structure"]
impl crate::Readable for SEL3 {}
#[doc = "`write(|w| ..)` method takes [sel3::W](sel3::W) writer structure"]
impl crate::Writable for SEL3 {}
#[doc = "Crossbar Select Register 3"]
pub mod sel3;
#[doc = "Crossbar Select Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel4](sel4) module"]
pub type SEL4 = crate::Reg<u16, _SEL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL4;
#[doc = "`read()` method returns [sel4::R](sel4::R) reader structure"]
impl crate::Readable for SEL4 {}
#[doc = "`write(|w| ..)` method takes [sel4::W](sel4::W) writer structure"]
impl crate::Writable for SEL4 {}
#[doc = "Crossbar Select Register 4"]
pub mod sel4;
#[doc = "Crossbar Select Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel5](sel5) module"]
pub type SEL5 = crate::Reg<u16, _SEL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL5;
#[doc = "`read()` method returns [sel5::R](sel5::R) reader structure"]
impl crate::Readable for SEL5 {}
#[doc = "`write(|w| ..)` method takes [sel5::W](sel5::W) writer structure"]
impl crate::Writable for SEL5 {}
#[doc = "Crossbar Select Register 5"]
pub mod sel5;
#[doc = "Crossbar Select Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel6](sel6) module"]
pub type SEL6 = crate::Reg<u16, _SEL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL6;
#[doc = "`read()` method returns [sel6::R](sel6::R) reader structure"]
impl crate::Readable for SEL6 {}
#[doc = "`write(|w| ..)` method takes [sel6::W](sel6::W) writer structure"]
impl crate::Writable for SEL6 {}
#[doc = "Crossbar Select Register 6"]
pub mod sel6;
#[doc = "Crossbar Select Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel7](sel7) module"]
pub type SEL7 = crate::Reg<u16, _SEL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL7;
#[doc = "`read()` method returns [sel7::R](sel7::R) reader structure"]
impl crate::Readable for SEL7 {}
#[doc = "`write(|w| ..)` method takes [sel7::W](sel7::W) writer structure"]
impl crate::Writable for SEL7 {}
#[doc = "Crossbar Select Register 7"]
pub mod sel7;
#[doc = "Crossbar Select Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel8](sel8) module"]
pub type SEL8 = crate::Reg<u16, _SEL8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL8;
#[doc = "`read()` method returns [sel8::R](sel8::R) reader structure"]
impl crate::Readable for SEL8 {}
#[doc = "`write(|w| ..)` method takes [sel8::W](sel8::W) writer structure"]
impl crate::Writable for SEL8 {}
#[doc = "Crossbar Select Register 8"]
pub mod sel8;
#[doc = "Crossbar Select Register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel9](sel9) module"]
pub type SEL9 = crate::Reg<u16, _SEL9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL9;
#[doc = "`read()` method returns [sel9::R](sel9::R) reader structure"]
impl crate::Readable for SEL9 {}
#[doc = "`write(|w| ..)` method takes [sel9::W](sel9::W) writer structure"]
impl crate::Writable for SEL9 {}
#[doc = "Crossbar Select Register 9"]
pub mod sel9;
#[doc = "Crossbar Select Register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel10](sel10) module"]
pub type SEL10 = crate::Reg<u16, _SEL10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL10;
#[doc = "`read()` method returns [sel10::R](sel10::R) reader structure"]
impl crate::Readable for SEL10 {}
#[doc = "`write(|w| ..)` method takes [sel10::W](sel10::W) writer structure"]
impl crate::Writable for SEL10 {}
#[doc = "Crossbar Select Register 10"]
pub mod sel10;
#[doc = "Crossbar Select Register 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel11](sel11) module"]
pub type SEL11 = crate::Reg<u16, _SEL11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL11;
#[doc = "`read()` method returns [sel11::R](sel11::R) reader structure"]
impl crate::Readable for SEL11 {}
#[doc = "`write(|w| ..)` method takes [sel11::W](sel11::W) writer structure"]
impl crate::Writable for SEL11 {}
#[doc = "Crossbar Select Register 11"]
pub mod sel11;
#[doc = "Crossbar Select Register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel12](sel12) module"]
pub type SEL12 = crate::Reg<u16, _SEL12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL12;
#[doc = "`read()` method returns [sel12::R](sel12::R) reader structure"]
impl crate::Readable for SEL12 {}
#[doc = "`write(|w| ..)` method takes [sel12::W](sel12::W) writer structure"]
impl crate::Writable for SEL12 {}
#[doc = "Crossbar Select Register 12"]
pub mod sel12;
#[doc = "Crossbar Select Register 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel13](sel13) module"]
pub type SEL13 = crate::Reg<u16, _SEL13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL13;
#[doc = "`read()` method returns [sel13::R](sel13::R) reader structure"]
impl crate::Readable for SEL13 {}
#[doc = "`write(|w| ..)` method takes [sel13::W](sel13::W) writer structure"]
impl crate::Writable for SEL13 {}
#[doc = "Crossbar Select Register 13"]
pub mod sel13;
#[doc = "Crossbar Select Register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel14](sel14) module"]
pub type SEL14 = crate::Reg<u16, _SEL14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL14;
#[doc = "`read()` method returns [sel14::R](sel14::R) reader structure"]
impl crate::Readable for SEL14 {}
#[doc = "`write(|w| ..)` method takes [sel14::W](sel14::W) writer structure"]
impl crate::Writable for SEL14 {}
#[doc = "Crossbar Select Register 14"]
pub mod sel14;
#[doc = "Crossbar Select Register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel15](sel15) module"]
pub type SEL15 = crate::Reg<u16, _SEL15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL15;
#[doc = "`read()` method returns [sel15::R](sel15::R) reader structure"]
impl crate::Readable for SEL15 {}
#[doc = "`write(|w| ..)` method takes [sel15::W](sel15::W) writer structure"]
impl crate::Writable for SEL15 {}
#[doc = "Crossbar Select Register 15"]
pub mod sel15;
#[doc = "Crossbar Select Register 16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel16](sel16) module"]
pub type SEL16 = crate::Reg<u16, _SEL16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL16;
#[doc = "`read()` method returns [sel16::R](sel16::R) reader structure"]
impl crate::Readable for SEL16 {}
#[doc = "`write(|w| ..)` method takes [sel16::W](sel16::W) writer structure"]
impl crate::Writable for SEL16 {}
#[doc = "Crossbar Select Register 16"]
pub mod sel16;
#[doc = "Crossbar Select Register 17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel17](sel17) module"]
pub type SEL17 = crate::Reg<u16, _SEL17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL17;
#[doc = "`read()` method returns [sel17::R](sel17::R) reader structure"]
impl crate::Readable for SEL17 {}
#[doc = "`write(|w| ..)` method takes [sel17::W](sel17::W) writer structure"]
impl crate::Writable for SEL17 {}
#[doc = "Crossbar Select Register 17"]
pub mod sel17;
#[doc = "Crossbar Select Register 18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel18](sel18) module"]
pub type SEL18 = crate::Reg<u16, _SEL18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL18;
#[doc = "`read()` method returns [sel18::R](sel18::R) reader structure"]
impl crate::Readable for SEL18 {}
#[doc = "`write(|w| ..)` method takes [sel18::W](sel18::W) writer structure"]
impl crate::Writable for SEL18 {}
#[doc = "Crossbar Select Register 18"]
pub mod sel18;
#[doc = "Crossbar Select Register 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel19](sel19) module"]
pub type SEL19 = crate::Reg<u16, _SEL19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL19;
#[doc = "`read()` method returns [sel19::R](sel19::R) reader structure"]
impl crate::Readable for SEL19 {}
#[doc = "`write(|w| ..)` method takes [sel19::W](sel19::W) writer structure"]
impl crate::Writable for SEL19 {}
#[doc = "Crossbar Select Register 19"]
pub mod sel19;
#[doc = "Crossbar Select Register 20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel20](sel20) module"]
pub type SEL20 = crate::Reg<u16, _SEL20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL20;
#[doc = "`read()` method returns [sel20::R](sel20::R) reader structure"]
impl crate::Readable for SEL20 {}
#[doc = "`write(|w| ..)` method takes [sel20::W](sel20::W) writer structure"]
impl crate::Writable for SEL20 {}
#[doc = "Crossbar Select Register 20"]
pub mod sel20;
#[doc = "Crossbar Select Register 21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sel21](sel21) module"]
pub type SEL21 = crate::Reg<u16, _SEL21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEL21;
#[doc = "`read()` method returns [sel21::R](sel21::R) reader structure"]
impl crate::Readable for SEL21 {}
#[doc = "`write(|w| ..)` method takes [sel21::W](sel21::W) writer structure"]
impl crate::Writable for SEL21 {}
#[doc = "Crossbar Select Register 21"]
pub mod sel21;
#[doc = "Crossbar Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0](ctrl0) module"]
pub type CTRL0 = crate::Reg<u16, _CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL0;
#[doc = "`read()` method returns [ctrl0::R](ctrl0::R) reader structure"]
impl crate::Readable for CTRL0 {}
#[doc = "`write(|w| ..)` method takes [ctrl0::W](ctrl0::W) writer structure"]
impl crate::Writable for CTRL0 {}
#[doc = "Crossbar Control Register 0"]
pub mod ctrl0;
#[doc = "Crossbar Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](ctrl1) module"]
pub type CTRL1 = crate::Reg<u16, _CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL1;
#[doc = "`read()` method returns [ctrl1::R](ctrl1::R) reader structure"]
impl crate::Readable for CTRL1 {}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](ctrl1::W) writer structure"]
impl crate::Writable for CTRL1 {}
#[doc = "Crossbar Control Register 1"]
pub mod ctrl1;
