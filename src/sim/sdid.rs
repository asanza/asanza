#[doc = "Reader of register SDID"]
pub type R = crate::R<u32, super::SDID>;
#[doc = "Pincount identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINID_A {
    #[doc = "5: 64-pin"]
    _0101 = 5,
    #[doc = "8: 100-pin"]
    _1000 = 8,
    #[doc = "9: 128-pin"]
    _1001 = 9,
    #[doc = "10: 144-pin"]
    _1010 = 10,
}
impl From<PINID_A> for u8 {
    #[inline(always)]
    fn from(variant: PINID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PINID`"]
pub type PINID_R = crate::R<u8, PINID_A>;
impl PINID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PINID_A> {
        use crate::Variant::*;
        match self.bits {
            5 => Val(PINID_A::_0101),
            8 => Val(PINID_A::_1000),
            9 => Val(PINID_A::_1001),
            10 => Val(PINID_A::_1010),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PINID_A::_0101
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == PINID_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == PINID_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == PINID_A::_1010
    }
}
#[doc = "Die ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIEID_A {
    #[doc = "0: First cut"]
    _0000 = 0,
}
impl From<DIEID_A> for u8 {
    #[inline(always)]
    fn from(variant: DIEID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIEID`"]
pub type DIEID_R = crate::R<u8, DIEID_A>;
impl DIEID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIEID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIEID_A::_0000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DIEID_A::_0000
    }
}
#[doc = "Revision ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REVID_A {
    #[doc = "0: First cut"]
    _0000 = 0,
}
impl From<REVID_A> for u8 {
    #[inline(always)]
    fn from(variant: REVID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REVID`"]
pub type REVID_R = crate::R<u8, REVID_A>;
impl REVID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REVID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REVID_A::_0000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == REVID_A::_0000
    }
}
#[doc = "SRAM Size\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRAMSIZE_A {
    #[doc = "6: 32 KB SRAM"]
    _0110 = 6,
}
impl From<SRAMSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRAMSIZE`"]
pub type SRAMSIZE_R = crate::R<u8, SRAMSIZE_A>;
impl SRAMSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRAMSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            6 => Val(SRAMSIZE_A::_0110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == SRAMSIZE_A::_0110
    }
}
#[doc = "Attribute ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ATTR_A {
    #[doc = "0: M0+ core"]
    _0000 = 0,
}
impl From<ATTR_A> for u8 {
    #[inline(always)]
    fn from(variant: ATTR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ATTR`"]
pub type ATTR_R = crate::R<u8, ATTR_A>;
impl ATTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ATTR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ATTR_A::_0000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == ATTR_A::_0000
    }
}
#[doc = "Series ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SERIESID_A {
    #[doc = "3: Metering Series"]
    _0011 = 3,
}
impl From<SERIESID_A> for u8 {
    #[inline(always)]
    fn from(variant: SERIESID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SERIESID`"]
pub type SERIESID_R = crate::R<u8, SERIESID_A>;
impl SERIESID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SERIESID_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(SERIESID_A::_0011),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == SERIESID_A::_0011
    }
}
#[doc = "Sub-Family ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SUBFAMID_A {
    #[doc = "0: Device derivatives with NO AFE enabled"]
    _0000 = 0,
    #[doc = "1: Device derivatives with 1 AFE enabled"]
    _0001 = 1,
    #[doc = "2: Device derivatives with 2 AFE enabled"]
    _0010 = 2,
    #[doc = "3: Device derivatives with 3 AFE enabled"]
    _0011 = 3,
    #[doc = "4: Device derivatives with 4 AFE enabled"]
    _0100 = 4,
}
impl From<SUBFAMID_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBFAMID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SUBFAMID`"]
pub type SUBFAMID_R = crate::R<u8, SUBFAMID_A>;
impl SUBFAMID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUBFAMID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUBFAMID_A::_0000),
            1 => Val(SUBFAMID_A::_0001),
            2 => Val(SUBFAMID_A::_0010),
            3 => Val(SUBFAMID_A::_0011),
            4 => Val(SUBFAMID_A::_0100),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == SUBFAMID_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == SUBFAMID_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == SUBFAMID_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == SUBFAMID_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == SUBFAMID_A::_0100
    }
}
#[doc = "Metering family ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FAMID_A {
    #[doc = "1: Device derivatives without LCD"]
    _0001 = 1,
    #[doc = "3: Device derivatives with LCD"]
    _0011 = 3,
}
impl From<FAMID_A> for u8 {
    #[inline(always)]
    fn from(variant: FAMID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FAMID`"]
pub type FAMID_R = crate::R<u8, FAMID_A>;
impl FAMID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FAMID_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(FAMID_A::_0001),
            3 => Val(FAMID_A::_0011),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == FAMID_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == FAMID_A::_0011
    }
}
impl R {
    #[doc = "Bits 0:3 - Pincount identification"]
    #[inline(always)]
    pub fn pinid(&self) -> PINID_R {
        PINID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Die ID"]
    #[inline(always)]
    pub fn dieid(&self) -> DIEID_R {
        DIEID_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Revision ID"]
    #[inline(always)]
    pub fn revid(&self) -> REVID_R {
        REVID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SRAM Size"]
    #[inline(always)]
    pub fn sramsize(&self) -> SRAMSIZE_R {
        SRAMSIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Attribute ID"]
    #[inline(always)]
    pub fn attr(&self) -> ATTR_R {
        ATTR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Series ID"]
    #[inline(always)]
    pub fn seriesid(&self) -> SERIESID_R {
        SERIESID_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Sub-Family ID"]
    #[inline(always)]
    pub fn subfamid(&self) -> SUBFAMID_R {
        SUBFAMID_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Metering family ID"]
    #[inline(always)]
    pub fn famid(&self) -> FAMID_R {
        FAMID_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
