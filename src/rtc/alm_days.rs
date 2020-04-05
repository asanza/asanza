#[doc = "Reader of register ALM_DAYS"]
pub type R = crate::R<u16, super::ALM_DAYS>;
#[doc = "Writer for register ALM_DAYS"]
pub type W = crate::W<u16, super::ALM_DAYS>;
#[doc = "Register ALM_DAYS `reset()`'s with value 0"]
impl crate::ResetValue for super::ALM_DAYS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALM_DAY`"]
pub type ALM_DAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALM_DAY`"]
pub struct ALM_DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Days Value for Alarm."]
    #[inline(always)]
    pub fn alm_day(&self) -> ALM_DAY_R {
        ALM_DAY_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Days Value for Alarm."]
    #[inline(always)]
    pub fn alm_day(&mut self) -> ALM_DAY_W {
        ALM_DAY_W { w: self }
    }
}
