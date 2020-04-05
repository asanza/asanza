#[doc = "Reader of register WF3TO0"]
pub type R = crate::R<u32, super::WF3TO0>;
#[doc = "Writer for register WF3TO0"]
pub type W = crate::W<u32, super::WF3TO0>;
#[doc = "Register WF3TO0 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF3TO0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WF0`"]
pub type WF0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF0`"]
pub struct WF0_W<'a> {
    w: &'a mut W,
}
impl<'a> WF0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WF1`"]
pub type WF1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF1`"]
pub struct WF1_W<'a> {
    w: &'a mut W,
}
impl<'a> WF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WF2`"]
pub type WF2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF2`"]
pub struct WF2_W<'a> {
    w: &'a mut W,
}
impl<'a> WF2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WF3`"]
pub type WF3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF3`"]
pub struct WF3_W<'a> {
    w: &'a mut W,
}
impl<'a> WF3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P0 as described above for WF3."]
    #[inline(always)]
    pub fn wf0(&self) -> WF0_R {
        WF0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P1 as described above for WF3."]
    #[inline(always)]
    pub fn wf1(&self) -> WF1_R {
        WF1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P2 as described above for WF3."]
    #[inline(always)]
    pub fn wf2(&self) -> WF2_R {
        WF2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Segment-on front plane operation - Each bit turns on or off the segments associated with LCD_P3 in the following pattern: HGFEDCBA (most significant bit controls segment H and least significant bit controls segment A)"]
    #[inline(always)]
    pub fn wf3(&self) -> WF3_R {
        WF3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P0 as described above for WF3."]
    #[inline(always)]
    pub fn wf0(&mut self) -> WF0_W {
        WF0_W { w: self }
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P1 as described above for WF3."]
    #[inline(always)]
    pub fn wf1(&mut self) -> WF1_W {
        WF1_W { w: self }
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P2 as described above for WF3."]
    #[inline(always)]
    pub fn wf2(&mut self) -> WF2_W {
        WF2_W { w: self }
    }
    #[doc = "Bits 24:31 - Segment-on front plane operation - Each bit turns on or off the segments associated with LCD_P3 in the following pattern: HGFEDCBA (most significant bit controls segment H and least significant bit controls segment A)"]
    #[inline(always)]
    pub fn wf3(&mut self) -> WF3_W {
        WF3_W { w: self }
    }
}
