#[doc = "Reader of register FDSR"]
pub type R = crate::R<u32, super::FDSR>;
#[doc = "Writer for register FDSR"]
pub type W = crate::W<u32, super::FDSR>;
#[doc = "Register FDSR `reset()`'s with value 0"]
impl crate::ResetValue for super::FDSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Fault Detect Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FDCNT_A {
    #[doc = "0: No \"one\" samples."]
    _0 = 0,
    #[doc = "1: 1 \"one\" samples."]
    _1 = 1,
    #[doc = "2: 2 \"one\" samples."]
    _10 = 2,
    #[doc = "254: 254 \"one\" samples."]
    _11111110 = 254,
    #[doc = "255: 255 or more \"one\" samples. The FDCNT can overflow. Therefore, FDSWW and FDPRS must be reconfigured for proper sampling."]
    _11111111 = 255,
}
impl From<FDCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: FDCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FDCNT`"]
pub type FDCNT_R = crate::R<u8, FDCNT_A>;
impl FDCNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FDCNT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FDCNT_A::_0),
            1 => Val(FDCNT_A::_1),
            2 => Val(FDCNT_A::_10),
            254 => Val(FDCNT_A::_11111110),
            255 => Val(FDCNT_A::_11111111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDCNT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDCNT_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FDCNT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11111110`"]
    #[inline(always)]
    pub fn is_11111110(&self) -> bool {
        *self == FDCNT_A::_11111110
    }
    #[doc = "Checks if the value of the field is `_11111111`"]
    #[inline(always)]
    pub fn is_11111111(&self) -> bool {
        *self == FDCNT_A::_11111111
    }
}
#[doc = "Fault Detection Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDCF_A {
    #[doc = "0: Fault detection is not completed."]
    _0 = 0,
    #[doc = "1: Fault detection is completed."]
    _1 = 1,
}
impl From<FDCF_A> for bool {
    #[inline(always)]
    fn from(variant: FDCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FDCF`"]
pub type FDCF_R = crate::R<bool, FDCF_A>;
impl FDCF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDCF_A {
        match self.bits {
            false => FDCF_A::_0,
            true => FDCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDCF_A::_1
    }
}
#[doc = "Write proxy for field `FDCF`"]
pub struct FDCF_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDCF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault detection is not completed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDCF_A::_0)
    }
    #[doc = "Fault detection is completed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDCF_A::_1)
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
impl R {
    #[doc = "Bits 0:7 - Fault Detect Counter"]
    #[inline(always)]
    pub fn fdcnt(&self) -> FDCNT_R {
        FDCNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Fault Detection Complete Flag"]
    #[inline(always)]
    pub fn fdcf(&self) -> FDCF_R {
        FDCF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Fault Detection Complete Flag"]
    #[inline(always)]
    pub fn fdcf(&mut self) -> FDCF_W {
        FDCF_W { w: self }
    }
}
