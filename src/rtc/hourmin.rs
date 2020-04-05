#[doc = "Reader of register HOURMIN"]
pub type R = crate::R<u16, super::HOURMIN>;
#[doc = "Writer for register HOURMIN"]
pub type W = crate::W<u16, super::HOURMIN>;
#[doc = "Register HOURMIN `reset()`'s with value 0"]
impl crate::ResetValue for super::HOURMIN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MIN_CNT`"]
pub type MIN_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MIN_CNT`"]
pub struct MIN_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `HOUR_CNT`"]
pub type HOUR_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOUR_CNT`"]
pub struct HOUR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u16) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Minutes Counter Value."]
    #[inline(always)]
    pub fn min_cnt(&self) -> MIN_CNT_R {
        MIN_CNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Hours Counter Value."]
    #[inline(always)]
    pub fn hour_cnt(&self) -> HOUR_CNT_R {
        HOUR_CNT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Minutes Counter Value."]
    #[inline(always)]
    pub fn min_cnt(&mut self) -> MIN_CNT_W {
        MIN_CNT_W { w: self }
    }
    #[doc = "Bits 8:12 - Hours Counter Value."]
    #[inline(always)]
    pub fn hour_cnt(&mut self) -> HOUR_CNT_W {
        HOUR_CNT_W { w: self }
    }
}
