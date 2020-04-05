#[doc = "Reader of register X1"]
pub type R = crate::R<u32, super::X1>;
#[doc = "Writer for register X1"]
pub type W = crate::W<u32, super::X1>;
#[doc = "Register X1 `reset()`'s with value 0"]
impl crate::ResetValue for super::X1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `X1`"]
pub type X1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `X1`"]
pub struct X1_W<'a> {
    w: &'a mut W,
}
impl<'a> X1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Operand Register X1"]
    #[inline(always)]
    pub fn x1(&self) -> X1_R {
        X1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Operand Register X1"]
    #[inline(always)]
    pub fn x1(&mut self) -> X1_W {
        X1_W { w: self }
    }
}
