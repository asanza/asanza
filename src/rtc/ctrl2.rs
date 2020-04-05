#[doc = "Reader of register CTRL2"]
pub type R = crate::R<u16, super::CTRL2>;
#[doc = "Writer for register CTRL2"]
pub type W = crate::W<u16, super::CTRL2>;
#[doc = "Register CTRL2 `reset()`'s with value 0x80"]
impl crate::ResetValue for super::CTRL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "Tamper Configuration Over\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP_CFG_OVER_A {
    #[doc = "0: Tamper filter processing disabled."]
    _0 = 0,
    #[doc = "1: Tamper filter processing enabled. To enable the tamper feature, this bitfield should be set."]
    _1 = 1,
}
impl From<TAMP_CFG_OVER_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP_CFG_OVER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TAMP_CFG_OVER`"]
pub type TAMP_CFG_OVER_R = crate::R<bool, TAMP_CFG_OVER_A>;
impl TAMP_CFG_OVER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP_CFG_OVER_A {
        match self.bits {
            false => TAMP_CFG_OVER_A::_0,
            true => TAMP_CFG_OVER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TAMP_CFG_OVER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TAMP_CFG_OVER_A::_1
    }
}
#[doc = "Write proxy for field `TAMP_CFG_OVER`"]
pub struct TAMP_CFG_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP_CFG_OVER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP_CFG_OVER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tamper filter processing disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TAMP_CFG_OVER_A::_0)
    }
    #[doc = "Tamper filter processing enabled. To enable the tamper feature, this bitfield should be set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TAMP_CFG_OVER_A::_1)
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
#[doc = "Wakeup Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAKEUP_STATUS_A {
    #[doc = "0: The wakeup/hibernation pin is in HiZ mode."]
    _00 = 0,
    #[doc = "1: The wakeup/hibernation pin is at logic 0. MCU is in sleep mode."]
    _01 = 1,
    #[doc = "2: The wakeup/ hibernation pin is at logic 1. MCU is in sleep mode."]
    _10 = 2,
}
impl From<WAKEUP_STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: WAKEUP_STATUS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WAKEUP_STATUS`"]
pub type WAKEUP_STATUS_R = crate::R<u8, WAKEUP_STATUS_A>;
impl WAKEUP_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WAKEUP_STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WAKEUP_STATUS_A::_00),
            1 => Val(WAKEUP_STATUS_A::_01),
            2 => Val(WAKEUP_STATUS_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WAKEUP_STATUS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WAKEUP_STATUS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WAKEUP_STATUS_A::_10
    }
}
#[doc = "Write proxy for field `WAKEUP_STATUS`"]
pub struct WAKEUP_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP_STATUS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The wakeup/hibernation pin is in HiZ mode."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WAKEUP_STATUS_A::_00)
    }
    #[doc = "The wakeup/hibernation pin is at logic 0. MCU is in sleep mode."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WAKEUP_STATUS_A::_01)
    }
    #[doc = "The wakeup/ hibernation pin is at logic 1. MCU is in sleep mode."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WAKEUP_STATUS_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u16) & 0x03) << 5);
        self.w
    }
}
#[doc = "Wakeup Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_MODE_A {
    #[doc = "0: Tamper pin 0 is used as the tamper pin."]
    _0 = 0,
    #[doc = "1: Tamper pin 0 is used as a wakeup/hibernation pin."]
    _1 = 1,
}
impl From<WAKEUP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAKEUP_MODE`"]
pub type WAKEUP_MODE_R = crate::R<bool, WAKEUP_MODE_A>;
impl WAKEUP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_MODE_A {
        match self.bits {
            false => WAKEUP_MODE_A::_0,
            true => WAKEUP_MODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAKEUP_MODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAKEUP_MODE_A::_1
    }
}
#[doc = "Write proxy for field `WAKEUP_MODE`"]
pub struct WAKEUP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tamper pin 0 is used as the tamper pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKEUP_MODE_A::_0)
    }
    #[doc = "Tamper pin 0 is used as a wakeup/hibernation pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKEUP_MODE_A::_1)
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
impl R {
    #[doc = "Bit 0 - Tamper Configuration Over"]
    #[inline(always)]
    pub fn tamp_cfg_over(&self) -> TAMP_CFG_OVER_R {
        TAMP_CFG_OVER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Wakeup Status"]
    #[inline(always)]
    pub fn wakeup_status(&self) -> WAKEUP_STATUS_R {
        WAKEUP_STATUS_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Wakeup Mode"]
    #[inline(always)]
    pub fn wakeup_mode(&self) -> WAKEUP_MODE_R {
        WAKEUP_MODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper Configuration Over"]
    #[inline(always)]
    pub fn tamp_cfg_over(&mut self) -> TAMP_CFG_OVER_W {
        TAMP_CFG_OVER_W { w: self }
    }
    #[doc = "Bits 5:6 - Wakeup Status"]
    #[inline(always)]
    pub fn wakeup_status(&mut self) -> WAKEUP_STATUS_W {
        WAKEUP_STATUS_W { w: self }
    }
    #[doc = "Bit 7 - Wakeup Mode"]
    #[inline(always)]
    pub fn wakeup_mode(&mut self) -> WAKEUP_MODE_W {
        WAKEUP_MODE_W { w: self }
    }
}
