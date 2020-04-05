#[doc = "Reader of register BPEN%s"]
pub type R = crate::R<u32, super::BPEN>;
#[doc = "Writer for register BPEN%s"]
pub type W = crate::W<u32, super::BPEN>;
#[doc = "Register BPEN%s `reset()`'s with value 0"]
impl crate::ResetValue for super::BPEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Back Plane Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum BPEN_A {
    #[doc = "0: Front plane operation enabled on LCD_Pn."]
    _0 = 0,
    #[doc = "1: Back plane operation enabled on LCD_Pn."]
    _1 = 1,
}
impl From<BPEN_A> for u32 {
    #[inline(always)]
    fn from(variant: BPEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BPEN`"]
pub type BPEN_R = crate::R<u32, BPEN_A>;
impl BPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, BPEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BPEN_A::_0),
            1 => Val(BPEN_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BPEN_A::_1
    }
}
#[doc = "Write proxy for field `BPEN`"]
pub struct BPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Front plane operation enabled on LCD_Pn."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BPEN_A::_0)
    }
    #[doc = "Back plane operation enabled on LCD_Pn."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BPEN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Back Plane Enable"]
    #[inline(always)]
    pub fn bpen(&self) -> BPEN_R {
        BPEN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Back Plane Enable"]
    #[inline(always)]
    pub fn bpen(&mut self) -> BPEN_W {
        BPEN_W { w: self }
    }
}
