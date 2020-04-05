#[doc = "Reader of register WF35TO32"]
pub type R = crate::R<u32, super::WF35TO32>;
#[doc = "Writer for register WF35TO32"]
pub type W = crate::W<u32, super::WF35TO32>;
#[doc = "Register WF35TO32 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF35TO32 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WF32`"]
pub type WF32_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF32`"]
pub struct WF32_W<'a> {
    w: &'a mut W,
}
impl<'a> WF32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WF33`"]
pub type WF33_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF33`"]
pub struct WF33_W<'a> {
    w: &'a mut W,
}
impl<'a> WF33_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WF34`"]
pub type WF34_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF34`"]
pub struct WF34_W<'a> {
    w: &'a mut W,
}
impl<'a> WF34_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WF35`"]
pub type WF35_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF35`"]
pub struct WF35_W<'a> {
    w: &'a mut W,
}
impl<'a> WF35_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P32 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf32(&self) -> WF32_R {
        WF32_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P33 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf33(&self) -> WF33_R {
        WF33_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P34 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf34(&self) -> WF34_R {
        WF34_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P35 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf35(&self) -> WF35_R {
        WF35_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P32 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf32(&mut self) -> WF32_W {
        WF32_W { w: self }
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P33 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf33(&mut self) -> WF33_W {
        WF33_W { w: self }
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P34 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf34(&mut self) -> WF34_W {
        WF34_W { w: self }
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P35 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf35(&mut self) -> WF35_W {
        WF35_W { w: self }
    }
}
