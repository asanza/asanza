#[doc = "Writer for register PTOR"]
pub type W = crate::W<u8, super::PTOR>;
#[doc = "Register PTOR `reset()`'s with value 0"]
impl crate::ResetValue for super::PTOR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PTTO_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO_AW> for u8 {
    #[inline(always)]
    fn from(variant: PTTO_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `PTTO`"]
pub struct PTTO_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTTO_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO_AW::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto(&mut self) -> PTTO_W {
        PTTO_W { w: self }
    }
}
