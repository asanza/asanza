#[doc = "Reader of register WF59TO56"]
pub type R = crate::R<u32, super::WF59TO56>;
#[doc = "Writer for register WF59TO56"]
pub type W = crate::W<u32, super::WF59TO56>;
#[doc = "Register WF59TO56 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF59TO56 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WF56`"]
pub type WF56_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF56`"]
pub struct WF56_W<'a> {
    w: &'a mut W,
}
impl<'a> WF56_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WF57`"]
pub type WF57_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF57`"]
pub struct WF57_W<'a> {
    w: &'a mut W,
}
impl<'a> WF57_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WF58`"]
pub type WF58_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF58`"]
pub struct WF58_W<'a> {
    w: &'a mut W,
}
impl<'a> WF58_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WF59`"]
pub type WF59_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF59`"]
pub struct WF59_W<'a> {
    w: &'a mut W,
}
impl<'a> WF59_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P56 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf56(&self) -> WF56_R {
        WF56_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P57 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf57(&self) -> WF57_R {
        WF57_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P58 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf58(&self) -> WF58_R {
        WF58_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P59 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf59(&self) -> WF59_R {
        WF59_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P56 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf56(&mut self) -> WF56_W {
        WF56_W { w: self }
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P57 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf57(&mut self) -> WF57_W {
        WF57_W { w: self }
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P58 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf58(&mut self) -> WF58_W {
        WF58_W { w: self }
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P59 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf59(&mut self) -> WF59_W {
        WF59_W { w: self }
    }
}
