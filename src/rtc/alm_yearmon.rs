#[doc = "Reader of register ALM_YEARMON"]
pub type R = crate::R<u16, super::ALM_YEARMON>;
#[doc = "Writer for register ALM_YEARMON"]
pub type W = crate::W<u16, super::ALM_YEARMON>;
#[doc = "Register ALM_YEARMON `reset()`'s with value 0"]
impl crate::ResetValue for super::ALM_YEARMON {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALM_MON`"]
pub type ALM_MON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALM_MON`"]
pub struct ALM_MON_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_MON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `ALM_YEAR`"]
pub type ALM_YEAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALM_YEAR`"]
pub struct ALM_YEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_YEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Months Value for Alarm."]
    #[inline(always)]
    pub fn alm_mon(&self) -> ALM_MON_R {
        ALM_MON_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Year Value for Alarm."]
    #[inline(always)]
    pub fn alm_year(&self) -> ALM_YEAR_R {
        ALM_YEAR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Months Value for Alarm."]
    #[inline(always)]
    pub fn alm_mon(&mut self) -> ALM_MON_W {
        ALM_MON_W { w: self }
    }
    #[doc = "Bits 8:15 - Year Value for Alarm."]
    #[inline(always)]
    pub fn alm_year(&mut self) -> ALM_YEAR_W {
        ALM_YEAR_W { w: self }
    }
}
