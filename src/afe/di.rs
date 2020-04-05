#[doc = "Reader of register DI"]
pub type R = crate::R<u32, super::DI>;
#[doc = "Writer for register DI"]
pub type W = crate::W<u32, super::DI>;
#[doc = "Register DI `reset()`'s with value 0"]
impl crate::ResetValue for super::DI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTEN3`"]
pub type INTEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN3`"]
pub struct INTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `INTEN2`"]
pub type INTEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN2`"]
pub struct INTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `INTEN1`"]
pub type INTEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN1`"]
pub struct INTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `INTEN0`"]
pub type INTEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTEN0`"]
pub struct INTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `DMAEN3`"]
pub type DMAEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEN3`"]
pub struct DMAEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `DMAEN2`"]
pub type DMAEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEN2`"]
pub struct DMAEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `DMAEN1`"]
pub type DMAEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEN1`"]
pub struct DMAEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DMAEN0`"]
pub type DMAEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEN0`"]
pub struct DMAEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 23 - Interrupt Enable 3"]
    #[inline(always)]
    pub fn inten3(&self) -> INTEN3_R {
        INTEN3_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Interrupt Enable 2"]
    #[inline(always)]
    pub fn inten2(&self) -> INTEN2_R {
        INTEN2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Interrupt Enable 1"]
    #[inline(always)]
    pub fn inten1(&self) -> INTEN1_R {
        INTEN1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Interrupt Enable 0"]
    #[inline(always)]
    pub fn inten0(&self) -> INTEN0_R {
        INTEN0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DMA Enable3"]
    #[inline(always)]
    pub fn dmaen3(&self) -> DMAEN3_R {
        DMAEN3_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DMA Enable2"]
    #[inline(always)]
    pub fn dmaen2(&self) -> DMAEN2_R {
        DMAEN2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DMA Enable1"]
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DMA Enable0"]
    #[inline(always)]
    pub fn dmaen0(&self) -> DMAEN0_R {
        DMAEN0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Interrupt Enable 3"]
    #[inline(always)]
    pub fn inten3(&mut self) -> INTEN3_W {
        INTEN3_W { w: self }
    }
    #[doc = "Bit 24 - Interrupt Enable 2"]
    #[inline(always)]
    pub fn inten2(&mut self) -> INTEN2_W {
        INTEN2_W { w: self }
    }
    #[doc = "Bit 25 - Interrupt Enable 1"]
    #[inline(always)]
    pub fn inten1(&mut self) -> INTEN1_W {
        INTEN1_W { w: self }
    }
    #[doc = "Bit 26 - Interrupt Enable 0"]
    #[inline(always)]
    pub fn inten0(&mut self) -> INTEN0_W {
        INTEN0_W { w: self }
    }
    #[doc = "Bit 28 - DMA Enable3"]
    #[inline(always)]
    pub fn dmaen3(&mut self) -> DMAEN3_W {
        DMAEN3_W { w: self }
    }
    #[doc = "Bit 29 - DMA Enable2"]
    #[inline(always)]
    pub fn dmaen2(&mut self) -> DMAEN2_W {
        DMAEN2_W { w: self }
    }
    #[doc = "Bit 30 - DMA Enable1"]
    #[inline(always)]
    pub fn dmaen1(&mut self) -> DMAEN1_W {
        DMAEN1_W { w: self }
    }
    #[doc = "Bit 31 - DMA Enable0"]
    #[inline(always)]
    pub fn dmaen0(&mut self) -> DMAEN0_W {
        DMAEN0_W { w: self }
    }
}
