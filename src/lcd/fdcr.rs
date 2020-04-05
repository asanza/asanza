#[doc = "Reader of register FDCR"]
pub type R = crate::R<u32, super::FDCR>;
#[doc = "Writer for register FDCR"]
pub type W = crate::W<u32, super::FDCR>;
#[doc = "Register FDCR `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Fault Detect Pin ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FDPINID_A {
    #[doc = "0: Fault detection for LCD_P0 pin."]
    _0 = 0,
    #[doc = "1: Fault detection for LCD_P1 pin."]
    _1 = 1,
    #[doc = "63: Fault detection for LCD_P63 pin."]
    _111111 = 63,
}
impl From<FDPINID_A> for u8 {
    #[inline(always)]
    fn from(variant: FDPINID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FDPINID`"]
pub type FDPINID_R = crate::R<u8, FDPINID_A>;
impl FDPINID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FDPINID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FDPINID_A::_0),
            1 => Val(FDPINID_A::_1),
            63 => Val(FDPINID_A::_111111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDPINID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDPINID_A::_1
    }
    #[doc = "Checks if the value of the field is `_111111`"]
    #[inline(always)]
    pub fn is_111111(&self) -> bool {
        *self == FDPINID_A::_111111
    }
}
#[doc = "Write proxy for field `FDPINID`"]
pub struct FDPINID_W<'a> {
    w: &'a mut W,
}
impl<'a> FDPINID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDPINID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Fault detection for LCD_P0 pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDPINID_A::_0)
    }
    #[doc = "Fault detection for LCD_P1 pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDPINID_A::_1)
    }
    #[doc = "Fault detection for LCD_P63 pin."]
    #[inline(always)]
    pub fn _111111(self) -> &'a mut W {
        self.variant(FDPINID_A::_111111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Fault Detect Back Plane Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDBPEN_A {
    #[doc = "0: Type of the selected pin under fault detect test is front plane."]
    _0 = 0,
    #[doc = "1: Type of the selected pin under fault detect test is back plane."]
    _1 = 1,
}
impl From<FDBPEN_A> for bool {
    #[inline(always)]
    fn from(variant: FDBPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FDBPEN`"]
pub type FDBPEN_R = crate::R<bool, FDBPEN_A>;
impl FDBPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDBPEN_A {
        match self.bits {
            false => FDBPEN_A::_0,
            true => FDBPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDBPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDBPEN_A::_1
    }
}
#[doc = "Write proxy for field `FDBPEN`"]
pub struct FDBPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDBPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDBPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Type of the selected pin under fault detect test is front plane."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDBPEN_A::_0)
    }
    #[doc = "Type of the selected pin under fault detect test is back plane."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDBPEN_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Fault Detect Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDEN_A {
    #[doc = "0: Disable fault detection."]
    _0 = 0,
    #[doc = "1: Enable fault detection."]
    _1 = 1,
}
impl From<FDEN_A> for bool {
    #[inline(always)]
    fn from(variant: FDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FDEN`"]
pub type FDEN_R = crate::R<bool, FDEN_A>;
impl FDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDEN_A {
        match self.bits {
            false => FDEN_A::_0,
            true => FDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDEN_A::_1
    }
}
#[doc = "Write proxy for field `FDEN`"]
pub struct FDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable fault detection."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDEN_A::_0)
    }
    #[doc = "Enable fault detection."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDEN_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Fault Detect Sample Window Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FDSWW_A {
    #[doc = "0: Sample window width is 4 sample clock cycles."]
    _0 = 0,
    #[doc = "1: Sample window width is 8 sample clock cycles."]
    _1 = 1,
    #[doc = "2: Sample window width is 16 sample clock cycles."]
    _10 = 2,
    #[doc = "3: Sample window width is 32 sample clock cycles."]
    _11 = 3,
    #[doc = "4: Sample window width is 64 sample clock cycles."]
    _100 = 4,
    #[doc = "5: Sample window width is 128 sample clock cycles."]
    _101 = 5,
    #[doc = "6: Sample window width is 256 sample clock cycles."]
    _110 = 6,
    #[doc = "7: Sample window width is 512 sample clock cycles."]
    _111 = 7,
}
impl From<FDSWW_A> for u8 {
    #[inline(always)]
    fn from(variant: FDSWW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FDSWW`"]
pub type FDSWW_R = crate::R<u8, FDSWW_A>;
impl FDSWW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDSWW_A {
        match self.bits {
            0 => FDSWW_A::_0,
            1 => FDSWW_A::_1,
            2 => FDSWW_A::_10,
            3 => FDSWW_A::_11,
            4 => FDSWW_A::_100,
            5 => FDSWW_A::_101,
            6 => FDSWW_A::_110,
            7 => FDSWW_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDSWW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDSWW_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FDSWW_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FDSWW_A::_11
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FDSWW_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FDSWW_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FDSWW_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == FDSWW_A::_111
    }
}
#[doc = "Write proxy for field `FDSWW`"]
pub struct FDSWW_W<'a> {
    w: &'a mut W,
}
impl<'a> FDSWW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDSWW_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Sample window width is 4 sample clock cycles."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDSWW_A::_0)
    }
    #[doc = "Sample window width is 8 sample clock cycles."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDSWW_A::_1)
    }
    #[doc = "Sample window width is 16 sample clock cycles."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FDSWW_A::_10)
    }
    #[doc = "Sample window width is 32 sample clock cycles."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FDSWW_A::_11)
    }
    #[doc = "Sample window width is 64 sample clock cycles."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(FDSWW_A::_100)
    }
    #[doc = "Sample window width is 128 sample clock cycles."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(FDSWW_A::_101)
    }
    #[doc = "Sample window width is 256 sample clock cycles."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(FDSWW_A::_110)
    }
    #[doc = "Sample window width is 512 sample clock cycles."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(FDSWW_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Fault Detect Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FDPRS_A {
    #[doc = "0: 1/1 bus clock."]
    _0 = 0,
    #[doc = "1: 1/2 bus clock."]
    _1 = 1,
    #[doc = "2: 1/4 bus clock."]
    _10 = 2,
    #[doc = "3: 1/8 bus clock."]
    _11 = 3,
    #[doc = "4: 1/16 bus clock."]
    _100 = 4,
    #[doc = "5: 1/32 bus clock."]
    _101 = 5,
    #[doc = "6: 1/64 bus clock."]
    _110 = 6,
    #[doc = "7: 1/128 bus clock."]
    _111 = 7,
}
impl From<FDPRS_A> for u8 {
    #[inline(always)]
    fn from(variant: FDPRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FDPRS`"]
pub type FDPRS_R = crate::R<u8, FDPRS_A>;
impl FDPRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDPRS_A {
        match self.bits {
            0 => FDPRS_A::_0,
            1 => FDPRS_A::_1,
            2 => FDPRS_A::_10,
            3 => FDPRS_A::_11,
            4 => FDPRS_A::_100,
            5 => FDPRS_A::_101,
            6 => FDPRS_A::_110,
            7 => FDPRS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDPRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDPRS_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FDPRS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FDPRS_A::_11
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FDPRS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FDPRS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FDPRS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == FDPRS_A::_111
    }
}
#[doc = "Write proxy for field `FDPRS`"]
pub struct FDPRS_W<'a> {
    w: &'a mut W,
}
impl<'a> FDPRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDPRS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1/1 bus clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDPRS_A::_0)
    }
    #[doc = "1/2 bus clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDPRS_A::_1)
    }
    #[doc = "1/4 bus clock."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FDPRS_A::_10)
    }
    #[doc = "1/8 bus clock."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FDPRS_A::_11)
    }
    #[doc = "1/16 bus clock."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(FDPRS_A::_100)
    }
    #[doc = "1/32 bus clock."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(FDPRS_A::_101)
    }
    #[doc = "1/64 bus clock."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(FDPRS_A::_110)
    }
    #[doc = "1/128 bus clock."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(FDPRS_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Fault Detect Pin ID"]
    #[inline(always)]
    pub fn fdpinid(&self) -> FDPINID_R {
        FDPINID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Fault Detect Back Plane Enable"]
    #[inline(always)]
    pub fn fdbpen(&self) -> FDBPEN_R {
        FDBPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fault Detect Enable"]
    #[inline(always)]
    pub fn fden(&self) -> FDEN_R {
        FDEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - Fault Detect Sample Window Width"]
    #[inline(always)]
    pub fn fdsww(&self) -> FDSWW_R {
        FDSWW_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Fault Detect Clock Prescaler"]
    #[inline(always)]
    pub fn fdprs(&self) -> FDPRS_R {
        FDPRS_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Fault Detect Pin ID"]
    #[inline(always)]
    pub fn fdpinid(&mut self) -> FDPINID_W {
        FDPINID_W { w: self }
    }
    #[doc = "Bit 6 - Fault Detect Back Plane Enable"]
    #[inline(always)]
    pub fn fdbpen(&mut self) -> FDBPEN_W {
        FDBPEN_W { w: self }
    }
    #[doc = "Bit 7 - Fault Detect Enable"]
    #[inline(always)]
    pub fn fden(&mut self) -> FDEN_W {
        FDEN_W { w: self }
    }
    #[doc = "Bits 9:11 - Fault Detect Sample Window Width"]
    #[inline(always)]
    pub fn fdsww(&mut self) -> FDSWW_W {
        FDSWW_W { w: self }
    }
    #[doc = "Bits 12:14 - Fault Detect Clock Prescaler"]
    #[inline(always)]
    pub fn fdprs(&mut self) -> FDPRS_W {
        FDPRS_W { w: self }
    }
}
