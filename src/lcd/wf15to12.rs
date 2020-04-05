#[doc = "Reader of register WF15TO12"]
pub type R = crate::R<u32, super::WF15TO12>;
#[doc = "Writer for register WF15TO12"]
pub type W = crate::W<u32, super::WF15TO12>;
#[doc = "Register WF15TO12 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF15TO12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WF12`"]
pub type WF12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF12`"]
pub struct WF12_W<'a> {
    w: &'a mut W,
}
impl<'a> WF12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WF13`"]
pub type WF13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF13`"]
pub struct WF13_W<'a> {
    w: &'a mut W,
}
impl<'a> WF13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WF14`"]
pub type WF14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF14`"]
pub struct WF14_W<'a> {
    w: &'a mut W,
}
impl<'a> WF14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WF15`"]
pub type WF15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF15`"]
pub struct WF15_W<'a> {
    w: &'a mut W,
}
impl<'a> WF15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P12 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf12(&self) -> WF12_R {
        WF12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P13 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf13(&self) -> WF13_R {
        WF13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P14 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf14(&self) -> WF14_R {
        WF14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P15 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf15(&self) -> WF15_R {
        WF15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P12 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf12(&mut self) -> WF12_W {
        WF12_W { w: self }
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P13 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf13(&mut self) -> WF13_W {
        WF13_W { w: self }
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P14 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf14(&mut self) -> WF14_W {
        WF14_W { w: self }
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P15 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf15(&mut self) -> WF15_W {
        WF15_W { w: self }
    }
}
