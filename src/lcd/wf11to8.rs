#[doc = "Reader of register WF11TO8"]
pub type R = crate::R<u32, super::WF11TO8>;
#[doc = "Writer for register WF11TO8"]
pub type W = crate::W<u32, super::WF11TO8>;
#[doc = "Register WF11TO8 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF11TO8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WF8`"]
pub type WF8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF8`"]
pub struct WF8_W<'a> {
    w: &'a mut W,
}
impl<'a> WF8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WF9`"]
pub type WF9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF9`"]
pub struct WF9_W<'a> {
    w: &'a mut W,
}
impl<'a> WF9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WF10`"]
pub type WF10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF10`"]
pub struct WF10_W<'a> {
    w: &'a mut W,
}
impl<'a> WF10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WF11`"]
pub type WF11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF11`"]
pub struct WF11_W<'a> {
    w: &'a mut W,
}
impl<'a> WF11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P8 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf8(&self) -> WF8_R {
        WF8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P9 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf9(&self) -> WF9_R {
        WF9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P10 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf10(&self) -> WF10_R {
        WF10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P11 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf11(&self) -> WF11_R {
        WF11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P8 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf8(&mut self) -> WF8_W {
        WF8_W { w: self }
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P9 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf9(&mut self) -> WF9_W {
        WF9_W { w: self }
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P10 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf10(&mut self) -> WF10_W {
        WF10_W { w: self }
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P11 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf11(&mut self) -> WF11_W {
        WF11_W { w: self }
    }
}
