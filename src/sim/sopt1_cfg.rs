#[doc = "Reader of register SOPT1_CFG"]
pub type R = crate::R<u32, super::SOPT1_CFG>;
#[doc = "Writer for register SOPT1_CFG"]
pub type W = crate::W<u32, super::SOPT1_CFG>;
#[doc = "Register SOPT1_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SOPT1_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LP Timer Channel0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPTMR0SEL_A {
    #[doc = "0: CMP\\[0\\]
output"]
    _00 = 0,
    #[doc = "1: CMP\\[1\\]
output"]
    _01 = 1,
    #[doc = "2: CMP\\[2\\]
output"]
    _10 = 2,
}
impl From<LPTMR0SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTMR0SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPTMR0SEL`"]
pub type LPTMR0SEL_R = crate::R<u8, LPTMR0SEL_A>;
impl LPTMR0SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPTMR0SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPTMR0SEL_A::_00),
            1 => Val(LPTMR0SEL_A::_01),
            2 => Val(LPTMR0SEL_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPTMR0SEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPTMR0SEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LPTMR0SEL_A::_10
    }
}
#[doc = "Write proxy for field `LPTMR0SEL`"]
pub struct LPTMR0SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTMR0SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTMR0SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CMP\\[0\\]
output"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPTMR0SEL_A::_00)
    }
    #[doc = "CMP\\[1\\]
output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPTMR0SEL_A::_01)
    }
    #[doc = "CMP\\[2\\]
output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPTMR0SEL_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "LP Timer Channel1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPTMR1SEL_A {
    #[doc = "0: Pad PTE4"]
    _00 = 0,
    #[doc = "1: Pad PTF4"]
    _01 = 1,
    #[doc = "2: Pad PTG1"]
    _10 = 2,
}
impl From<LPTMR1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTMR1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPTMR1SEL`"]
pub type LPTMR1SEL_R = crate::R<u8, LPTMR1SEL_A>;
impl LPTMR1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPTMR1SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPTMR1SEL_A::_00),
            1 => Val(LPTMR1SEL_A::_01),
            2 => Val(LPTMR1SEL_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPTMR1SEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPTMR1SEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LPTMR1SEL_A::_10
    }
}
#[doc = "Write proxy for field `LPTMR1SEL`"]
pub struct LPTMR1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTMR1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTMR1SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Pad PTE4"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPTMR1SEL_A::_00)
    }
    #[doc = "Pad PTF4"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPTMR1SEL_A::_01)
    }
    #[doc = "Pad PTG1"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPTMR1SEL_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "LP Timer Channel2 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPTMR2SEL_A {
    #[doc = "0: Pad PTD6"]
    _00 = 0,
    #[doc = "1: Pad PTF3"]
    _01 = 1,
    #[doc = "2: Pad PTG5"]
    _10 = 2,
}
impl From<LPTMR2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTMR2SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPTMR2SEL`"]
pub type LPTMR2SEL_R = crate::R<u8, LPTMR2SEL_A>;
impl LPTMR2SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPTMR2SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPTMR2SEL_A::_00),
            1 => Val(LPTMR2SEL_A::_01),
            2 => Val(LPTMR2SEL_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPTMR2SEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPTMR2SEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LPTMR2SEL_A::_10
    }
}
#[doc = "Write proxy for field `LPTMR2SEL`"]
pub struct LPTMR2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTMR2SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTMR2SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Pad PTD6"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPTMR2SEL_A::_00)
    }
    #[doc = "Pad PTF3"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPTMR2SEL_A::_01)
    }
    #[doc = "Pad PTG5"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPTMR2SEL_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "LP Timer Channel3 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPTMR3SEL_A {
    #[doc = "0: Pad PTD5"]
    _00 = 0,
    #[doc = "1: Pad PTG0"]
    _01 = 1,
    #[doc = "2: Pad PTG6"]
    _10 = 2,
}
impl From<LPTMR3SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTMR3SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPTMR3SEL`"]
pub type LPTMR3SEL_R = crate::R<u8, LPTMR3SEL_A>;
impl LPTMR3SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPTMR3SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPTMR3SEL_A::_00),
            1 => Val(LPTMR3SEL_A::_01),
            2 => Val(LPTMR3SEL_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPTMR3SEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPTMR3SEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LPTMR3SEL_A::_10
    }
}
#[doc = "Write proxy for field `LPTMR3SEL`"]
pub struct LPTMR3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTMR3SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTMR3SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Pad PTD5"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPTMR3SEL_A::_00)
    }
    #[doc = "Pad PTG0"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPTMR3SEL_A::_01)
    }
    #[doc = "Pad PTG6"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPTMR3SEL_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Disable source bias of System SRAM arrays during VLPR and VLPW modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMSBDIS_A {
    #[doc = "0: Source bias of System SRAM enabled during VLPR and VLPW modes."]
    _0 = 0,
    #[doc = "1: Source bias of System SRAM disabled during VLPR and VLPW modes."]
    _1 = 1,
}
impl From<RAMSBDIS_A> for bool {
    #[inline(always)]
    fn from(variant: RAMSBDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RAMSBDIS`"]
pub type RAMSBDIS_R = crate::R<bool, RAMSBDIS_A>;
impl RAMSBDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMSBDIS_A {
        match self.bits {
            false => RAMSBDIS_A::_0,
            true => RAMSBDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RAMSBDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RAMSBDIS_A::_1
    }
}
#[doc = "Write proxy for field `RAMSBDIS`"]
pub struct RAMSBDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMSBDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMSBDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Source bias of System SRAM enabled during VLPR and VLPW modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RAMSBDIS_A::_0)
    }
    #[doc = "Source bias of System SRAM disabled during VLPR and VLPW modes."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RAMSBDIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "RAM Bitline Precharge Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMBPEN_A {
    #[doc = "0: Bitline precharge of system SRAM disabled during VLPR and VLPW modes."]
    _0 = 0,
    #[doc = "1: Bitline precharge of system SRAM enabled during VLPR and VLPW modes."]
    _1 = 1,
}
impl From<RAMBPEN_A> for bool {
    #[inline(always)]
    fn from(variant: RAMBPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RAMBPEN`"]
pub type RAMBPEN_R = crate::R<bool, RAMBPEN_A>;
impl RAMBPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMBPEN_A {
        match self.bits {
            false => RAMBPEN_A::_0,
            true => RAMBPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RAMBPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RAMBPEN_A::_1
    }
}
#[doc = "Write proxy for field `RAMBPEN`"]
pub struct RAMBPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMBPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMBPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bitline precharge of system SRAM disabled during VLPR and VLPW modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RAMBPEN_A::_0)
    }
    #[doc = "Bitline precharge of system SRAM enabled during VLPR and VLPW modes."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RAMBPEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - LP Timer Channel0 Select"]
    #[inline(always)]
    pub fn lptmr0sel(&self) -> LPTMR0SEL_R {
        LPTMR0SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - LP Timer Channel1 Select"]
    #[inline(always)]
    pub fn lptmr1sel(&self) -> LPTMR1SEL_R {
        LPTMR1SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - LP Timer Channel2 Select"]
    #[inline(always)]
    pub fn lptmr2sel(&self) -> LPTMR2SEL_R {
        LPTMR2SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - LP Timer Channel3 Select"]
    #[inline(always)]
    pub fn lptmr3sel(&self) -> LPTMR3SEL_R {
        LPTMR3SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Disable source bias of System SRAM arrays during VLPR and VLPW modes."]
    #[inline(always)]
    pub fn ramsbdis(&self) -> RAMSBDIS_R {
        RAMSBDIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RAM Bitline Precharge Enable"]
    #[inline(always)]
    pub fn rambpen(&self) -> RAMBPEN_R {
        RAMBPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - LP Timer Channel0 Select"]
    #[inline(always)]
    pub fn lptmr0sel(&mut self) -> LPTMR0SEL_W {
        LPTMR0SEL_W { w: self }
    }
    #[doc = "Bits 2:3 - LP Timer Channel1 Select"]
    #[inline(always)]
    pub fn lptmr1sel(&mut self) -> LPTMR1SEL_W {
        LPTMR1SEL_W { w: self }
    }
    #[doc = "Bits 4:5 - LP Timer Channel2 Select"]
    #[inline(always)]
    pub fn lptmr2sel(&mut self) -> LPTMR2SEL_W {
        LPTMR2SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - LP Timer Channel3 Select"]
    #[inline(always)]
    pub fn lptmr3sel(&mut self) -> LPTMR3SEL_W {
        LPTMR3SEL_W { w: self }
    }
    #[doc = "Bit 8 - Disable source bias of System SRAM arrays during VLPR and VLPW modes."]
    #[inline(always)]
    pub fn ramsbdis(&mut self) -> RAMSBDIS_W {
        RAMSBDIS_W { w: self }
    }
    #[doc = "Bit 9 - RAM Bitline Precharge Enable"]
    #[inline(always)]
    pub fn rambpen(&mut self) -> RAMBPEN_W {
        RAMBPEN_W { w: self }
    }
}
