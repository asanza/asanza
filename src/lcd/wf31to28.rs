#[doc = "Reader of register WF31TO28"]
pub type R = crate::R<u32, super::WF31TO28>;
#[doc = "Writer for register WF31TO28"]
pub type W = crate::W<u32, super::WF31TO28>;
#[doc = "Register WF31TO28 `reset()`'s with value 0"]
impl crate::ResetValue for super::WF31TO28 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WF28`"]
pub type WF28_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF28`"]
pub struct WF28_W<'a> {
    w: &'a mut W,
}
impl<'a> WF28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WF29`"]
pub type WF29_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF29`"]
pub struct WF29_W<'a> {
    w: &'a mut W,
}
impl<'a> WF29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WF30`"]
pub type WF30_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF30`"]
pub struct WF30_W<'a> {
    w: &'a mut W,
}
impl<'a> WF30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WF31`"]
pub type WF31_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WF31`"]
pub struct WF31_W<'a> {
    w: &'a mut W,
}
impl<'a> WF31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P28 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf28(&self) -> WF28_R {
        WF28_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P29 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf29(&self) -> WF29_R {
        WF29_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P30 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf30(&self) -> WF30_R {
        WF30_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P31 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf31(&self) -> WF31_R {
        WF31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P28 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf28(&mut self) -> WF28_W {
        WF28_W { w: self }
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P29 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf29(&mut self) -> WF29_W {
        WF29_W { w: self }
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P30 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf30(&mut self) -> WF30_W {
        WF30_W { w: self }
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P31 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf31(&mut self) -> WF31_W {
        WF31_W { w: self }
    }
}
