#[doc = "Reader of register X2"]
pub type R = crate::R<u32, super::X2>;
#[doc = "Writer for register X2"]
pub type W = crate::W<u32, super::X2>;
#[doc = "Register X2 `reset()`'s with value 0"]
impl crate::ResetValue for super::X2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `X2`"]
pub type X2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `X2`"]
pub struct X2_W<'a> {
    w: &'a mut W,
}
impl<'a> X2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Operand Register X2"]
    #[inline(always)]
    pub fn x2(&self) -> X2_R {
        X2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Operand Register X2"]
    #[inline(always)]
    pub fn x2(&mut self) -> X2_W {
        X2_W { w: self }
    }
}
