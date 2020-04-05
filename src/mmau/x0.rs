#[doc = "Reader of register X0"]
pub type R = crate::R<u32, super::X0>;
#[doc = "Writer for register X0"]
pub type W = crate::W<u32, super::X0>;
#[doc = "Register X0 `reset()`'s with value 0"]
impl crate::ResetValue for super::X0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `X0`"]
pub type X0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `X0`"]
pub struct X0_W<'a> {
    w: &'a mut W,
}
impl<'a> X0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Operand Register X0"]
    #[inline(always)]
    pub fn x0(&self) -> X0_R {
        X0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Operand Register X0"]
    #[inline(always)]
    pub fn x0(&mut self) -> X0_W {
        X0_W { w: self }
    }
}
