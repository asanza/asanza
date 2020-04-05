#[doc = "Reader of register WF7TO4"]
pub type R = crate::R<u32, super::WF7TO4>;
#[doc = "Writer for register WF7TO4"]
pub type W = crate::W<u32, super::WF7TO4>;
#[doc = "Register WF7TO4 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF7TO4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WF4`"]
pub type WF4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF4`"]
pub struct WF4_W<'a> {
    w: &'a mut W,
}
impl<'a> WF4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WF5`"]
pub type WF5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF5`"]
pub struct WF5_W<'a> {
    w: &'a mut W,
}
impl<'a> WF5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WF6`"]
pub type WF6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF6`"]
pub struct WF6_W<'a> {
    w: &'a mut W,
}
impl<'a> WF6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WF7`"]
pub type WF7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF7`"]
pub struct WF7_W<'a> {
    w: &'a mut W,
}
impl<'a> WF7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P4 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf4(&self) -> WF4_R {
        WF4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P5 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf5(&self) -> WF5_R {
        WF5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P6 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf6(&self) -> WF6_R {
        WF6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P7 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf7(&self) -> WF7_R {
        WF7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P4 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf4(&mut self) -> WF4_W {
        WF4_W { w: self }
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P5 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf5(&mut self) -> WF5_W {
        WF5_W { w: self }
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P6 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf6(&mut self) -> WF6_W {
        WF6_W { w: self }
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P7 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf7(&mut self) -> WF7_W {
        WF7_W { w: self }
    }
}
