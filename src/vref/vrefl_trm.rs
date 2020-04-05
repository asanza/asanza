#[doc = "Reader of register VREFL_TRM"]
pub type R = crate::R<u8, super::VREFL_TRM>;
#[doc = "Writer for register VREFL_TRM"]
pub type W = crate::W<u8, super::VREFL_TRM>;
#[doc = "Register VREFL_TRM `reset()`'s with value 0x03"]
impl crate::ResetValue for super::VREFL_TRM {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `VREFL_TRIM`"]
pub type VREFL_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VREFL_TRIM`"]
pub struct VREFL_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFL_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "This bit enables the VREFL (0.4 V) reference buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFL_EN_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<VREFL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFL_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VREFL_EN`"]
pub type VREFL_EN_R = crate::R<bool, VREFL_EN_A>;
impl VREFL_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFL_EN_A {
        match self.bits {
            false => VREFL_EN_A::_0,
            true => VREFL_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VREFL_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VREFL_EN_A::_1
    }
}
#[doc = "Write proxy for field `VREFL_EN`"]
pub struct VREFL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFL_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFL_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VREFL_EN_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VREFL_EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "This bit selects between internal and external 1.2 V reference.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFL_SEL_A {
    #[doc = "0: Internal reference"]
    _0 = 0,
    #[doc = "1: External reference"]
    _1 = 1,
}
impl From<VREFL_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: VREFL_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VREFL_SEL`"]
pub type VREFL_SEL_R = crate::R<bool, VREFL_SEL_A>;
impl VREFL_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFL_SEL_A {
        match self.bits {
            false => VREFL_SEL_A::_0,
            true => VREFL_SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VREFL_SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VREFL_SEL_A::_1
    }
}
#[doc = "Write proxy for field `VREFL_SEL`"]
pub struct VREFL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFL_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFL_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal reference"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VREFL_SEL_A::_0)
    }
    #[doc = "External reference"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VREFL_SEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - These bits trim the VREFL reference voltage in steps of 10 mV"]
    #[inline(always)]
    pub fn vrefl_trim(&self) -> VREFL_TRIM_R {
        VREFL_TRIM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - This bit enables the VREFL (0.4 V) reference buffer."]
    #[inline(always)]
    pub fn vrefl_en(&self) -> VREFL_EN_R {
        VREFL_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit selects between internal and external 1.2 V reference."]
    #[inline(always)]
    pub fn vrefl_sel(&self) -> VREFL_SEL_R {
        VREFL_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - These bits trim the VREFL reference voltage in steps of 10 mV"]
    #[inline(always)]
    pub fn vrefl_trim(&mut self) -> VREFL_TRIM_W {
        VREFL_TRIM_W { w: self }
    }
    #[doc = "Bit 3 - This bit enables the VREFL (0.4 V) reference buffer."]
    #[inline(always)]
    pub fn vrefl_en(&mut self) -> VREFL_EN_W {
        VREFL_EN_W { w: self }
    }
    #[doc = "Bit 4 - This bit selects between internal and external 1.2 V reference."]
    #[inline(always)]
    pub fn vrefl_sel(&mut self) -> VREFL_SEL_W {
        VREFL_SEL_W { w: self }
    }
}
