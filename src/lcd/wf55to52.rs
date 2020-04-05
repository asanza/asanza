#[doc = "Reader of register WF55TO52"]
pub type R = crate::R<u32, super::WF55TO52>;
#[doc = "Writer for register WF55TO52"]
pub type W = crate::W<u32, super::WF55TO52>;
#[doc = "Register WF55TO52 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF55TO52 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WF52`"]
pub type WF52_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF52`"]
pub struct WF52_W<'a> {
    w: &'a mut W,
}
impl<'a> WF52_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WF53`"]
pub type WF53_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF53`"]
pub struct WF53_W<'a> {
    w: &'a mut W,
}
impl<'a> WF53_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WF54`"]
pub type WF54_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF54`"]
pub struct WF54_W<'a> {
    w: &'a mut W,
}
impl<'a> WF54_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WF55`"]
pub type WF55_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF55`"]
pub struct WF55_W<'a> {
    w: &'a mut W,
}
impl<'a> WF55_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P52 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf52(&self) -> WF52_R {
        WF52_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P53 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf53(&self) -> WF53_R {
        WF53_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P54 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf54(&self) -> WF54_R {
        WF54_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P55 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf55(&self) -> WF55_R {
        WF55_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P52 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf52(&mut self) -> WF52_W {
        WF52_W { w: self }
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P53 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf53(&mut self) -> WF53_W {
        WF53_W { w: self }
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P54 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf54(&mut self) -> WF54_W {
        WF54_W { w: self }
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P55 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf55(&mut self) -> WF55_W {
        WF55_W { w: self }
    }
}
