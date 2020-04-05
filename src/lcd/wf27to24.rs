#[doc = "Reader of register WF27TO24"]
pub type R = crate::R<u32, super::WF27TO24>;
#[doc = "Writer for register WF27TO24"]
pub type W = crate::W<u32, super::WF27TO24>;
#[doc = "Register WF27TO24 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF27TO24 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WF24`"]
pub type WF24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF24`"]
pub struct WF24_W<'a> {
    w: &'a mut W,
}
impl<'a> WF24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WF25`"]
pub type WF25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF25`"]
pub struct WF25_W<'a> {
    w: &'a mut W,
}
impl<'a> WF25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WF26`"]
pub type WF26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF26`"]
pub struct WF26_W<'a> {
    w: &'a mut W,
}
impl<'a> WF26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WF27`"]
pub type WF27_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF27`"]
pub struct WF27_W<'a> {
    w: &'a mut W,
}
impl<'a> WF27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P24 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf24(&self) -> WF24_R {
        WF24_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P25 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf25(&self) -> WF25_R {
        WF25_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P26 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf26(&self) -> WF26_R {
        WF26_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P27 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf27(&self) -> WF27_R {
        WF27_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P24 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf24(&mut self) -> WF24_W {
        WF24_W { w: self }
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P25 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf25(&mut self) -> WF25_W {
        WF25_W { w: self }
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P26 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf26(&mut self) -> WF26_W {
        WF26_W { w: self }
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P27 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf27(&mut self) -> WF27_W {
        WF27_W { w: self }
    }
}
