#[doc = "Reader of register WF47TO44"]
pub type R = crate::R<u32, super::WF47TO44>;
#[doc = "Writer for register WF47TO44"]
pub type W = crate::W<u32, super::WF47TO44>;
#[doc = "Register WF47TO44 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF47TO44 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WF44`"]
pub type WF44_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF44`"]
pub struct WF44_W<'a> {
    w: &'a mut W,
}
impl<'a> WF44_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WF45`"]
pub type WF45_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF45`"]
pub struct WF45_W<'a> {
    w: &'a mut W,
}
impl<'a> WF45_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WF46`"]
pub type WF46_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF46`"]
pub struct WF46_W<'a> {
    w: &'a mut W,
}
impl<'a> WF46_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WF47`"]
pub type WF47_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF47`"]
pub struct WF47_W<'a> {
    w: &'a mut W,
}
impl<'a> WF47_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P44 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf44(&self) -> WF44_R {
        WF44_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P45 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf45(&self) -> WF45_R {
        WF45_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P46 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf46(&self) -> WF46_R {
        WF46_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P47 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf47(&self) -> WF47_R {
        WF47_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P44 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf44(&mut self) -> WF44_W {
        WF44_W { w: self }
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P45 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf45(&mut self) -> WF45_W {
        WF45_W { w: self }
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P46 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf46(&mut self) -> WF46_W {
        WF46_W { w: self }
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P47 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf47(&mut self) -> WF47_W {
        WF47_W { w: self }
    }
}
