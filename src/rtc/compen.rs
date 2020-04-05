#[doc = "Reader of register COMPEN"]
pub type R = crate::R<u16, super::COMPEN>;
#[doc = "Writer for register COMPEN"]
pub type W = crate::W<u16, super::COMPEN>;
#[doc = "Register COMPEN `reset()`'s with value 0"]
impl crate::ResetValue for super::COMPEN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMPEN_VAL`"]
pub type COMPEN_VAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMPEN_VAL`"]
pub struct COMPEN_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPEN_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Compensation Value"]
    #[inline(always)]
    pub fn compen_val(&self) -> COMPEN_VAL_R {
        COMPEN_VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compensation Value"]
    #[inline(always)]
    pub fn compen_val(&mut self) -> COMPEN_VAL_W {
        COMPEN_VAL_W { w: self }
    }
}
