#[doc = "Reader of register WF39TO36"]
pub type R = crate::R<u32, super::WF39TO36>;
#[doc = "Writer for register WF39TO36"]
pub type W = crate::W<u32, super::WF39TO36>;
#[doc = "Register WF39TO36 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF39TO36 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WF36`"]
pub type WF36_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF36`"]
pub struct WF36_W<'a> {
    w: &'a mut W,
}
impl<'a> WF36_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WF37`"]
pub type WF37_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF37`"]
pub struct WF37_W<'a> {
    w: &'a mut W,
}
impl<'a> WF37_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WF38`"]
pub type WF38_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF38`"]
pub struct WF38_W<'a> {
    w: &'a mut W,
}
impl<'a> WF38_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WF39`"]
pub type WF39_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF39`"]
pub struct WF39_W<'a> {
    w: &'a mut W,
}
impl<'a> WF39_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P36 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf36(&self) -> WF36_R {
        WF36_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P37 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf37(&self) -> WF37_R {
        WF37_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P38 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf38(&self) -> WF38_R {
        WF38_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P39 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf39(&self) -> WF39_R {
        WF39_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P36 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf36(&mut self) -> WF36_W {
        WF36_W { w: self }
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P37 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf37(&mut self) -> WF37_W {
        WF37_W { w: self }
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P38 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf38(&mut self) -> WF38_W {
        WF38_W { w: self }
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P39 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf39(&mut self) -> WF39_W {
        WF39_W { w: self }
    }
}
