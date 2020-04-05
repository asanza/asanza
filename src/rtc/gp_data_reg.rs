#[doc = "Reader of register GP_DATA_REG"]
pub type R = crate::R<u16, super::GP_DATA_REG>;
#[doc = "Writer for register GP_DATA_REG"]
pub type W = crate::W<u16, super::GP_DATA_REG>;
#[doc = "Register GP_DATA_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::GP_DATA_REG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "32 kHz RTC OSC Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG0_A {
    #[doc = "0: Oscillator is disabled."]
    _0 = 0,
    #[doc = "1: Oscillator is enabled."]
    _1 = 1,
}
impl From<CFG0_A> for bool {
    #[inline(always)]
    fn from(variant: CFG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CFG0`"]
pub type CFG0_R = crate::R<bool, CFG0_A>;
impl CFG0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG0_A {
        match self.bits {
            false => CFG0_A::_0,
            true => CFG0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFG0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFG0_A::_1
    }
}
#[doc = "Write proxy for field `CFG0`"]
pub struct CFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Oscillator is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFG0_A::_0)
    }
    #[doc = "Oscillator is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFG0_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Switched capacitor 2 pF enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG1_A {
    #[doc = "0: Disables capacitor"]
    _0 = 0,
    #[doc = "1: Enables capacitor"]
    _1 = 1,
}
impl From<CFG1_A> for bool {
    #[inline(always)]
    fn from(variant: CFG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CFG1`"]
pub type CFG1_R = crate::R<bool, CFG1_A>;
impl CFG1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG1_A {
        match self.bits {
            false => CFG1_A::_0,
            true => CFG1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFG1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFG1_A::_1
    }
}
#[doc = "Write proxy for field `CFG1`"]
pub struct CFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables capacitor"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFG1_A::_0)
    }
    #[doc = "Enables capacitor"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFG1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Switched capacitor 4 pF enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG2_A {
    #[doc = "0: Disables capacitor"]
    _0 = 0,
    #[doc = "1: Enables capacitor"]
    _1 = 1,
}
impl From<CFG2_A> for bool {
    #[inline(always)]
    fn from(variant: CFG2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CFG2`"]
pub type CFG2_R = crate::R<bool, CFG2_A>;
impl CFG2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG2_A {
        match self.bits {
            false => CFG2_A::_0,
            true => CFG2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFG2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFG2_A::_1
    }
}
#[doc = "Write proxy for field `CFG2`"]
pub struct CFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables capacitor"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFG2_A::_0)
    }
    #[doc = "Enables capacitor"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFG2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Switched capacitor 8 pF enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG3_A {
    #[doc = "0: Disables capacitor"]
    _0 = 0,
    #[doc = "1: Enables capacitor"]
    _1 = 1,
}
impl From<CFG3_A> for bool {
    #[inline(always)]
    fn from(variant: CFG3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CFG3`"]
pub type CFG3_R = crate::R<bool, CFG3_A>;
impl CFG3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG3_A {
        match self.bits {
            false => CFG3_A::_0,
            true => CFG3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFG3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFG3_A::_1
    }
}
#[doc = "Write proxy for field `CFG3`"]
pub struct CFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables capacitor"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFG3_A::_0)
    }
    #[doc = "Enables capacitor"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFG3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Switched capacitor 16 pF enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG4_A {
    #[doc = "0: Disables capacitor"]
    _0 = 0,
    #[doc = "1: Enables capacitor"]
    _1 = 1,
}
impl From<CFG4_A> for bool {
    #[inline(always)]
    fn from(variant: CFG4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CFG4`"]
pub type CFG4_R = crate::R<bool, CFG4_A>;
impl CFG4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG4_A {
        match self.bits {
            false => CFG4_A::_0,
            true => CFG4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFG4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFG4_A::_1
    }
}
#[doc = "Write proxy for field `CFG4`"]
pub struct CFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables capacitor"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFG4_A::_0)
    }
    #[doc = "Enables capacitor"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFG4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CFG5`"]
pub type CFG5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFG5`"]
pub struct CFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CFG6`"]
pub type CFG6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFG6`"]
pub struct CFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Boot mode override bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG7_A {
    #[doc = "0: Boot in RUN."]
    _0 = 0,
    #[doc = "1: Boot in VLPR."]
    _1 = 1,
}
impl From<CFG7_A> for bool {
    #[inline(always)]
    fn from(variant: CFG7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CFG7`"]
pub type CFG7_R = crate::R<bool, CFG7_A>;
impl CFG7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG7_A {
        match self.bits {
            false => CFG7_A::_0,
            true => CFG7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFG7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFG7_A::_1
    }
}
#[doc = "Write proxy for field `CFG7`"]
pub struct CFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Boot in RUN."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFG7_A::_0)
    }
    #[doc = "Boot in VLPR."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFG7_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CFG8`"]
pub type CFG8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFG8`"]
pub struct CFG8_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CFG9`"]
pub type CFG9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFG9`"]
pub struct CFG9_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CFG10`"]
pub type CFG10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFG10`"]
pub struct CFG10_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CFG11`"]
pub type CFG11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFG11`"]
pub struct CFG11_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `CFG12`"]
pub type CFG12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFG12`"]
pub struct CFG12_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `CFG13`"]
pub type CFG13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFG13`"]
pub struct CFG13_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `CFG14`"]
pub type CFG14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFG14`"]
pub struct CFG14_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CFG15`"]
pub type CFG15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFG15`"]
pub struct CFG15_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 32 kHz RTC OSC Control."]
    #[inline(always)]
    pub fn cfg0(&self) -> CFG0_R {
        CFG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Switched capacitor 2 pF enable."]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Switched capacitor 4 pF enable."]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Switched capacitor 8 pF enable."]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Switched capacitor 16 pF enable."]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Reserved bits"]
    #[inline(always)]
    pub fn cfg5(&self) -> CFG5_R {
        CFG5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reserved bits"]
    #[inline(always)]
    pub fn cfg6(&self) -> CFG6_R {
        CFG6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Boot mode override bit."]
    #[inline(always)]
    pub fn cfg7(&self) -> CFG7_R {
        CFG7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Reserved bits"]
    #[inline(always)]
    pub fn cfg8(&self) -> CFG8_R {
        CFG8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reserved bits"]
    #[inline(always)]
    pub fn cfg9(&self) -> CFG9_R {
        CFG9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reserved bits"]
    #[inline(always)]
    pub fn cfg10(&self) -> CFG10_R {
        CFG10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Reserved bits"]
    #[inline(always)]
    pub fn cfg11(&self) -> CFG11_R {
        CFG11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Reserved bits"]
    #[inline(always)]
    pub fn cfg12(&self) -> CFG12_R {
        CFG12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Reserved bits"]
    #[inline(always)]
    pub fn cfg13(&self) -> CFG13_R {
        CFG13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Reserved bits"]
    #[inline(always)]
    pub fn cfg14(&self) -> CFG14_R {
        CFG14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Reserved bits"]
    #[inline(always)]
    pub fn cfg15(&self) -> CFG15_R {
        CFG15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 32 kHz RTC OSC Control."]
    #[inline(always)]
    pub fn cfg0(&mut self) -> CFG0_W {
        CFG0_W { w: self }
    }
    #[doc = "Bit 1 - Switched capacitor 2 pF enable."]
    #[inline(always)]
    pub fn cfg1(&mut self) -> CFG1_W {
        CFG1_W { w: self }
    }
    #[doc = "Bit 2 - Switched capacitor 4 pF enable."]
    #[inline(always)]
    pub fn cfg2(&mut self) -> CFG2_W {
        CFG2_W { w: self }
    }
    #[doc = "Bit 3 - Switched capacitor 8 pF enable."]
    #[inline(always)]
    pub fn cfg3(&mut self) -> CFG3_W {
        CFG3_W { w: self }
    }
    #[doc = "Bit 4 - Switched capacitor 16 pF enable."]
    #[inline(always)]
    pub fn cfg4(&mut self) -> CFG4_W {
        CFG4_W { w: self }
    }
    #[doc = "Bit 5 - Reserved bits"]
    #[inline(always)]
    pub fn cfg5(&mut self) -> CFG5_W {
        CFG5_W { w: self }
    }
    #[doc = "Bit 6 - Reserved bits"]
    #[inline(always)]
    pub fn cfg6(&mut self) -> CFG6_W {
        CFG6_W { w: self }
    }
    #[doc = "Bit 7 - Boot mode override bit."]
    #[inline(always)]
    pub fn cfg7(&mut self) -> CFG7_W {
        CFG7_W { w: self }
    }
    #[doc = "Bit 8 - Reserved bits"]
    #[inline(always)]
    pub fn cfg8(&mut self) -> CFG8_W {
        CFG8_W { w: self }
    }
    #[doc = "Bit 9 - Reserved bits"]
    #[inline(always)]
    pub fn cfg9(&mut self) -> CFG9_W {
        CFG9_W { w: self }
    }
    #[doc = "Bit 10 - Reserved bits"]
    #[inline(always)]
    pub fn cfg10(&mut self) -> CFG10_W {
        CFG10_W { w: self }
    }
    #[doc = "Bit 11 - Reserved bits"]
    #[inline(always)]
    pub fn cfg11(&mut self) -> CFG11_W {
        CFG11_W { w: self }
    }
    #[doc = "Bit 12 - Reserved bits"]
    #[inline(always)]
    pub fn cfg12(&mut self) -> CFG12_W {
        CFG12_W { w: self }
    }
    #[doc = "Bit 13 - Reserved bits"]
    #[inline(always)]
    pub fn cfg13(&mut self) -> CFG13_W {
        CFG13_W { w: self }
    }
    #[doc = "Bit 14 - Reserved bits"]
    #[inline(always)]
    pub fn cfg14(&mut self) -> CFG14_W {
        CFG14_W { w: self }
    }
    #[doc = "Bit 15 - Reserved bits"]
    #[inline(always)]
    pub fn cfg15(&mut self) -> CFG15_W {
        CFG15_W { w: self }
    }
}
