#[doc = "Reader of register WF51TO48"]
pub type R = crate::R<u32, super::WF51TO48>;
#[doc = "Writer for register WF51TO48"]
pub type W = crate::W<u32, super::WF51TO48>;
#[doc = "Register WF51TO48 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF51TO48 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WF48`"]
pub type WF48_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF48`"]
pub struct WF48_W<'a> {
    w: &'a mut W,
}
impl<'a> WF48_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WF49`"]
pub type WF49_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF49`"]
pub struct WF49_W<'a> {
    w: &'a mut W,
}
impl<'a> WF49_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WF50`"]
pub type WF50_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF50`"]
pub struct WF50_W<'a> {
    w: &'a mut W,
}
impl<'a> WF50_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WF51`"]
pub type WF51_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF51`"]
pub struct WF51_W<'a> {
    w: &'a mut W,
}
impl<'a> WF51_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P48 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf48(&self) -> WF48_R {
        WF48_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P49 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf49(&self) -> WF49_R {
        WF49_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P50 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf50(&self) -> WF50_R {
        WF50_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P51 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf51(&self) -> WF51_R {
        WF51_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P48 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf48(&mut self) -> WF48_W {
        WF48_W { w: self }
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P49 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf49(&mut self) -> WF49_W {
        WF49_W { w: self }
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P50 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf50(&mut self) -> WF50_W {
        WF50_W { w: self }
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P51 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf51(&mut self) -> WF51_W {
        WF51_W { w: self }
    }
}
