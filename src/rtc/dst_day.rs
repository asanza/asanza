#[doc = "Reader of register DST_DAY"]
pub type R = crate::R<u16, super::DST_DAY>;
#[doc = "Writer for register DST_DAY"]
pub type W = crate::W<u16, super::DST_DAY>;
#[doc = "Register DST_DAY `reset()`'s with value 0"]
impl crate::ResetValue for super::DST_DAY {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DST_END_DAY`"]
pub type DST_END_DAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DST_END_DAY`"]
pub struct DST_END_DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_END_DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `DST_START_DAY`"]
pub type DST_START_DAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DST_START_DAY`"]
pub struct DST_START_DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_START_DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u16) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Daylight Saving Time (DST) Day End Value."]
    #[inline(always)]
    pub fn dst_end_day(&self) -> DST_END_DAY_R {
        DST_END_DAY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Daylight Saving Time (DST) Day Start Value."]
    #[inline(always)]
    pub fn dst_start_day(&self) -> DST_START_DAY_R {
        DST_START_DAY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Daylight Saving Time (DST) Day End Value."]
    #[inline(always)]
    pub fn dst_end_day(&mut self) -> DST_END_DAY_W {
        DST_END_DAY_W { w: self }
    }
    #[doc = "Bits 8:12 - Daylight Saving Time (DST) Day Start Value."]
    #[inline(always)]
    pub fn dst_start_day(&mut self) -> DST_START_DAY_W {
        DST_START_DAY_W { w: self }
    }
}
