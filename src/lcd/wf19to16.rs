#[doc = "Reader of register WF19TO16"]
pub type R = crate::R<u32, super::WF19TO16>;
#[doc = "Writer for register WF19TO16"]
pub type W = crate::W<u32, super::WF19TO16>;
#[doc = "Register WF19TO16 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF19TO16 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WF16`"]
pub type WF16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF16`"]
pub struct WF16_W<'a> {
    w: &'a mut W,
}
impl<'a> WF16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WF17`"]
pub type WF17_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF17`"]
pub struct WF17_W<'a> {
    w: &'a mut W,
}
impl<'a> WF17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WF18`"]
pub type WF18_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF18`"]
pub struct WF18_W<'a> {
    w: &'a mut W,
}
impl<'a> WF18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WF19`"]
pub type WF19_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF19`"]
pub struct WF19_W<'a> {
    w: &'a mut W,
}
impl<'a> WF19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P16 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf16(&self) -> WF16_R {
        WF16_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P17 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf17(&self) -> WF17_R {
        WF17_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P18 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf18(&self) -> WF18_R {
        WF18_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P19 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf19(&self) -> WF19_R {
        WF19_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P16 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf16(&mut self) -> WF16_W {
        WF16_W { w: self }
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P17 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf17(&mut self) -> WF17_W {
        WF17_W { w: self }
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P18 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf18(&mut self) -> WF18_W {
        WF18_W { w: self }
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P19 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf19(&mut self) -> WF19_W {
        WF19_W { w: self }
    }
}
