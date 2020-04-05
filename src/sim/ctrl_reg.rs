#[doc = "Reader of register CTRL_REG"]
pub type R = crate::R<u32, super::CTRL_REG>;
#[doc = "Writer for register CTRL_REG"]
pub type W = crate::W<u32, super::CTRL_REG>;
#[doc = "Register CTRL_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "NMI Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIDIS_A {
    #[doc = "0: NMI enabled"]
    _0 = 0,
    #[doc = "1: NMI disabled"]
    _1 = 1,
}
impl From<NMIDIS_A> for bool {
    #[inline(always)]
    fn from(variant: NMIDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NMIDIS`"]
pub type NMIDIS_R = crate::R<bool, NMIDIS_A>;
impl NMIDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIDIS_A {
        match self.bits {
            false => NMIDIS_A::_0,
            true => NMIDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NMIDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NMIDIS_A::_1
    }
}
#[doc = "Write proxy for field `NMIDIS`"]
pub struct NMIDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMIDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "NMI enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NMIDIS_A::_0)
    }
    #[doc = "NMI disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NMIDIS_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `PLLVLPEN`"]
pub type PLLVLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLVLPEN`"]
pub struct PLLVLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLVLPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "SAR ADC Trigger Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCTRGSEL_A {
    #[doc = "0: Bus ClockDuring Low Power Modes such as stop, the Bus clock is not available for conversion and should not be selected in case a conversion needs to be performed while in stop."]
    _00 = 0,
    #[doc = "1: ADC asynchronous Clock"]
    _01 = 1,
    #[doc = "2: ERCLK32K"]
    _10 = 2,
    #[doc = "3: OSCCLK"]
    _11 = 3,
}
impl From<ADCTRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCTRGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCTRGSEL`"]
pub type ADCTRGSEL_R = crate::R<u8, ADCTRGSEL_A>;
impl ADCTRGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCTRGSEL_A {
        match self.bits {
            0 => ADCTRGSEL_A::_00,
            1 => ADCTRGSEL_A::_01,
            2 => ADCTRGSEL_A::_10,
            3 => ADCTRGSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADCTRGSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ADCTRGSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ADCTRGSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADCTRGSEL_A::_11
    }
}
#[doc = "Write proxy for field `ADCTRGSEL`"]
pub struct ADCTRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCTRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCTRGSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bus ClockDuring Low Power Modes such as stop, the Bus clock is not available for conversion and should not be selected in case a conversion needs to be performed while in stop."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADCTRGSEL_A::_00)
    }
    #[doc = "ADC asynchronous Clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADCTRGSEL_A::_01)
    }
    #[doc = "ERCLK32K"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADCTRGSEL_A::_10)
    }
    #[doc = "OSCCLK"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADCTRGSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Clock out Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUT_A {
    #[doc = "0: Disabled"]
    _000 = 0,
    #[doc = "1: Gated Core Clk"]
    _001 = 1,
    #[doc = "2: Bus Clk"]
    _010 = 2,
    #[doc = "3: LPO clock from PMC"]
    _011 = 3,
    #[doc = "4: IRC clock from MCG"]
    _100 = 4,
    #[doc = "5: Muxed 32Khz source (please refer to SOPT1\\[19:18\\]
for possible options)"]
    _101 = 5,
    #[doc = "6: MHz Oscillator external reference clock"]
    _110 = 6,
    #[doc = "7: PLL clock output from MCG"]
    _111 = 7,
}
impl From<CLKOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKOUT`"]
pub type CLKOUT_R = crate::R<u8, CLKOUT_A>;
impl CLKOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKOUT_A {
        match self.bits {
            0 => CLKOUT_A::_000,
            1 => CLKOUT_A::_001,
            2 => CLKOUT_A::_010,
            3 => CLKOUT_A::_011,
            4 => CLKOUT_A::_100,
            5 => CLKOUT_A::_101,
            6 => CLKOUT_A::_110,
            7 => CLKOUT_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CLKOUT_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CLKOUT_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CLKOUT_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CLKOUT_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CLKOUT_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CLKOUT_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CLKOUT_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CLKOUT_A::_111
    }
}
#[doc = "Write proxy for field `CLKOUT`"]
pub struct CLKOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CLKOUT_A::_000)
    }
    #[doc = "Gated Core Clk"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CLKOUT_A::_001)
    }
    #[doc = "Bus Clk"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLKOUT_A::_010)
    }
    #[doc = "LPO clock from PMC"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLKOUT_A::_011)
    }
    #[doc = "IRC clock from MCG"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUT_A::_100)
    }
    #[doc = "Muxed 32Khz source (please refer to SOPT1\\[19:18\\]
for possible options)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CLKOUT_A::_101)
    }
    #[doc = "MHz Oscillator external reference clock"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLKOUT_A::_110)
    }
    #[doc = "PLL clock output from MCG"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CLKOUT_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "This bit inverts the SPI0 signal output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_INV0_A {
    #[doc = "0: not inverted"]
    _0 = 0,
    #[doc = "1: inverts SS"]
    _1 = 1,
}
impl From<SPI0_INV0_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_INV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0_INV0`"]
pub type SPI0_INV0_R = crate::R<bool, SPI0_INV0_A>;
impl SPI0_INV0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_INV0_A {
        match self.bits {
            false => SPI0_INV0_A::_0,
            true => SPI0_INV0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI0_INV0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI0_INV0_A::_1
    }
}
#[doc = "Write proxy for field `SPI0_INV0`"]
pub struct SPI0_INV0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_INV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_INV0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI0_INV0_A::_0)
    }
    #[doc = "inverts SS"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI0_INV0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "This bit inverts the SPI0 signal output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_INV1_A {
    #[doc = "0: not inverted"]
    _0 = 0,
    #[doc = "1: inverts SCK"]
    _1 = 1,
}
impl From<SPI0_INV1_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_INV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0_INV1`"]
pub type SPI0_INV1_R = crate::R<bool, SPI0_INV1_A>;
impl SPI0_INV1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_INV1_A {
        match self.bits {
            false => SPI0_INV1_A::_0,
            true => SPI0_INV1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI0_INV1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI0_INV1_A::_1
    }
}
#[doc = "Write proxy for field `SPI0_INV1`"]
pub struct SPI0_INV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_INV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_INV1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI0_INV1_A::_0)
    }
    #[doc = "inverts SCK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI0_INV1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "This bit inverts the SPI0 signal output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_INV2_A {
    #[doc = "0: not inverted"]
    _0 = 0,
    #[doc = "1: inverts MOSI"]
    _1 = 1,
}
impl From<SPI0_INV2_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_INV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0_INV2`"]
pub type SPI0_INV2_R = crate::R<bool, SPI0_INV2_A>;
impl SPI0_INV2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_INV2_A {
        match self.bits {
            false => SPI0_INV2_A::_0,
            true => SPI0_INV2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI0_INV2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI0_INV2_A::_1
    }
}
#[doc = "Write proxy for field `SPI0_INV2`"]
pub struct SPI0_INV2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_INV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_INV2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI0_INV2_A::_0)
    }
    #[doc = "inverts MOSI"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI0_INV2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "This bit inverts the SPI0 signal output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_INV3_A {
    #[doc = "0: not inverted"]
    _0 = 0,
    #[doc = "1: inverts MISO"]
    _1 = 1,
}
impl From<SPI0_INV3_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_INV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI0_INV3`"]
pub type SPI0_INV3_R = crate::R<bool, SPI0_INV3_A>;
impl SPI0_INV3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_INV3_A {
        match self.bits {
            false => SPI0_INV3_A::_0,
            true => SPI0_INV3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI0_INV3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI0_INV3_A::_1
    }
}
#[doc = "Write proxy for field `SPI0_INV3`"]
pub struct SPI0_INV3_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_INV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_INV3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI0_INV3_A::_0)
    }
    #[doc = "inverts MISO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI0_INV3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "This bit inverts the SPI1 signal output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_INV0_A {
    #[doc = "0: not inverted"]
    _0 = 0,
    #[doc = "1: inverts SS"]
    _1 = 1,
}
impl From<SPI1_INV0_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_INV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI1_INV0`"]
pub type SPI1_INV0_R = crate::R<bool, SPI1_INV0_A>;
impl SPI1_INV0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_INV0_A {
        match self.bits {
            false => SPI1_INV0_A::_0,
            true => SPI1_INV0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI1_INV0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI1_INV0_A::_1
    }
}
#[doc = "Write proxy for field `SPI1_INV0`"]
pub struct SPI1_INV0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_INV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_INV0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI1_INV0_A::_0)
    }
    #[doc = "inverts SS"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI1_INV0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "This bit inverts the SPI1 signal output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_INV1_A {
    #[doc = "0: not inverted"]
    _0 = 0,
    #[doc = "1: inverts SCK"]
    _1 = 1,
}
impl From<SPI1_INV1_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_INV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI1_INV1`"]
pub type SPI1_INV1_R = crate::R<bool, SPI1_INV1_A>;
impl SPI1_INV1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_INV1_A {
        match self.bits {
            false => SPI1_INV1_A::_0,
            true => SPI1_INV1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI1_INV1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI1_INV1_A::_1
    }
}
#[doc = "Write proxy for field `SPI1_INV1`"]
pub struct SPI1_INV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_INV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_INV1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI1_INV1_A::_0)
    }
    #[doc = "inverts SCK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI1_INV1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "This bit inverts the SPI1 signal output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_INV2_A {
    #[doc = "0: not inverted"]
    _0 = 0,
    #[doc = "1: inverts MOSI"]
    _1 = 1,
}
impl From<SPI1_INV2_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_INV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI1_INV2`"]
pub type SPI1_INV2_R = crate::R<bool, SPI1_INV2_A>;
impl SPI1_INV2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_INV2_A {
        match self.bits {
            false => SPI1_INV2_A::_0,
            true => SPI1_INV2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI1_INV2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI1_INV2_A::_1
    }
}
#[doc = "Write proxy for field `SPI1_INV2`"]
pub struct SPI1_INV2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_INV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_INV2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI1_INV2_A::_0)
    }
    #[doc = "inverts MOSI"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI1_INV2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "This bit inverts the SPI1 signal output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_INV3_A {
    #[doc = "0: not inverted"]
    _0 = 0,
    #[doc = "1: inverts MISO"]
    _1 = 1,
}
impl From<SPI1_INV3_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_INV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI1_INV3`"]
pub type SPI1_INV3_R = crate::R<bool, SPI1_INV3_A>;
impl SPI1_INV3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_INV3_A {
        match self.bits {
            false => SPI1_INV3_A::_0,
            true => SPI1_INV3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI1_INV3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI1_INV3_A::_1
    }
}
#[doc = "Write proxy for field `SPI1_INV3`"]
pub struct SPI1_INV3_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_INV3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_INV3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "not inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI1_INV3_A::_0)
    }
    #[doc = "inverts MISO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI1_INV3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "PLL/FLL selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLFLLSEL_A {
    #[doc = "0: MCGFLLCLK"]
    _00 = 0,
    #[doc = "1: MCGPLLCLK"]
    _01 = 1,
    #[doc = "2: BUSCLK"]
    _10 = 2,
    #[doc = "3: OSC32KCLK (RTC Oscillator output)"]
    _11 = 3,
}
impl From<PLLFLLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLFLLSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLLFLLSEL`"]
pub type PLLFLLSEL_R = crate::R<u8, PLLFLLSEL_A>;
impl PLLFLLSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLFLLSEL_A {
        match self.bits {
            0 => PLLFLLSEL_A::_00,
            1 => PLLFLLSEL_A::_01,
            2 => PLLFLLSEL_A::_10,
            3 => PLLFLLSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PLLFLLSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PLLFLLSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PLLFLLSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PLLFLLSEL_A::_11
    }
}
#[doc = "Write proxy for field `PLLFLLSEL`"]
pub struct PLLFLLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLFLLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLFLLSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "MCGFLLCLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_00)
    }
    #[doc = "MCGPLLCLK"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_01)
    }
    #[doc = "BUSCLK"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_10)
    }
    #[doc = "OSC32KCLK (RTC Oscillator output)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "XBAR clock out selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XBARCLKOUT_A {
    #[doc = "0: Disabled"]
    _000 = 0,
    #[doc = "1: Gated Core Clk"]
    _001 = 1,
    #[doc = "2: Bus Clk"]
    _010 = 2,
    #[doc = "3: LPO clock from PMC"]
    _011 = 3,
    #[doc = "4: IRC clock from MCG"]
    _100 = 4,
    #[doc = "5: MUXed 32 kHz source (please refer to SOPT1\\[19:18\\]
for possible options)"]
    _101 = 5,
    #[doc = "6: MHz Oscillator external reference clock"]
    _110 = 6,
    #[doc = "7: PLL clock output from MCG"]
    _111 = 7,
}
impl From<XBARCLKOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: XBARCLKOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XBARCLKOUT`"]
pub type XBARCLKOUT_R = crate::R<u8, XBARCLKOUT_A>;
impl XBARCLKOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XBARCLKOUT_A {
        match self.bits {
            0 => XBARCLKOUT_A::_000,
            1 => XBARCLKOUT_A::_001,
            2 => XBARCLKOUT_A::_010,
            3 => XBARCLKOUT_A::_011,
            4 => XBARCLKOUT_A::_100,
            5 => XBARCLKOUT_A::_101,
            6 => XBARCLKOUT_A::_110,
            7 => XBARCLKOUT_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == XBARCLKOUT_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == XBARCLKOUT_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == XBARCLKOUT_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == XBARCLKOUT_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == XBARCLKOUT_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == XBARCLKOUT_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == XBARCLKOUT_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == XBARCLKOUT_A::_111
    }
}
#[doc = "Write proxy for field `XBARCLKOUT`"]
pub struct XBARCLKOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> XBARCLKOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XBARCLKOUT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(XBARCLKOUT_A::_000)
    }
    #[doc = "Gated Core Clk"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(XBARCLKOUT_A::_001)
    }
    #[doc = "Bus Clk"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(XBARCLKOUT_A::_010)
    }
    #[doc = "LPO clock from PMC"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(XBARCLKOUT_A::_011)
    }
    #[doc = "IRC clock from MCG"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(XBARCLKOUT_A::_100)
    }
    #[doc = "MUXed 32 kHz source (please refer to SOPT1\\[19:18\\]
for possible options)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(XBARCLKOUT_A::_101)
    }
    #[doc = "MHz Oscillator external reference clock"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(XBARCLKOUT_A::_110)
    }
    #[doc = "PLL clock output from MCG"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(XBARCLKOUT_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "AFE clock output select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFEOUTCLKSEL_A {
    #[doc = "0: AFE output clock is divided by AFE clock prescaler."]
    _0 = 0,
    #[doc = "1: AFE output clock is NOT divided by AFE clock prescaler."]
    _1 = 1,
}
impl From<AFEOUTCLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: AFEOUTCLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AFEOUTCLKSEL`"]
pub type AFEOUTCLKSEL_R = crate::R<bool, AFEOUTCLKSEL_A>;
impl AFEOUTCLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFEOUTCLKSEL_A {
        match self.bits {
            false => AFEOUTCLKSEL_A::_0,
            true => AFEOUTCLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AFEOUTCLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AFEOUTCLKSEL_A::_1
    }
}
#[doc = "Write proxy for field `AFEOUTCLKSEL`"]
pub struct AFEOUTCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> AFEOUTCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFEOUTCLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AFE output clock is divided by AFE clock prescaler."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AFEOUTCLKSEL_A::_0)
    }
    #[doc = "AFE output clock is NOT divided by AFE clock prescaler."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AFEOUTCLKSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "LPUART clock Source configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPUARTSRC_A {
    #[doc = "0: Clock disabled"]
    _00 = 0,
    #[doc = "1: MCGPLLCLK/MCGFLLCLK"]
    _01 = 1,
    #[doc = "2: OSCERCLK"]
    _10 = 2,
    #[doc = "3: MCGIRCLK"]
    _11 = 3,
}
impl From<LPUARTSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUARTSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPUARTSRC`"]
pub type LPUARTSRC_R = crate::R<u8, LPUARTSRC_A>;
impl LPUARTSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUARTSRC_A {
        match self.bits {
            0 => LPUARTSRC_A::_00,
            1 => LPUARTSRC_A::_01,
            2 => LPUARTSRC_A::_10,
            3 => LPUARTSRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPUARTSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPUARTSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LPUARTSRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == LPUARTSRC_A::_11
    }
}
#[doc = "Write proxy for field `LPUARTSRC`"]
pub struct LPUARTSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUARTSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUARTSRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUARTSRC_A::_00)
    }
    #[doc = "MCGPLLCLK/MCGFLLCLK"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUARTSRC_A::_01)
    }
    #[doc = "OSCERCLK"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPUARTSRC_A::_10)
    }
    #[doc = "MCGIRCLK"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(LPUARTSRC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "QTMR counters Freeze control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRFREEZE_A {
    #[doc = "0: QTMR counters operate normally."]
    _0 = 0,
    #[doc = "1: QTMR counters and OFLAGs are reset. Clearing this bit will resume QTMR operation."]
    _1 = 1,
}
impl From<TMRFREEZE_A> for bool {
    #[inline(always)]
    fn from(variant: TMRFREEZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRFREEZE`"]
pub type TMRFREEZE_R = crate::R<bool, TMRFREEZE_A>;
impl TMRFREEZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRFREEZE_A {
        match self.bits {
            false => TMRFREEZE_A::_0,
            true => TMRFREEZE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMRFREEZE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMRFREEZE_A::_1
    }
}
#[doc = "Write proxy for field `TMRFREEZE`"]
pub struct TMRFREEZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRFREEZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRFREEZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "QTMR counters operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMRFREEZE_A::_0)
    }
    #[doc = "QTMR counters and OFLAGs are reset. Clearing this bit will resume QTMR operation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMRFREEZE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - NMI Disable"]
    #[inline(always)]
    pub fn nmidis(&self) -> NMIDIS_R {
        NMIDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PLL VLP Enable"]
    #[inline(always)]
    pub fn pllvlpen(&self) -> PLLVLPEN_R {
        PLLVLPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - SAR ADC Trigger Clock Select"]
    #[inline(always)]
    pub fn adctrgsel(&self) -> ADCTRGSEL_R {
        ADCTRGSEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - Clock out Select"]
    #[inline(always)]
    pub fn clkout(&self) -> CLKOUT_R {
        CLKOUT_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - This bit inverts the SPI0 signal output."]
    #[inline(always)]
    pub fn spi0_inv0(&self) -> SPI0_INV0_R {
        SPI0_INV0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit inverts the SPI0 signal output."]
    #[inline(always)]
    pub fn spi0_inv1(&self) -> SPI0_INV1_R {
        SPI0_INV1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This bit inverts the SPI0 signal output."]
    #[inline(always)]
    pub fn spi0_inv2(&self) -> SPI0_INV2_R {
        SPI0_INV2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This bit inverts the SPI0 signal output."]
    #[inline(always)]
    pub fn spi0_inv3(&self) -> SPI0_INV3_R {
        SPI0_INV3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This bit inverts the SPI1 signal output."]
    #[inline(always)]
    pub fn spi1_inv0(&self) -> SPI1_INV0_R {
        SPI1_INV0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This bit inverts the SPI1 signal output."]
    #[inline(always)]
    pub fn spi1_inv1(&self) -> SPI1_INV1_R {
        SPI1_INV1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This bit inverts the SPI1 signal output."]
    #[inline(always)]
    pub fn spi1_inv2(&self) -> SPI1_INV2_R {
        SPI1_INV2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This bit inverts the SPI1 signal output."]
    #[inline(always)]
    pub fn spi1_inv3(&self) -> SPI1_INV3_R {
        SPI1_INV3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - PLL/FLL selection"]
    #[inline(always)]
    pub fn pllfllsel(&self) -> PLLFLLSEL_R {
        PLLFLLSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 21:23 - XBAR clock out selection"]
    #[inline(always)]
    pub fn xbarclkout(&self) -> XBARCLKOUT_R {
        XBARCLKOUT_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bit 24 - AFE clock output select"]
    #[inline(always)]
    pub fn afeoutclksel(&self) -> AFEOUTCLKSEL_R {
        AFEOUTCLKSEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - LPUART clock Source configuration"]
    #[inline(always)]
    pub fn lpuartsrc(&self) -> LPUARTSRC_R {
        LPUARTSRC_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 31 - QTMR counters Freeze control"]
    #[inline(always)]
    pub fn tmrfreeze(&self) -> TMRFREEZE_R {
        TMRFREEZE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMI Disable"]
    #[inline(always)]
    pub fn nmidis(&mut self) -> NMIDIS_W {
        NMIDIS_W { w: self }
    }
    #[doc = "Bit 1 - PLL VLP Enable"]
    #[inline(always)]
    pub fn pllvlpen(&mut self) -> PLLVLPEN_W {
        PLLVLPEN_W { w: self }
    }
    #[doc = "Bits 3:4 - SAR ADC Trigger Clock Select"]
    #[inline(always)]
    pub fn adctrgsel(&mut self) -> ADCTRGSEL_W {
        ADCTRGSEL_W { w: self }
    }
    #[doc = "Bits 5:7 - Clock out Select"]
    #[inline(always)]
    pub fn clkout(&mut self) -> CLKOUT_W {
        CLKOUT_W { w: self }
    }
    #[doc = "Bit 8 - This bit inverts the SPI0 signal output."]
    #[inline(always)]
    pub fn spi0_inv0(&mut self) -> SPI0_INV0_W {
        SPI0_INV0_W { w: self }
    }
    #[doc = "Bit 9 - This bit inverts the SPI0 signal output."]
    #[inline(always)]
    pub fn spi0_inv1(&mut self) -> SPI0_INV1_W {
        SPI0_INV1_W { w: self }
    }
    #[doc = "Bit 10 - This bit inverts the SPI0 signal output."]
    #[inline(always)]
    pub fn spi0_inv2(&mut self) -> SPI0_INV2_W {
        SPI0_INV2_W { w: self }
    }
    #[doc = "Bit 11 - This bit inverts the SPI0 signal output."]
    #[inline(always)]
    pub fn spi0_inv3(&mut self) -> SPI0_INV3_W {
        SPI0_INV3_W { w: self }
    }
    #[doc = "Bit 12 - This bit inverts the SPI1 signal output."]
    #[inline(always)]
    pub fn spi1_inv0(&mut self) -> SPI1_INV0_W {
        SPI1_INV0_W { w: self }
    }
    #[doc = "Bit 13 - This bit inverts the SPI1 signal output."]
    #[inline(always)]
    pub fn spi1_inv1(&mut self) -> SPI1_INV1_W {
        SPI1_INV1_W { w: self }
    }
    #[doc = "Bit 14 - This bit inverts the SPI1 signal output."]
    #[inline(always)]
    pub fn spi1_inv2(&mut self) -> SPI1_INV2_W {
        SPI1_INV2_W { w: self }
    }
    #[doc = "Bit 15 - This bit inverts the SPI1 signal output."]
    #[inline(always)]
    pub fn spi1_inv3(&mut self) -> SPI1_INV3_W {
        SPI1_INV3_W { w: self }
    }
    #[doc = "Bits 16:17 - PLL/FLL selection"]
    #[inline(always)]
    pub fn pllfllsel(&mut self) -> PLLFLLSEL_W {
        PLLFLLSEL_W { w: self }
    }
    #[doc = "Bits 21:23 - XBAR clock out selection"]
    #[inline(always)]
    pub fn xbarclkout(&mut self) -> XBARCLKOUT_W {
        XBARCLKOUT_W { w: self }
    }
    #[doc = "Bit 24 - AFE clock output select"]
    #[inline(always)]
    pub fn afeoutclksel(&mut self) -> AFEOUTCLKSEL_W {
        AFEOUTCLKSEL_W { w: self }
    }
    #[doc = "Bits 26:27 - LPUART clock Source configuration"]
    #[inline(always)]
    pub fn lpuartsrc(&mut self) -> LPUARTSRC_W {
        LPUARTSRC_W { w: self }
    }
    #[doc = "Bit 31 - QTMR counters Freeze control"]
    #[inline(always)]
    pub fn tmrfreeze(&mut self) -> TMRFREEZE_W {
        TMRFREEZE_W { w: self }
    }
}
