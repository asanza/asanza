#[doc = "Reader of register SECONDS"]
pub type R = crate::R<u16, super::SECONDS>;
#[doc = "Writer for register SECONDS"]
pub type W = crate::W<u16, super::SECONDS>;
#[doc = "Register SECONDS `reset()`'s with value 0"]
impl crate::ResetValue for super::SECONDS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEC_CNT`"]
pub type SEC_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEC_CNT`"]
pub struct SEC_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Seconds Counter Value."]
    #[inline(always)]
    pub fn sec_cnt(&self) -> SEC_CNT_R {
        SEC_CNT_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds Counter Value."]
    #[inline(always)]
    pub fn sec_cnt(&mut self) -> SEC_CNT_W {
        SEC_CNT_W { w: self }
    }
}
