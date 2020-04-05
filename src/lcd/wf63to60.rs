#[doc = "Reader of register WF63TO60"]
pub type R = crate::R<u32, super::WF63TO60>;
#[doc = "Writer for register WF63TO60"]
pub type W = crate::W<u32, super::WF63TO60>;
#[doc = "Register WF63TO60 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF63TO60 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WF60`"]
pub type WF60_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF60`"]
pub struct WF60_W<'a> {
    w: &'a mut W,
}
impl<'a> WF60_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WF61`"]
pub type WF61_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF61`"]
pub struct WF61_W<'a> {
    w: &'a mut W,
}
impl<'a> WF61_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WF62`"]
pub type WF62_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF62`"]
pub struct WF62_W<'a> {
    w: &'a mut W,
}
impl<'a> WF62_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WF63`"]
pub type WF63_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF63`"]
pub struct WF63_W<'a> {
    w: &'a mut W,
}
impl<'a> WF63_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P60 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf60(&self) -> WF60_R {
        WF60_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P61 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf61(&self) -> WF61_R {
        WF61_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P62 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf62(&self) -> WF62_R {
        WF62_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P63 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf63(&self) -> WF63_R {
        WF63_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P60 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf60(&mut self) -> WF60_W {
        WF60_W { w: self }
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P61 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf61(&mut self) -> WF61_W {
        WF61_W { w: self }
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P62 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf62(&mut self) -> WF62_W {
        WF62_W { w: self }
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P63 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf63(&mut self) -> WF63_W {
        WF63_W { w: self }
    }
}
