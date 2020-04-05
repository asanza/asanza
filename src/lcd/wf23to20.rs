#[doc = "Reader of register WF23TO20"]
pub type R = crate::R<u32, super::WF23TO20>;
#[doc = "Writer for register WF23TO20"]
pub type W = crate::W<u32, super::WF23TO20>;
#[doc = "Register WF23TO20 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF23TO20 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WF20`"]
pub type WF20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF20`"]
pub struct WF20_W<'a> {
    w: &'a mut W,
}
impl<'a> WF20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WF21`"]
pub type WF21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF21`"]
pub struct WF21_W<'a> {
    w: &'a mut W,
}
impl<'a> WF21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WF22`"]
pub type WF22_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF22`"]
pub struct WF22_W<'a> {
    w: &'a mut W,
}
impl<'a> WF22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WF23`"]
pub type WF23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF23`"]
pub struct WF23_W<'a> {
    w: &'a mut W,
}
impl<'a> WF23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P20 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf20(&self) -> WF20_R {
        WF20_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P21 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf21(&self) -> WF21_R {
        WF21_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P22 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf22(&self) -> WF22_R {
        WF22_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P23 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf23(&self) -> WF23_R {
        WF23_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P20 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf20(&mut self) -> WF20_W {
        WF20_W { w: self }
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P21 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf21(&mut self) -> WF21_W {
        WF21_W { w: self }
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P22 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf22(&mut self) -> WF22_W {
        WF22_W { w: self }
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P23 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf23(&mut self) -> WF23_W {
        WF23_W { w: self }
    }
}
