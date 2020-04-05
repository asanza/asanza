#[doc = "Reader of register GCR"]
pub type R = crate::R<u32, super::GCR>;
#[doc = "Writer for register GCR"]
pub type W = crate::W<u32, super::GCR>;
#[doc = "Register GCR `reset()`'s with value 0x0830_0003"]
impl crate::ResetValue for super::GCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0830_0003
    }
}
#[doc = "LCD duty select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DUTY_A {
    #[doc = "0: Use 1 BP (1/1 duty cycle)."]
    _000 = 0,
    #[doc = "1: Use 2 BP (1/2 duty cycle)."]
    _001 = 1,
    #[doc = "2: Use 3 BP (1/3 duty cycle)."]
    _010 = 2,
    #[doc = "3: Use 4 BP (1/4 duty cycle). (Default)"]
    _011 = 3,
    #[doc = "4: Use 5 BP (1/5 duty cycle)."]
    _100 = 4,
    #[doc = "5: Use 6 BP (1/6 duty cycle)."]
    _101 = 5,
    #[doc = "6: Use 7 BP (1/7 duty cycle)."]
    _110 = 6,
    #[doc = "7: Use 8 BP (1/8 duty cycle)."]
    _111 = 7,
}
impl From<DUTY_A> for u8 {
    #[inline(always)]
    fn from(variant: DUTY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DUTY`"]
pub type DUTY_R = crate::R<u8, DUTY_A>;
impl DUTY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUTY_A {
        match self.bits {
            0 => DUTY_A::_000,
            1 => DUTY_A::_001,
            2 => DUTY_A::_010,
            3 => DUTY_A::_011,
            4 => DUTY_A::_100,
            5 => DUTY_A::_101,
            6 => DUTY_A::_110,
            7 => DUTY_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DUTY_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DUTY_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DUTY_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DUTY_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DUTY_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DUTY_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DUTY_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DUTY_A::_111
    }
}
#[doc = "Write proxy for field `DUTY`"]
pub struct DUTY_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DUTY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Use 1 BP (1/1 duty cycle)."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DUTY_A::_000)
    }
    #[doc = "Use 2 BP (1/2 duty cycle)."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DUTY_A::_001)
    }
    #[doc = "Use 3 BP (1/3 duty cycle)."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DUTY_A::_010)
    }
    #[doc = "Use 4 BP (1/4 duty cycle). (Default)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DUTY_A::_011)
    }
    #[doc = "Use 5 BP (1/5 duty cycle)."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DUTY_A::_100)
    }
    #[doc = "Use 6 BP (1/6 duty cycle)."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DUTY_A::_101)
    }
    #[doc = "Use 7 BP (1/7 duty cycle)."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DUTY_A::_110)
    }
    #[doc = "Use 8 BP (1/8 duty cycle)."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DUTY_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `LCLK`"]
pub type LCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LCLK`"]
pub struct LCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> LCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "LCD Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOURCE_A {
    #[doc = "0: Selects the default clock as the LCD clock source."]
    _0 = 0,
    #[doc = "1: Selects the alternate clock as the LCD clock source."]
    _1 = 1,
}
impl From<SOURCE_A> for bool {
    #[inline(always)]
    fn from(variant: SOURCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOURCE`"]
pub type SOURCE_R = crate::R<bool, SOURCE_A>;
impl SOURCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOURCE_A {
        match self.bits {
            false => SOURCE_A::_0,
            true => SOURCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOURCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOURCE_A::_1
    }
}
#[doc = "Write proxy for field `SOURCE`"]
pub struct SOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOURCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOURCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selects the default clock as the LCD clock source."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOURCE_A::_0)
    }
    #[doc = "Selects the alternate clock as the LCD clock source."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOURCE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "LCD Driver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCDEN_A {
    #[doc = "0: All front plane and back plane pins are disabled. The LCD controller system is also disabled, and all LCD waveform generation clocks are stopped. V LL3 is connected to V DD internally."]
    _0 = 0,
    #[doc = "1: LCD controller driver system is enabled, and front plane and back plane waveforms are generated. All LCD pins, LCD_Pn, enabled using the LCD Pin Enable register, output an LCD driver waveform. The back plane pins output an LCD driver back plane waveform based on the settings of DUTY\\[2:0\\]. Charge pump or resistor bias is enabled."]
    _1 = 1,
}
impl From<LCDEN_A> for bool {
    #[inline(always)]
    fn from(variant: LCDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LCDEN`"]
pub type LCDEN_R = crate::R<bool, LCDEN_A>;
impl LCDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDEN_A {
        match self.bits {
            false => LCDEN_A::_0,
            true => LCDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCDEN_A::_1
    }
}
#[doc = "Write proxy for field `LCDEN`"]
pub struct LCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All front plane and back plane pins are disabled. The LCD controller system is also disabled, and all LCD waveform generation clocks are stopped. V LL3 is connected to V DD internally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LCDEN_A::_0)
    }
    #[doc = "LCD controller driver system is enabled, and front plane and back plane waveforms are generated. All LCD pins, LCD_Pn, enabled using the LCD Pin Enable register, output an LCD driver waveform. The back plane pins output an LCD driver back plane waveform based on the settings of DUTY\\[2:0\\]. Charge pump or resistor bias is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LCDEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "LCD Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCDSTP_A {
    #[doc = "0: Allows the LCD driver, charge pump, resistor bias network, and voltage regulator to continue running during Stop mode."]
    _0 = 0,
    #[doc = "1: Disables the LCD driver, charge pump, resistor bias network, and voltage regulator when MCU enters Stop mode."]
    _1 = 1,
}
impl From<LCDSTP_A> for bool {
    #[inline(always)]
    fn from(variant: LCDSTP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LCDSTP`"]
pub type LCDSTP_R = crate::R<bool, LCDSTP_A>;
impl LCDSTP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDSTP_A {
        match self.bits {
            false => LCDSTP_A::_0,
            true => LCDSTP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCDSTP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCDSTP_A::_1
    }
}
#[doc = "Write proxy for field `LCDSTP`"]
pub struct LCDSTP_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDSTP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCDSTP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Allows the LCD driver, charge pump, resistor bias network, and voltage regulator to continue running during Stop mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LCDSTP_A::_0)
    }
    #[doc = "Disables the LCD driver, charge pump, resistor bias network, and voltage regulator when MCU enters Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LCDSTP_A::_1)
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
#[doc = "LCD Doze enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCDDOZE_A {
    #[doc = "0: Allows the LCD driver, charge pump, resistor bias network, and voltage regulator to continue running during Doze mode."]
    _0 = 0,
    #[doc = "1: Disables the LCD driver, charge pump, resistor bias network, and voltage regulator when MCU enters Doze mode."]
    _1 = 1,
}
impl From<LCDDOZE_A> for bool {
    #[inline(always)]
    fn from(variant: LCDDOZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LCDDOZE`"]
pub type LCDDOZE_R = crate::R<bool, LCDDOZE_A>;
impl LCDDOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDDOZE_A {
        match self.bits {
            false => LCDDOZE_A::_0,
            true => LCDDOZE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCDDOZE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCDDOZE_A::_1
    }
}
#[doc = "Write proxy for field `LCDDOZE`"]
pub struct LCDDOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDDOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCDDOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Allows the LCD driver, charge pump, resistor bias network, and voltage regulator to continue running during Doze mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LCDDOZE_A::_0)
    }
    #[doc = "Disables the LCD driver, charge pump, resistor bias network, and voltage regulator when MCU enters Doze mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LCDDOZE_A::_1)
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
#[doc = "LCD Alternate Clock Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ALTDIV_A {
    #[doc = "0: Divide factor = 1 (No divide)"]
    _00 = 0,
    #[doc = "1: Divide factor = 64"]
    _01 = 1,
    #[doc = "2: Divide factor = 256"]
    _10 = 2,
    #[doc = "3: Divide factor = 512"]
    _11 = 3,
}
impl From<ALTDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ALTDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ALTDIV`"]
pub type ALTDIV_R = crate::R<u8, ALTDIV_A>;
impl ALTDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALTDIV_A {
        match self.bits {
            0 => ALTDIV_A::_00,
            1 => ALTDIV_A::_01,
            2 => ALTDIV_A::_10,
            3 => ALTDIV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ALTDIV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ALTDIV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ALTDIV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ALTDIV_A::_11
    }
}
#[doc = "Write proxy for field `ALTDIV`"]
pub struct ALTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALTDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide factor = 1 (No divide)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ALTDIV_A::_00)
    }
    #[doc = "Divide factor = 64"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ALTDIV_A::_01)
    }
    #[doc = "Divide factor = 256"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ALTDIV_A::_10)
    }
    #[doc = "Divide factor = 512"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ALTDIV_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "LCD Fault Detection Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDCIEN_A {
    #[doc = "0: No interrupt request is generated by this event."]
    _0 = 0,
    #[doc = "1: When a fault is detected and FDCF bit is set, this event causes an interrupt request."]
    _1 = 1,
}
impl From<FDCIEN_A> for bool {
    #[inline(always)]
    fn from(variant: FDCIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FDCIEN`"]
pub type FDCIEN_R = crate::R<bool, FDCIEN_A>;
impl FDCIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDCIEN_A {
        match self.bits {
            false => FDCIEN_A::_0,
            true => FDCIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDCIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDCIEN_A::_1
    }
}
#[doc = "Write proxy for field `FDCIEN`"]
pub struct FDCIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDCIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt request is generated by this event."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDCIEN_A::_0)
    }
    #[doc = "When a fault is detected and FDCF bit is set, this event causes an interrupt request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDCIEN_A::_1)
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
#[doc = "Voltage Supply Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSUPPLY_A {
    #[doc = "0: Drive VLL3 internally from VDD"]
    _0 = 0,
    #[doc = "1: Drive VLL3 externally from VDD or drive VLL1 internally from vIREG"]
    _1 = 1,
}
impl From<VSUPPLY_A> for bool {
    #[inline(always)]
    fn from(variant: VSUPPLY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VSUPPLY`"]
pub type VSUPPLY_R = crate::R<bool, VSUPPLY_A>;
impl VSUPPLY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSUPPLY_A {
        match self.bits {
            false => VSUPPLY_A::_0,
            true => VSUPPLY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VSUPPLY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VSUPPLY_A::_1
    }
}
#[doc = "Write proxy for field `VSUPPLY`"]
pub struct VSUPPLY_W<'a> {
    w: &'a mut W,
}
impl<'a> VSUPPLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VSUPPLY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Drive VLL3 internally from VDD"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VSUPPLY_A::_0)
    }
    #[doc = "Drive VLL3 externally from VDD or drive VLL1 internally from vIREG"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VSUPPLY_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `LADJ`"]
pub type LADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LADJ`"]
pub struct LADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> LADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Charge Pump or Resistor Bias Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPSEL_A {
    #[doc = "0: LCD charge pump is disabled. Resistor network selected. (The internal 1/3-bias is forced.)"]
    _0 = 0,
    #[doc = "1: LCD charge pump is selected. Resistor network disabled. (The internal 1/3-bias is forced.)"]
    _1 = 1,
}
impl From<CPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CPSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPSEL`"]
pub type CPSEL_R = crate::R<bool, CPSEL_A>;
impl CPSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPSEL_A {
        match self.bits {
            false => CPSEL_A::_0,
            true => CPSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPSEL_A::_1
    }
}
#[doc = "Write proxy for field `CPSEL`"]
pub struct CPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LCD charge pump is disabled. Resistor network selected. (The internal 1/3-bias is forced.)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPSEL_A::_0)
    }
    #[doc = "LCD charge pump is selected. Resistor network disabled. (The internal 1/3-bias is forced.)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `RVTRIM`"]
pub type RVTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RVTRIM`"]
pub struct RVTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RVTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Regulated Voltage Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RVEN_A {
    #[doc = "0: Regulated voltage disabled."]
    _0 = 0,
    #[doc = "1: Regulated voltage enabled."]
    _1 = 1,
}
impl From<RVEN_A> for bool {
    #[inline(always)]
    fn from(variant: RVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RVEN`"]
pub type RVEN_R = crate::R<bool, RVEN_A>;
impl RVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RVEN_A {
        match self.bits {
            false => RVEN_A::_0,
            true => RVEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RVEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RVEN_A::_1
    }
}
#[doc = "Write proxy for field `RVEN`"]
pub struct RVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Regulated voltage disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RVEN_A::_0)
    }
    #[doc = "Regulated voltage enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RVEN_A::_1)
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
    #[doc = "Bits 0:2 - LCD duty select"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - LCD Clock Prescaler"]
    #[inline(always)]
    pub fn lclk(&self) -> LCLK_R {
        LCLK_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 6 - LCD Clock Source Select"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LCD Driver Enable"]
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LCD Stop"]
    #[inline(always)]
    pub fn lcdstp(&self) -> LCDSTP_R {
        LCDSTP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LCD Doze enable"]
    #[inline(always)]
    pub fn lcddoze(&self) -> LCDDOZE_R {
        LCDDOZE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - LCD Alternate Clock Divider"]
    #[inline(always)]
    pub fn altdiv(&self) -> ALTDIV_R {
        ALTDIV_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - LCD Fault Detection Complete Interrupt Enable"]
    #[inline(always)]
    pub fn fdcien(&self) -> FDCIEN_R {
        FDCIEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Voltage Supply Control"]
    #[inline(always)]
    pub fn vsupply(&self) -> VSUPPLY_R {
        VSUPPLY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Load Adjust"]
    #[inline(always)]
    pub fn ladj(&self) -> LADJ_R {
        LADJ_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Charge Pump or Resistor Bias Select"]
    #[inline(always)]
    pub fn cpsel(&self) -> CPSEL_R {
        CPSEL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Regulated Voltage Trim"]
    #[inline(always)]
    pub fn rvtrim(&self) -> RVTRIM_R {
        RVTRIM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Regulated Voltage Enable"]
    #[inline(always)]
    pub fn rven(&self) -> RVEN_R {
        RVEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - LCD duty select"]
    #[inline(always)]
    pub fn duty(&mut self) -> DUTY_W {
        DUTY_W { w: self }
    }
    #[doc = "Bits 3:5 - LCD Clock Prescaler"]
    #[inline(always)]
    pub fn lclk(&mut self) -> LCLK_W {
        LCLK_W { w: self }
    }
    #[doc = "Bit 6 - LCD Clock Source Select"]
    #[inline(always)]
    pub fn source(&mut self) -> SOURCE_W {
        SOURCE_W { w: self }
    }
    #[doc = "Bit 7 - LCD Driver Enable"]
    #[inline(always)]
    pub fn lcden(&mut self) -> LCDEN_W {
        LCDEN_W { w: self }
    }
    #[doc = "Bit 8 - LCD Stop"]
    #[inline(always)]
    pub fn lcdstp(&mut self) -> LCDSTP_W {
        LCDSTP_W { w: self }
    }
    #[doc = "Bit 9 - LCD Doze enable"]
    #[inline(always)]
    pub fn lcddoze(&mut self) -> LCDDOZE_W {
        LCDDOZE_W { w: self }
    }
    #[doc = "Bits 12:13 - LCD Alternate Clock Divider"]
    #[inline(always)]
    pub fn altdiv(&mut self) -> ALTDIV_W {
        ALTDIV_W { w: self }
    }
    #[doc = "Bit 14 - LCD Fault Detection Complete Interrupt Enable"]
    #[inline(always)]
    pub fn fdcien(&mut self) -> FDCIEN_W {
        FDCIEN_W { w: self }
    }
    #[doc = "Bit 17 - Voltage Supply Control"]
    #[inline(always)]
    pub fn vsupply(&mut self) -> VSUPPLY_W {
        VSUPPLY_W { w: self }
    }
    #[doc = "Bits 20:21 - Load Adjust"]
    #[inline(always)]
    pub fn ladj(&mut self) -> LADJ_W {
        LADJ_W { w: self }
    }
    #[doc = "Bit 23 - Charge Pump or Resistor Bias Select"]
    #[inline(always)]
    pub fn cpsel(&mut self) -> CPSEL_W {
        CPSEL_W { w: self }
    }
    #[doc = "Bits 24:27 - Regulated Voltage Trim"]
    #[inline(always)]
    pub fn rvtrim(&mut self) -> RVTRIM_W {
        RVTRIM_W { w: self }
    }
    #[doc = "Bit 31 - Regulated Voltage Enable"]
    #[inline(always)]
    pub fn rven(&mut self) -> RVEN_W {
        RVEN_W { w: self }
    }
}
