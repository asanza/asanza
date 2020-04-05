#[doc = "Reader of register ALM_SECONDS"]
pub type R = crate::R<u16, super::ALM_SECONDS>;
#[doc = "Writer for register ALM_SECONDS"]
pub type W = crate::W<u16, super::ALM_SECONDS>;
#[doc = "Register ALM_SECONDS `reset()`'s with value 0"]
impl crate::ResetValue for super::ALM_SECONDS {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALM_SEC`"]
pub type ALM_SEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALM_SEC`"]
pub struct ALM_SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_SEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
    }
}
#[doc = "Write proxy for field `DEC_SEC`"]
pub struct DEC_SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> DEC_SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `INC_SEC`"]
pub struct INC_SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Seconds Value for Alarm. Same as Seconds Counter Value in SECONDSRTC Seconds Counters Register ."]
    #[inline(always)]
    pub fn alm_sec(&self) -> ALM_SEC_R {
        ALM_SEC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds Value for Alarm. Same as Seconds Counter Value in SECONDSRTC Seconds Counters Register ."]
    #[inline(always)]
    pub fn alm_sec(&mut self) -> ALM_SEC_W {
        ALM_SEC_W { w: self }
    }
    #[doc = "Bit 8 - Decrement Seconds Counter by 1."]
    #[inline(always)]
    pub fn dec_sec(&mut self) -> DEC_SEC_W {
        DEC_SEC_W { w: self }
    }
    #[doc = "Bit 9 - Increment Seconds Counter by 1."]
    #[inline(always)]
    pub fn inc_sec(&mut self) -> INC_SEC_W {
        INC_SEC_W { w: self }
    }
}
