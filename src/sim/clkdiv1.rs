#[doc = "Reader of register CLKDIV1"]
pub type R = crate::R<u32, super::CLKDIV1>;
#[doc = "Writer for register CLKDIV1"]
pub type W = crate::W<u32, super::CLKDIV1>;
#[doc = "Register CLKDIV1 `reset()`'s with value 0x0100_0000"]
impl crate::ResetValue for super::CLKDIV1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100_0000
    }
}
#[doc = "Flash Clock Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHCLKMODE_A {
    #[doc = "0: Flash Clock is the same as BUS clock."]
    _0 = 0,
    #[doc = "1: Flash Clock is a half of BUS clock."]
    _1 = 1,
}
impl From<FLASHCLKMODE_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHCLKMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASHCLKMODE`"]
pub type FLASHCLKMODE_R = crate::R<bool, FLASHCLKMODE_A>;
impl FLASHCLKMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHCLKMODE_A {
        match self.bits {
            false => FLASHCLKMODE_A::_0,
            true => FLASHCLKMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLASHCLKMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLASHCLKMODE_A::_1
    }
}
#[doc = "Write proxy for field `FLASHCLKMODE`"]
pub struct FLASHCLKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHCLKMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHCLKMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash Clock is the same as BUS clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLASHCLKMODE_A::_0)
    }
    #[doc = "Flash Clock is a half of BUS clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLASHCLKMODE_A::_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Bus Clock divider\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKDIVBUS_A {
    #[doc = "0: SYSCLK:BUSCLK = 1:1"]
    _00 = 0,
    #[doc = "1: SYSCLK:BUSCLK = 2:1"]
    _01 = 1,
    #[doc = "2: SYSCLK:BUSCLK = 3:1"]
    _10 = 2,
    #[doc = "3: SYSCLK:BUSCLK = 4:1"]
    _11 = 3,
}
impl From<CLKDIVBUS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKDIVBUS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKDIVBUS`"]
pub type CLKDIVBUS_R = crate::R<u8, CLKDIVBUS_A>;
impl CLKDIVBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKDIVBUS_A {
        match self.bits {
            0 => CLKDIVBUS_A::_00,
            1 => CLKDIVBUS_A::_01,
            2 => CLKDIVBUS_A::_10,
            3 => CLKDIVBUS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CLKDIVBUS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CLKDIVBUS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CLKDIVBUS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CLKDIVBUS_A::_11
    }
}
#[doc = "Write proxy for field `CLKDIVBUS`"]
pub struct CLKDIVBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIVBUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKDIVBUS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SYSCLK:BUSCLK = 1:1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLKDIVBUS_A::_00)
    }
    #[doc = "SYSCLK:BUSCLK = 2:1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLKDIVBUS_A::_01)
    }
    #[doc = "SYSCLK:BUSCLK = 3:1"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLKDIVBUS_A::_10)
    }
    #[doc = "SYSCLK:BUSCLK = 4:1"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLKDIVBUS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "System Clock divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKDIVSYS_A {
    #[doc = "0: Divide by 1"]
    _0000 = 0,
    #[doc = "1: Divide by 2"]
    _0001 = 1,
    #[doc = "2: Divide by 3"]
    _0010 = 2,
    #[doc = "3: Divide by 4"]
    _0011 = 3,
    #[doc = "4: Divide by 5"]
    _0100 = 4,
    #[doc = "5: Divide by 6"]
    _0101 = 5,
    #[doc = "6: Divide by 7"]
    _0110 = 6,
    #[doc = "7: Divide by 8"]
    _0111 = 7,
    #[doc = "8: Divide by 9"]
    _1000 = 8,
    #[doc = "9: Divide by 10"]
    _1001 = 9,
    #[doc = "10: Divide by 11"]
    _1010 = 10,
    #[doc = "11: Divide by 12"]
    _1011 = 11,
    #[doc = "12: Divide by 13"]
    _1100 = 12,
    #[doc = "13: Divide by 14"]
    _1101 = 13,
    #[doc = "14: Divide by 15"]
    _1110 = 14,
    #[doc = "15: Divide by 16"]
    _1111 = 15,
}
impl From<CLKDIVSYS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKDIVSYS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKDIVSYS`"]
pub type CLKDIVSYS_R = crate::R<u8, CLKDIVSYS_A>;
impl CLKDIVSYS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKDIVSYS_A {
        match self.bits {
            0 => CLKDIVSYS_A::_0000,
            1 => CLKDIVSYS_A::_0001,
            2 => CLKDIVSYS_A::_0010,
            3 => CLKDIVSYS_A::_0011,
            4 => CLKDIVSYS_A::_0100,
            5 => CLKDIVSYS_A::_0101,
            6 => CLKDIVSYS_A::_0110,
            7 => CLKDIVSYS_A::_0111,
            8 => CLKDIVSYS_A::_1000,
            9 => CLKDIVSYS_A::_1001,
            10 => CLKDIVSYS_A::_1010,
            11 => CLKDIVSYS_A::_1011,
            12 => CLKDIVSYS_A::_1100,
            13 => CLKDIVSYS_A::_1101,
            14 => CLKDIVSYS_A::_1110,
            15 => CLKDIVSYS_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == CLKDIVSYS_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == CLKDIVSYS_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == CLKDIVSYS_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == CLKDIVSYS_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == CLKDIVSYS_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == CLKDIVSYS_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == CLKDIVSYS_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == CLKDIVSYS_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == CLKDIVSYS_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == CLKDIVSYS_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == CLKDIVSYS_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == CLKDIVSYS_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == CLKDIVSYS_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == CLKDIVSYS_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == CLKDIVSYS_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == CLKDIVSYS_A::_1111
    }
}
#[doc = "Write proxy for field `CLKDIVSYS`"]
pub struct CLKDIVSYS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIVSYS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKDIVSYS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(CLKDIVSYS_A::_0000)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(CLKDIVSYS_A::_0001)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(CLKDIVSYS_A::_0010)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(CLKDIVSYS_A::_0011)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(CLKDIVSYS_A::_0100)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(CLKDIVSYS_A::_0101)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(CLKDIVSYS_A::_0110)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(CLKDIVSYS_A::_0111)
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(CLKDIVSYS_A::_1000)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(CLKDIVSYS_A::_1001)
    }
    #[doc = "Divide by 11"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(CLKDIVSYS_A::_1010)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(CLKDIVSYS_A::_1011)
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(CLKDIVSYS_A::_1100)
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(CLKDIVSYS_A::_1101)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(CLKDIVSYS_A::_1110)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(CLKDIVSYS_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Flash Clock Mode"]
    #[inline(always)]
    pub fn flashclkmode(&self) -> FLASHCLKMODE_R {
        FLASHCLKMODE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Bus Clock divider"]
    #[inline(always)]
    pub fn clkdivbus(&self) -> CLKDIVBUS_R {
        CLKDIVBUS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:31 - System Clock divider"]
    #[inline(always)]
    pub fn clkdivsys(&self) -> CLKDIVSYS_R {
        CLKDIVSYS_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Flash Clock Mode"]
    #[inline(always)]
    pub fn flashclkmode(&mut self) -> FLASHCLKMODE_W {
        FLASHCLKMODE_W { w: self }
    }
    #[doc = "Bits 24:25 - Bus Clock divider"]
    #[inline(always)]
    pub fn clkdivbus(&mut self) -> CLKDIVBUS_W {
        CLKDIVBUS_W { w: self }
    }
    #[doc = "Bits 28:31 - System Clock divider"]
    #[inline(always)]
    pub fn clkdivsys(&mut self) -> CLKDIVSYS_W {
        CLKDIVSYS_W { w: self }
    }
}
