#[doc = "Reader of register PACRE"]
pub type R = crate::R<u32, super::PACRE>;
#[doc = "Writer for register PACRE"]
pub type W = crate::W<u32, super::PACRE>;
#[doc = "Register PACRE `reset()`'s with value 0"]
impl crate::ResetValue for super::PACRE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AC7`"]
pub type AC7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AC7`"]
pub struct AC7_W<'a> {
    w: &'a mut W,
}
impl<'a> AC7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Read Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO7_A {
    #[doc = "0: Writes to corresponding AC field are allowed."]
    _0 = 0,
    #[doc = "1: Writes to corresponding AC field are ignored."]
    _1 = 1,
}
impl From<RO7_A> for bool {
    #[inline(always)]
    fn from(variant: RO7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RO7`"]
pub type RO7_R = crate::R<bool, RO7_A>;
impl RO7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RO7_A {
        match self.bits {
            false => RO7_A::_0,
            true => RO7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RO7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RO7_A::_1
    }
}
#[doc = "Write proxy for field `RO7`"]
pub struct RO7_W<'a> {
    w: &'a mut W,
}
impl<'a> RO7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RO7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes to corresponding AC field are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RO7_A::_0)
    }
    #[doc = "Writes to corresponding AC field are ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RO7_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `AC6`"]
pub type AC6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AC6`"]
pub struct AC6_W<'a> {
    w: &'a mut W,
}
impl<'a> AC6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Read Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO6_A {
    #[doc = "0: Writes to corresponding AC field are allowed."]
    _0 = 0,
    #[doc = "1: Writes to corresponding AC field are ignored."]
    _1 = 1,
}
impl From<RO6_A> for bool {
    #[inline(always)]
    fn from(variant: RO6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RO6`"]
pub type RO6_R = crate::R<bool, RO6_A>;
impl RO6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RO6_A {
        match self.bits {
            false => RO6_A::_0,
            true => RO6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RO6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RO6_A::_1
    }
}
#[doc = "Write proxy for field `RO6`"]
pub struct RO6_W<'a> {
    w: &'a mut W,
}
impl<'a> RO6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RO6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes to corresponding AC field are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RO6_A::_0)
    }
    #[doc = "Writes to corresponding AC field are ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RO6_A::_1)
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
#[doc = "Reader of field `AC5`"]
pub type AC5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AC5`"]
pub struct AC5_W<'a> {
    w: &'a mut W,
}
impl<'a> AC5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Read Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO5_A {
    #[doc = "0: Writes to corresponding AC field are allowed."]
    _0 = 0,
    #[doc = "1: Writes to corresponding AC field are ignored."]
    _1 = 1,
}
impl From<RO5_A> for bool {
    #[inline(always)]
    fn from(variant: RO5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RO5`"]
pub type RO5_R = crate::R<bool, RO5_A>;
impl RO5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RO5_A {
        match self.bits {
            false => RO5_A::_0,
            true => RO5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RO5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RO5_A::_1
    }
}
#[doc = "Write proxy for field `RO5`"]
pub struct RO5_W<'a> {
    w: &'a mut W,
}
impl<'a> RO5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RO5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes to corresponding AC field are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RO5_A::_0)
    }
    #[doc = "Writes to corresponding AC field are ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RO5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `AC4`"]
pub type AC4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AC4`"]
pub struct AC4_W<'a> {
    w: &'a mut W,
}
impl<'a> AC4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Read Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO4_A {
    #[doc = "0: Writes to corresponding AC field are allowed."]
    _0 = 0,
    #[doc = "1: Writes to corresponding AC field are ignored."]
    _1 = 1,
}
impl From<RO4_A> for bool {
    #[inline(always)]
    fn from(variant: RO4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RO4`"]
pub type RO4_R = crate::R<bool, RO4_A>;
impl RO4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RO4_A {
        match self.bits {
            false => RO4_A::_0,
            true => RO4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RO4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RO4_A::_1
    }
}
#[doc = "Write proxy for field `RO4`"]
pub struct RO4_W<'a> {
    w: &'a mut W,
}
impl<'a> RO4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RO4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes to corresponding AC field are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RO4_A::_0)
    }
    #[doc = "Writes to corresponding AC field are ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RO4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `AC3`"]
pub type AC3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AC3`"]
pub struct AC3_W<'a> {
    w: &'a mut W,
}
impl<'a> AC3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Read Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO3_A {
    #[doc = "0: Writes to corresponding AC field are allowed."]
    _0 = 0,
    #[doc = "1: Writes to corresponding AC field are ignored."]
    _1 = 1,
}
impl From<RO3_A> for bool {
    #[inline(always)]
    fn from(variant: RO3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RO3`"]
pub type RO3_R = crate::R<bool, RO3_A>;
impl RO3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RO3_A {
        match self.bits {
            false => RO3_A::_0,
            true => RO3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RO3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RO3_A::_1
    }
}
#[doc = "Write proxy for field `RO3`"]
pub struct RO3_W<'a> {
    w: &'a mut W,
}
impl<'a> RO3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RO3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes to corresponding AC field are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RO3_A::_0)
    }
    #[doc = "Writes to corresponding AC field are ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RO3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `AC2`"]
pub type AC2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AC2`"]
pub struct AC2_W<'a> {
    w: &'a mut W,
}
impl<'a> AC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Read Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO2_A {
    #[doc = "0: Writes to corresponding AC field are allowed."]
    _0 = 0,
    #[doc = "1: Writes to corresponding AC field are ignored."]
    _1 = 1,
}
impl From<RO2_A> for bool {
    #[inline(always)]
    fn from(variant: RO2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RO2`"]
pub type RO2_R = crate::R<bool, RO2_A>;
impl RO2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RO2_A {
        match self.bits {
            false => RO2_A::_0,
            true => RO2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RO2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RO2_A::_1
    }
}
#[doc = "Write proxy for field `RO2`"]
pub struct RO2_W<'a> {
    w: &'a mut W,
}
impl<'a> RO2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RO2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes to corresponding AC field are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RO2_A::_0)
    }
    #[doc = "Writes to corresponding AC field are ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RO2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `AC1`"]
pub type AC1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AC1`"]
pub struct AC1_W<'a> {
    w: &'a mut W,
}
impl<'a> AC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Read Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO1_A {
    #[doc = "0: Writes to corresponding AC field are allowed."]
    _0 = 0,
    #[doc = "1: Writes to corresponding AC field are ignored."]
    _1 = 1,
}
impl From<RO1_A> for bool {
    #[inline(always)]
    fn from(variant: RO1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RO1`"]
pub type RO1_R = crate::R<bool, RO1_A>;
impl RO1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RO1_A {
        match self.bits {
            false => RO1_A::_0,
            true => RO1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RO1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RO1_A::_1
    }
}
#[doc = "Write proxy for field `RO1`"]
pub struct RO1_W<'a> {
    w: &'a mut W,
}
impl<'a> RO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RO1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes to corresponding AC field are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RO1_A::_0)
    }
    #[doc = "Writes to corresponding AC field are ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RO1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `AC0`"]
pub type AC0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AC0`"]
pub struct AC0_W<'a> {
    w: &'a mut W,
}
impl<'a> AC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Read Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO0_A {
    #[doc = "0: Writes to corresponding AC field are allowed."]
    _0 = 0,
    #[doc = "1: Writes to corresponding AC field are ignored."]
    _1 = 1,
}
impl From<RO0_A> for bool {
    #[inline(always)]
    fn from(variant: RO0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RO0`"]
pub type RO0_R = crate::R<bool, RO0_A>;
impl RO0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RO0_A {
        match self.bits {
            false => RO0_A::_0,
            true => RO0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RO0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RO0_A::_1
    }
}
#[doc = "Write proxy for field `RO0`"]
pub struct RO0_W<'a> {
    w: &'a mut W,
}
impl<'a> RO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RO0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes to corresponding AC field are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RO0_A::_0)
    }
    #[doc = "Writes to corresponding AC field are ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RO0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Attribute Check"]
    #[inline(always)]
    pub fn ac7(&self) -> AC7_R {
        AC7_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Read Only"]
    #[inline(always)]
    pub fn ro7(&self) -> RO7_R {
        RO7_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Attribute Check"]
    #[inline(always)]
    pub fn ac6(&self) -> AC6_R {
        AC6_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Read Only"]
    #[inline(always)]
    pub fn ro6(&self) -> RO6_R {
        RO6_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Attribute Check"]
    #[inline(always)]
    pub fn ac5(&self) -> AC5_R {
        AC5_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Read Only"]
    #[inline(always)]
    pub fn ro5(&self) -> RO5_R {
        RO5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Attribute Check"]
    #[inline(always)]
    pub fn ac4(&self) -> AC4_R {
        AC4_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Read Only"]
    #[inline(always)]
    pub fn ro4(&self) -> RO4_R {
        RO4_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Attribute Check"]
    #[inline(always)]
    pub fn ac3(&self) -> AC3_R {
        AC3_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - Read Only"]
    #[inline(always)]
    pub fn ro3(&self) -> RO3_R {
        RO3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - Attribute Check"]
    #[inline(always)]
    pub fn ac2(&self) -> AC2_R {
        AC2_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - Read Only"]
    #[inline(always)]
    pub fn ro2(&self) -> RO2_R {
        RO2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Attribute Check"]
    #[inline(always)]
    pub fn ac1(&self) -> AC1_R {
        AC1_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - Read Only"]
    #[inline(always)]
    pub fn ro1(&self) -> RO1_R {
        RO1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - Attribute Check"]
    #[inline(always)]
    pub fn ac0(&self) -> AC0_R {
        AC0_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Read Only"]
    #[inline(always)]
    pub fn ro0(&self) -> RO0_R {
        RO0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Attribute Check"]
    #[inline(always)]
    pub fn ac7(&mut self) -> AC7_W {
        AC7_W { w: self }
    }
    #[doc = "Bit 3 - Read Only"]
    #[inline(always)]
    pub fn ro7(&mut self) -> RO7_W {
        RO7_W { w: self }
    }
    #[doc = "Bits 4:6 - Attribute Check"]
    #[inline(always)]
    pub fn ac6(&mut self) -> AC6_W {
        AC6_W { w: self }
    }
    #[doc = "Bit 7 - Read Only"]
    #[inline(always)]
    pub fn ro6(&mut self) -> RO6_W {
        RO6_W { w: self }
    }
    #[doc = "Bits 8:10 - Attribute Check"]
    #[inline(always)]
    pub fn ac5(&mut self) -> AC5_W {
        AC5_W { w: self }
    }
    #[doc = "Bit 11 - Read Only"]
    #[inline(always)]
    pub fn ro5(&mut self) -> RO5_W {
        RO5_W { w: self }
    }
    #[doc = "Bits 12:14 - Attribute Check"]
    #[inline(always)]
    pub fn ac4(&mut self) -> AC4_W {
        AC4_W { w: self }
    }
    #[doc = "Bit 15 - Read Only"]
    #[inline(always)]
    pub fn ro4(&mut self) -> RO4_W {
        RO4_W { w: self }
    }
    #[doc = "Bits 16:18 - Attribute Check"]
    #[inline(always)]
    pub fn ac3(&mut self) -> AC3_W {
        AC3_W { w: self }
    }
    #[doc = "Bit 19 - Read Only"]
    #[inline(always)]
    pub fn ro3(&mut self) -> RO3_W {
        RO3_W { w: self }
    }
    #[doc = "Bits 20:22 - Attribute Check"]
    #[inline(always)]
    pub fn ac2(&mut self) -> AC2_W {
        AC2_W { w: self }
    }
    #[doc = "Bit 23 - Read Only"]
    #[inline(always)]
    pub fn ro2(&mut self) -> RO2_W {
        RO2_W { w: self }
    }
    #[doc = "Bits 24:26 - Attribute Check"]
    #[inline(always)]
    pub fn ac1(&mut self) -> AC1_W {
        AC1_W { w: self }
    }
    #[doc = "Bit 27 - Read Only"]
    #[inline(always)]
    pub fn ro1(&mut self) -> RO1_W {
        RO1_W { w: self }
    }
    #[doc = "Bits 28:30 - Attribute Check"]
    #[inline(always)]
    pub fn ac0(&mut self) -> AC0_W {
        AC0_W { w: self }
    }
    #[doc = "Bit 31 - Read Only"]
    #[inline(always)]
    pub fn ro0(&mut self) -> RO0_W {
        RO0_W { w: self }
    }
}
