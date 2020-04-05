#[doc = "Reader of register ALM_HOURMIN"]
pub type R = crate::R<u16, super::ALM_HOURMIN>;
#[doc = "Writer for register ALM_HOURMIN"]
pub type W = crate::W<u16, super::ALM_HOURMIN>;
#[doc = "Register ALM_HOURMIN `reset()`'s with value 0"]
impl crate::ResetValue for super::ALM_HOURMIN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALM_MIN`"]
pub type ALM_MIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALM_MIN`"]
pub struct ALM_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `ALM_HOUR`"]
pub type ALM_HOUR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALM_HOUR`"]
pub struct ALM_HOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_HOUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u16) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Minutes Value for Alarm."]
    #[inline(always)]
    pub fn alm_min(&self) -> ALM_MIN_R {
        ALM_MIN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Hours Value for Alarm."]
    #[inline(always)]
    pub fn alm_hour(&self) -> ALM_HOUR_R {
        ALM_HOUR_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Minutes Value for Alarm."]
    #[inline(always)]
    pub fn alm_min(&mut self) -> ALM_MIN_W {
        ALM_MIN_W { w: self }
    }
    #[doc = "Bits 8:12 - Hours Value for Alarm."]
    #[inline(always)]
    pub fn alm_hour(&mut self) -> ALM_HOUR_W {
        ALM_HOUR_W { w: self }
    }
}
