#[doc = "Reader of register PEN%s"]
pub type R = crate::R<u32, super::PEN>;
#[doc = "Writer for register PEN%s"]
pub type W = crate::W<u32, super::PEN>;
#[doc = "Register PEN%s `reset()`'s with value 0"]
impl crate::ResetValue for super::PEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LCD Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PEN_A {
    #[doc = "0: LCD operation disabled on LCD_Pn."]
    _0 = 0,
    #[doc = "1: LCD operation enabled on LCD_Pn."]
    _1 = 1,
}
impl From<PEN_A> for u32 {
    #[inline(always)]
    fn from(variant: PEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PEN`"]
pub type PEN_R = crate::R<u32, PEN_A>;
impl PEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PEN_A::_0),
            1 => Val(PEN_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEN_A::_1
    }
}
#[doc = "Write proxy for field `PEN`"]
pub struct PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LCD operation disabled on LCD_Pn."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN_A::_0)
    }
    #[doc = "LCD operation enabled on LCD_Pn."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - LCD Pin Enable"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - LCD Pin Enable"]
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W {
        PEN_W { w: self }
    }
}
