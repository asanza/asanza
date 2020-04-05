#[doc = "Reader of register TAMPER_SCR"]
pub type R = crate::R<u16, super::TAMPER_SCR>;
#[doc = "Writer for register TAMPER_SCR"]
pub type W = crate::W<u16, super::TAMPER_SCR>;
#[doc = "Register TAMPER_SCR `reset()`'s with value 0x080f"]
impl crate::ResetValue for super::TAMPER_SCR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x080f
    }
}
#[doc = "Reader of field `TMPR_EN`"]
pub type TMPR_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMPR_EN`"]
pub struct TMPR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMPR_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `TMPR_STS`"]
pub type TMPR_STS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMPR_STS`"]
pub struct TMPR_STS_W<'a> {
    w: &'a mut W,
}
impl<'a> TMPR_STS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Tamper Control"]
    #[inline(always)]
    pub fn tmpr_en(&self) -> TMPR_EN_R {
        TMPR_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Tamper Status Bit"]
    #[inline(always)]
    pub fn tmpr_sts(&self) -> TMPR_STS_R {
        TMPR_STS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Tamper Control"]
    #[inline(always)]
    pub fn tmpr_en(&mut self) -> TMPR_EN_W {
        TMPR_EN_W { w: self }
    }
    #[doc = "Bits 8:11 - Tamper Status Bit"]
    #[inline(always)]
    pub fn tmpr_sts(&mut self) -> TMPR_STS_W {
        TMPR_STS_W { w: self }
    }
}
