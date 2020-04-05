#[doc = "Reader of register WF43TO40"]
pub type R = crate::R<u32, super::WF43TO40>;
#[doc = "Writer for register WF43TO40"]
pub type W = crate::W<u32, super::WF43TO40>;
#[doc = "Register WF43TO40 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF43TO40 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WF40`"]
pub type WF40_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF40`"]
pub struct WF40_W<'a> {
    w: &'a mut W,
}
impl<'a> WF40_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WF41`"]
pub type WF41_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF41`"]
pub struct WF41_W<'a> {
    w: &'a mut W,
}
impl<'a> WF41_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WF42`"]
pub type WF42_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF42`"]
pub struct WF42_W<'a> {
    w: &'a mut W,
}
impl<'a> WF42_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WF43`"]
pub type WF43_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF43`"]
pub struct WF43_W<'a> {
    w: &'a mut W,
}
impl<'a> WF43_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P40 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf40(&self) -> WF40_R {
        WF40_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P41 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf41(&self) -> WF41_R {
        WF41_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P42 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf42(&self) -> WF42_R {
        WF42_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P43 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf43(&self) -> WF43_R {
        WF43_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P40 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf40(&mut self) -> WF40_W {
        WF40_W { w: self }
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P41 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf41(&mut self) -> WF41_W {
        WF41_W { w: self }
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P42 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf42(&mut self) -> WF42_W {
        WF42_W { w: self }
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P43 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf43(&mut self) -> WF43_W {
        WF43_W { w: self }
    }
}
