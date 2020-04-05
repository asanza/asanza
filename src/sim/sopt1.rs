#[doc = "Reader of register SOPT1"]
pub type R = crate::R<u32, super::SOPT1>;
#[doc = "Writer for register SOPT1"]
pub type W = crate::W<u32, super::SOPT1>;
#[doc = "Register SOPT1 `reset()`'s with value 0x6000"]
impl crate::ResetValue for super::SOPT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x6000
    }
}
#[doc = "Returns the size of the system RAM\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRAMSIZE_A {
    #[doc = "6: 32 KB System RAM"]
    _0110 = 6,
}
impl From<SRAMSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRAMSIZE`"]
pub type SRAMSIZE_R = crate::R<u8, SRAMSIZE_A>;
impl SRAMSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRAMSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            6 => Val(SRAMSIZE_A::_0110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == SRAMSIZE_A::_0110
    }
}
#[doc = "32K oscillator clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSC32KSEL_A {
    #[doc = "0: OSC32KCLK (RTC Oscillator output)"]
    _00 = 0,
    #[doc = "1: ERCLK32K"]
    _01 = 1,
    #[doc = "2: MCGIRCLK"]
    _10 = 2,
    #[doc = "3: LPO"]
    _11 = 3,
}
impl From<OSC32KSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSC32KSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OSC32KSEL`"]
pub type OSC32KSEL_R = crate::R<u8, OSC32KSEL_A>;
impl OSC32KSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSC32KSEL_A {
        match self.bits {
            0 => OSC32KSEL_A::_00,
            1 => OSC32KSEL_A::_01,
            2 => OSC32KSEL_A::_10,
            3 => OSC32KSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OSC32KSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OSC32KSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OSC32KSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OSC32KSEL_A::_11
    }
}
#[doc = "Write proxy for field `OSC32KSEL`"]
pub struct OSC32KSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC32KSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSC32KSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "OSC32KCLK (RTC Oscillator output)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OSC32KSEL_A::_00)
    }
    #[doc = "ERCLK32K"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OSC32KSEL_A::_01)
    }
    #[doc = "MCGIRCLK"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OSC32KSEL_A::_10)
    }
    #[doc = "LPO"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OSC32KSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - Returns the size of the system RAM"]
    #[inline(always)]
    pub fn sramsize(&self) -> SRAMSIZE_R {
        SRAMSIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:19 - 32K oscillator clock select"]
    #[inline(always)]
    pub fn osc32ksel(&self) -> OSC32KSEL_R {
        OSC32KSEL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - 32K oscillator clock select"]
    #[inline(always)]
    pub fn osc32ksel(&mut self) -> OSC32KSEL_W {
        OSC32KSEL_W { w: self }
    }
}
