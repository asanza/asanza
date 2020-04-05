#[doc = "Reader of register CTRL"]
pub type R = crate::R<u16, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u16, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Output Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUTMODE_A {
    #[doc = "0: Asserted while counter is active"]
    _000 = 0,
    #[doc = "1: Clear OFLAG output on successful compare"]
    _001 = 1,
    #[doc = "2: Set OFLAG output on successful compare"]
    _010 = 2,
    #[doc = "3: Toggle OFLAG output on successful compare"]
    _011 = 3,
    #[doc = "4: Toggle OFLAG output using alternating compare registers"]
    _100 = 4,
    #[doc = "5: Set on compare, cleared on secondary source input edge"]
    _101 = 5,
    #[doc = "6: Set on compare, cleared on counter rollover"]
    _110 = 6,
    #[doc = "7: Enable gated clock output while counter is active"]
    _111 = 7,
}
impl From<OUTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OUTMODE`"]
pub type OUTMODE_R = crate::R<u8, OUTMODE_A>;
impl OUTMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTMODE_A {
        match self.bits {
            0 => OUTMODE_A::_000,
            1 => OUTMODE_A::_001,
            2 => OUTMODE_A::_010,
            3 => OUTMODE_A::_011,
            4 => OUTMODE_A::_100,
            5 => OUTMODE_A::_101,
            6 => OUTMODE_A::_110,
            7 => OUTMODE_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == OUTMODE_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == OUTMODE_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == OUTMODE_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == OUTMODE_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == OUTMODE_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == OUTMODE_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == OUTMODE_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == OUTMODE_A::_111
    }
}
#[doc = "Write proxy for field `OUTMODE`"]
pub struct OUTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Asserted while counter is active"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(OUTMODE_A::_000)
    }
    #[doc = "Clear OFLAG output on successful compare"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(OUTMODE_A::_001)
    }
    #[doc = "Set OFLAG output on successful compare"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(OUTMODE_A::_010)
    }
    #[doc = "Toggle OFLAG output on successful compare"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(OUTMODE_A::_011)
    }
    #[doc = "Toggle OFLAG output using alternating compare registers"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(OUTMODE_A::_100)
    }
    #[doc = "Set on compare, cleared on secondary source input edge"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(OUTMODE_A::_101)
    }
    #[doc = "Set on compare, cleared on counter rollover"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(OUTMODE_A::_110)
    }
    #[doc = "Enable gated clock output while counter is active"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(OUTMODE_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "Co-Channel Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COINIT_A {
    #[doc = "0: Co-channel counter/timers cannot force a re-initialization of this counter/timer"]
    _0 = 0,
    #[doc = "1: Co-channel counter/timers may force a re-initialization of this counter/timer"]
    _1 = 1,
}
impl From<COINIT_A> for bool {
    #[inline(always)]
    fn from(variant: COINIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COINIT`"]
pub type COINIT_R = crate::R<bool, COINIT_A>;
impl COINIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COINIT_A {
        match self.bits {
            false => COINIT_A::_0,
            true => COINIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COINIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COINIT_A::_1
    }
}
#[doc = "Write proxy for field `COINIT`"]
pub struct COINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> COINIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COINIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Co-channel counter/timers cannot force a re-initialization of this counter/timer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COINIT_A::_0)
    }
    #[doc = "Co-channel counter/timers may force a re-initialization of this counter/timer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COINIT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Count Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR_A {
    #[doc = "0: Count up."]
    _0 = 0,
    #[doc = "1: Count down."]
    _1 = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, DIR_A>;
impl DIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::_0,
            true => DIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIR_A::_1
    }
}
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Count up."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIR_A::_0)
    }
    #[doc = "Count down."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Count Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LENGTH_A {
    #[doc = "0: Count until roll over at $FFFF and continue from $0000."]
    _0 = 0,
    #[doc = "1: Count until compare, then re-initialize. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on."]
    _1 = 1,
}
impl From<LENGTH_A> for bool {
    #[inline(always)]
    fn from(variant: LENGTH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LENGTH`"]
pub type LENGTH_R = crate::R<bool, LENGTH_A>;
impl LENGTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LENGTH_A {
        match self.bits {
            false => LENGTH_A::_0,
            true => LENGTH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LENGTH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LENGTH_A::_1
    }
}
#[doc = "Write proxy for field `LENGTH`"]
pub struct LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LENGTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LENGTH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Count until roll over at $FFFF and continue from $0000."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LENGTH_A::_0)
    }
    #[doc = "Count until compare, then re-initialize. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LENGTH_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Count Once\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONCE_A {
    #[doc = "0: Count repeatedly."]
    _0 = 0,
    #[doc = "1: Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops."]
    _1 = 1,
}
impl From<ONCE_A> for bool {
    #[inline(always)]
    fn from(variant: ONCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ONCE`"]
pub type ONCE_R = crate::R<bool, ONCE_A>;
impl ONCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONCE_A {
        match self.bits {
            false => ONCE_A::_0,
            true => ONCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ONCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ONCE_A::_1
    }
}
#[doc = "Write proxy for field `ONCE`"]
pub struct ONCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ONCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Count repeatedly."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ONCE_A::_0)
    }
    #[doc = "Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ONCE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Secondary Count Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCS_A {
    #[doc = "0: Counter 0 input pin"]
    _00 = 0,
    #[doc = "1: Counter 1 input pin"]
    _01 = 1,
    #[doc = "2: Counter 2 input pin"]
    _10 = 2,
    #[doc = "3: Counter 3 input pin"]
    _11 = 3,
}
impl From<SCS_A> for u8 {
    #[inline(always)]
    fn from(variant: SCS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SCS`"]
pub type SCS_R = crate::R<u8, SCS_A>;
impl SCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCS_A {
        match self.bits {
            0 => SCS_A::_00,
            1 => SCS_A::_01,
            2 => SCS_A::_10,
            3 => SCS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SCS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SCS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SCS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SCS_A::_11
    }
}
#[doc = "Write proxy for field `SCS`"]
pub struct SCS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Counter 0 input pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SCS_A::_00)
    }
    #[doc = "Counter 1 input pin"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SCS_A::_01)
    }
    #[doc = "Counter 2 input pin"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SCS_A::_10)
    }
    #[doc = "Counter 3 input pin"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SCS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u16) & 0x03) << 7);
        self.w
    }
}
#[doc = "Primary Count Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCS_A {
    #[doc = "0: Counter 0 input pin"]
    _0000 = 0,
    #[doc = "1: Counter 1 input pin"]
    _0001 = 1,
    #[doc = "2: Counter 2 input pin"]
    _0010 = 2,
    #[doc = "3: Counter 3 input pin"]
    _0011 = 3,
    #[doc = "4: Counter 0 output"]
    _0100 = 4,
    #[doc = "5: Counter 1 output"]
    _0101 = 5,
    #[doc = "6: Counter 2 output"]
    _0110 = 6,
    #[doc = "7: Counter 3 output"]
    _0111 = 7,
    #[doc = "8: IP bus clock divide by 1 prescaler"]
    _1000 = 8,
    #[doc = "9: IP bus clock divide by 2 prescaler"]
    _1001 = 9,
    #[doc = "10: IP bus clock divide by 4 prescaler"]
    _1010 = 10,
    #[doc = "11: IP bus clock divide by 8 prescaler"]
    _1011 = 11,
    #[doc = "12: IP bus clock divide by 16 prescaler"]
    _1100 = 12,
    #[doc = "13: IP bus clock divide by 32 prescaler"]
    _1101 = 13,
    #[doc = "14: IP bus clock divide by 64 prescaler"]
    _1110 = 14,
    #[doc = "15: IP bus clock divide by 128 prescaler"]
    _1111 = 15,
}
impl From<PCS_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCS`"]
pub type PCS_R = crate::R<u8, PCS_A>;
impl PCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS_A {
        match self.bits {
            0 => PCS_A::_0000,
            1 => PCS_A::_0001,
            2 => PCS_A::_0010,
            3 => PCS_A::_0011,
            4 => PCS_A::_0100,
            5 => PCS_A::_0101,
            6 => PCS_A::_0110,
            7 => PCS_A::_0111,
            8 => PCS_A::_1000,
            9 => PCS_A::_1001,
            10 => PCS_A::_1010,
            11 => PCS_A::_1011,
            12 => PCS_A::_1100,
            13 => PCS_A::_1101,
            14 => PCS_A::_1110,
            15 => PCS_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == PCS_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == PCS_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == PCS_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == PCS_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == PCS_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PCS_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == PCS_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == PCS_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == PCS_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == PCS_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == PCS_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == PCS_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == PCS_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == PCS_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == PCS_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == PCS_A::_1111
    }
}
#[doc = "Write proxy for field `PCS`"]
pub struct PCS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Counter 0 input pin"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(PCS_A::_0000)
    }
    #[doc = "Counter 1 input pin"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(PCS_A::_0001)
    }
    #[doc = "Counter 2 input pin"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(PCS_A::_0010)
    }
    #[doc = "Counter 3 input pin"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(PCS_A::_0011)
    }
    #[doc = "Counter 0 output"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(PCS_A::_0100)
    }
    #[doc = "Counter 1 output"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(PCS_A::_0101)
    }
    #[doc = "Counter 2 output"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(PCS_A::_0110)
    }
    #[doc = "Counter 3 output"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(PCS_A::_0111)
    }
    #[doc = "IP bus clock divide by 1 prescaler"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(PCS_A::_1000)
    }
    #[doc = "IP bus clock divide by 2 prescaler"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(PCS_A::_1001)
    }
    #[doc = "IP bus clock divide by 4 prescaler"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(PCS_A::_1010)
    }
    #[doc = "IP bus clock divide by 8 prescaler"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(PCS_A::_1011)
    }
    #[doc = "IP bus clock divide by 16 prescaler"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(PCS_A::_1100)
    }
    #[doc = "IP bus clock divide by 32 prescaler"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(PCS_A::_1101)
    }
    #[doc = "IP bus clock divide by 64 prescaler"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(PCS_A::_1110)
    }
    #[doc = "IP bus clock divide by 128 prescaler"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(PCS_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | (((value as u16) & 0x0f) << 9);
        self.w
    }
}
#[doc = "Count Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: No operation"]
    _000 = 0,
    #[doc = "1: Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\]
= 0. Falling edges are counted when SCTRL\\[IPS\\]
= 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
    _001 = 1,
    #[doc = "2: Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
    _010 = 2,
    #[doc = "3: Count rising edges of primary source while secondary input high active"]
    _011 = 3,
    #[doc = "4: Quadrature count mode, uses primary and secondary sources"]
    _100 = 4,
    #[doc = "5: Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\]
= 0. Falling edges are counted when SCTRL\\[IPS\\]
= 1."]
    _101 = 5,
    #[doc = "6: Edge of secondary source triggers primary count until compare"]
    _110 = 6,
    #[doc = "7: Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
    _111 = 7,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CM`"]
pub type CM_R = crate::R<u8, CM_A>;
impl CM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM_A {
        match self.bits {
            0 => CM_A::_000,
            1 => CM_A::_001,
            2 => CM_A::_010,
            3 => CM_A::_011,
            4 => CM_A::_100,
            5 => CM_A::_101,
            6 => CM_A::_110,
            7 => CM_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CM_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CM_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CM_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CM_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CM_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CM_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CM_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CM_A::_111
    }
}
#[doc = "Write proxy for field `CM`"]
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CM_A::_000)
    }
    #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\]
= 0. Falling edges are counted when SCTRL\\[IPS\\]
= 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CM_A::_001)
    }
    #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CM_A::_010)
    }
    #[doc = "Count rising edges of primary source while secondary input high active"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CM_A::_011)
    }
    #[doc = "Quadrature count mode, uses primary and secondary sources"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CM_A::_100)
    }
    #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\]
= 0. Falling edges are counted when SCTRL\\[IPS\\]
= 1."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CM_A::_101)
    }
    #[doc = "Edge of secondary source triggers primary count until compare"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CM_A::_110)
    }
    #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CM_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u16) & 0x07) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Output Mode"]
    #[inline(always)]
    pub fn outmode(&self) -> OUTMODE_R {
        OUTMODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Co-Channel Initialization"]
    #[inline(always)]
    pub fn coinit(&self) -> COINIT_R {
        COINIT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Count Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Count Length"]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Count Once"]
    #[inline(always)]
    pub fn once(&self) -> ONCE_R {
        ONCE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - Secondary Count Source"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 9:12 - Primary Count Source"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - Count Mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Output Mode"]
    #[inline(always)]
    pub fn outmode(&mut self) -> OUTMODE_W {
        OUTMODE_W { w: self }
    }
    #[doc = "Bit 3 - Co-Channel Initialization"]
    #[inline(always)]
    pub fn coinit(&mut self) -> COINIT_W {
        COINIT_W { w: self }
    }
    #[doc = "Bit 4 - Count Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 5 - Count Length"]
    #[inline(always)]
    pub fn length(&mut self) -> LENGTH_W {
        LENGTH_W { w: self }
    }
    #[doc = "Bit 6 - Count Once"]
    #[inline(always)]
    pub fn once(&mut self) -> ONCE_W {
        ONCE_W { w: self }
    }
    #[doc = "Bits 7:8 - Secondary Count Source"]
    #[inline(always)]
    pub fn scs(&mut self) -> SCS_W {
        SCS_W { w: self }
    }
    #[doc = "Bits 9:12 - Primary Count Source"]
    #[inline(always)]
    pub fn pcs(&mut self) -> PCS_W {
        PCS_W { w: self }
    }
    #[doc = "Bits 13:15 - Count Mode"]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
}
