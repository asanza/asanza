#[doc = "Reader of register X3"]
pub type R = crate::R<u32, super::X3>;
#[doc = "Writer for register X3"]
pub type W = crate::W<u32, super::X3>;
#[doc = "Register X3 `reset()`'s with value 0"]
impl crate::ResetValue for super::X3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `X3`"]
pub type X3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `X3`"]
pub struct X3_W<'a> {
    w: &'a mut W,
}
impl<'a> X3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Operand Register X3"]
    #[inline(always)]
    pub fn x3(&self) -> X3_R {
        X3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Operand Register X3"]
    #[inline(always)]
    pub fn x3(&mut self) -> X3_W {
        X3_W { w: self }
    }
}
