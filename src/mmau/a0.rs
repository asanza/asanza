#[doc = "Reader of register A0"]
pub type R = crate::R<u32, super::A0>;
#[doc = "Writer for register A0"]
pub type W = crate::W<u32, super::A0>;
#[doc = "Register A0 `reset()`'s with value 0"]
impl crate::ResetValue for super::A0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `A0`"]
pub type A0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `A0`"]
pub struct A0_W<'a> {
    w: &'a mut W,
}
impl<'a> A0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Accumulator Register A0"]
    #[inline(always)]
    pub fn a0(&self) -> A0_R {
        A0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Accumulator Register A0"]
    #[inline(always)]
    pub fn a0(&mut self) -> A0_W {
        A0_W { w: self }
    }
}
