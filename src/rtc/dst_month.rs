#[doc = "Reader of register DST_MONTH"]
pub type R = crate::R<u16, super::DST_MONTH>;
#[doc = "Writer for register DST_MONTH"]
pub type W = crate::W<u16, super::DST_MONTH>;
#[doc = "Register DST_MONTH `reset()`'s with value 0"]
impl crate::ResetValue for super::DST_MONTH {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DST_END_MONTH`"]
pub type DST_END_MONTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DST_END_MONTH`"]
pub struct DST_END_MONTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_END_MONTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DST_START_MONTH`"]
pub type DST_START_MONTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DST_START_MONTH`"]
pub struct DST_START_MONTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_START_MONTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Daylight Saving Time (DST) Month End Value."]
    #[inline(always)]
    pub fn dst_end_month(&self) -> DST_END_MONTH_R {
        DST_END_MONTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Daylight Saving Time (DST) Month Start Value."]
    #[inline(always)]
    pub fn dst_start_month(&self) -> DST_START_MONTH_R {
        DST_START_MONTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Daylight Saving Time (DST) Month End Value."]
    #[inline(always)]
    pub fn dst_end_month(&mut self) -> DST_END_MONTH_W {
        DST_END_MONTH_W { w: self }
    }
    #[doc = "Bits 8:11 - Daylight Saving Time (DST) Month Start Value."]
    #[inline(always)]
    pub fn dst_start_month(&mut self) -> DST_START_MONTH_W {
        DST_START_MONTH_W { w: self }
    }
}
