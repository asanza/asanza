#[doc = "Reader of register SEL16"]
pub type R = crate::R<u16, super::SEL16>;
#[doc = "Writer for register SEL16"]
pub type W = crate::W<u16, super::SEL16>;
#[doc = "Register SEL16 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEL16 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Input (XBAR_INn) to be muxed to XBAR_OUT32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL32_A {
    #[doc = "0: Logic 1 (VDD)"]
    _0 = 0,
    #[doc = "1: Logic 0 (VSS)"]
    _1 = 1,
    #[doc = "2: AFE modulator clock output"]
    _2 = 2,
    #[doc = "3: AFE modulator 0 data output"]
    _3 = 3,
    #[doc = "4: LPTimer Output"]
    _4 = 4,
    #[doc = "5: Clock Output"]
    _5 = 5,
    #[doc = "6: Quad Timer channel 0 output"]
    _6 = 6,
    #[doc = "7: Quad Timer channel 1 output"]
    _7 = 7,
    #[doc = "8: Quad Timer channel 2 output"]
    _8 = 8,
    #[doc = "9: Quad Timer channel 3 output"]
    _9 = 9,
    #[doc = "10: iRTC Clock Output"]
    _10 = 10,
    #[doc = "11: CMP0 Output"]
    _11 = 11,
    #[doc = "12: CMP1 Output"]
    _12 = 12,
    #[doc = "13: iRTC Alarm Output"]
    _13 = 13,
    #[doc = "14: UART TX Output (after modulation)"]
    _14 = 14,
    #[doc = "15: EWM Output (EWM_OUT)"]
    _15 = 15,
    #[doc = "16: PIT 0 Timer Interrupt Flag 0 (Timeout 0)"]
    _16 = 16,
    #[doc = "17: XBAR Input pin 0"]
    _17 = 17,
    #[doc = "18: XBAR Input pin 1"]
    _18 = 18,
    #[doc = "19: XBAR Input pin 2"]
    _19 = 19,
    #[doc = "20: XBAR Input pin 3"]
    _20 = 20,
    #[doc = "21: XBAR Input pin 4"]
    _21 = 21,
    #[doc = "22: XBAR Input pin 5"]
    _22 = 22,
    #[doc = "23: XBAR Input pin 6"]
    _23 = 23,
    #[doc = "24: XBAR Input pin 7"]
    _24 = 24,
    #[doc = "25: XBAR Input pin 8"]
    _25 = 25,
    #[doc = "26: ORed conversion complete flag for all SAR ADC channels"]
    _26 = 26,
    #[doc = "27: ORed conversion complete flag for all AFE channels"]
    _27 = 27,
    #[doc = "28: AFE Channel 0 conversion complete"]
    _28 = 28,
    #[doc = "29: AFE Channel 1 conversion complete"]
    _29 = 29,
    #[doc = "30: AFE Channel 2 conversion complete"]
    _30 = 30,
    #[doc = "31: AFE Channel 3 conversion complete"]
    _31 = 31,
    #[doc = "32: DMA Done Signal"]
    _32 = 32,
    #[doc = "33: XBAR Input pin 9"]
    _33 = 33,
    #[doc = "34: XBAR Input pin 10"]
    _34 = 34,
    #[doc = "35: CMP2 Output"]
    _35 = 35,
    #[doc = "36: PIT 0 Timer Interrupt Flag 1 (Timeout 1)"]
    _36 = 36,
    #[doc = "37: PIT 1 Timer Interrupt Flag 0 (Timeout 0)"]
    _37 = 37,
    #[doc = "38: PIT 1 Timer Interrupt Flag 1 (Timeout 1)"]
    _38 = 38,
    #[doc = "39: AFE modulator 1 data output"]
    _39 = 39,
    #[doc = "40: AFE modulator 2 data output"]
    _40 = 40,
    #[doc = "41: AFE modulator 3 data output"]
    _41 = 41,
    #[doc = "42: SAR ADC conversion complete A"]
    _42 = 42,
    #[doc = "43: SAR ADC conversion complete B"]
    _43 = 43,
    #[doc = "44: SAR ADC conversion complete C"]
    _44 = 44,
    #[doc = "45: SAR ADC conversion complete D"]
    _45 = 45,
    #[doc = "46: PDB0 CH0 Pre-trigger 0"]
    _46 = 46,
    #[doc = "47: PDB0 CH0 Pre-trigger 1"]
    _47 = 47,
    #[doc = "48: PDB0 CH0 Pre-trigger 2"]
    _48 = 48,
    #[doc = "49: PDB0 CH0 Pre-trigger 3"]
    _49 = 49,
    #[doc = "50: PDB0 CH0 Trigger"]
    _50 = 50,
    #[doc = "51: PDB0 Pulse-Out 0"]
    _51 = 51,
}
impl From<SEL32_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL32_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEL32`"]
pub type SEL32_R = crate::R<u8, SEL32_A>;
impl SEL32_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEL32_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEL32_A::_0),
            1 => Val(SEL32_A::_1),
            2 => Val(SEL32_A::_2),
            3 => Val(SEL32_A::_3),
            4 => Val(SEL32_A::_4),
            5 => Val(SEL32_A::_5),
            6 => Val(SEL32_A::_6),
            7 => Val(SEL32_A::_7),
            8 => Val(SEL32_A::_8),
            9 => Val(SEL32_A::_9),
            10 => Val(SEL32_A::_10),
            11 => Val(SEL32_A::_11),
            12 => Val(SEL32_A::_12),
            13 => Val(SEL32_A::_13),
            14 => Val(SEL32_A::_14),
            15 => Val(SEL32_A::_15),
            16 => Val(SEL32_A::_16),
            17 => Val(SEL32_A::_17),
            18 => Val(SEL32_A::_18),
            19 => Val(SEL32_A::_19),
            20 => Val(SEL32_A::_20),
            21 => Val(SEL32_A::_21),
            22 => Val(SEL32_A::_22),
            23 => Val(SEL32_A::_23),
            24 => Val(SEL32_A::_24),
            25 => Val(SEL32_A::_25),
            26 => Val(SEL32_A::_26),
            27 => Val(SEL32_A::_27),
            28 => Val(SEL32_A::_28),
            29 => Val(SEL32_A::_29),
            30 => Val(SEL32_A::_30),
            31 => Val(SEL32_A::_31),
            32 => Val(SEL32_A::_32),
            33 => Val(SEL32_A::_33),
            34 => Val(SEL32_A::_34),
            35 => Val(SEL32_A::_35),
            36 => Val(SEL32_A::_36),
            37 => Val(SEL32_A::_37),
            38 => Val(SEL32_A::_38),
            39 => Val(SEL32_A::_39),
            40 => Val(SEL32_A::_40),
            41 => Val(SEL32_A::_41),
            42 => Val(SEL32_A::_42),
            43 => Val(SEL32_A::_43),
            44 => Val(SEL32_A::_44),
            45 => Val(SEL32_A::_45),
            46 => Val(SEL32_A::_46),
            47 => Val(SEL32_A::_47),
            48 => Val(SEL32_A::_48),
            49 => Val(SEL32_A::_49),
            50 => Val(SEL32_A::_50),
            51 => Val(SEL32_A::_51),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEL32_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEL32_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == SEL32_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == SEL32_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == SEL32_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == SEL32_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == SEL32_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == SEL32_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == SEL32_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        *self == SEL32_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SEL32_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SEL32_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        *self == SEL32_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        *self == SEL32_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        *self == SEL32_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        *self == SEL32_A::_15
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == SEL32_A::_16
    }
    #[doc = "Checks if the value of the field is `_17`"]
    #[inline(always)]
    pub fn is_17(&self) -> bool {
        *self == SEL32_A::_17
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline(always)]
    pub fn is_18(&self) -> bool {
        *self == SEL32_A::_18
    }
    #[doc = "Checks if the value of the field is `_19`"]
    #[inline(always)]
    pub fn is_19(&self) -> bool {
        *self == SEL32_A::_19
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        *self == SEL32_A::_20
    }
    #[doc = "Checks if the value of the field is `_21`"]
    #[inline(always)]
    pub fn is_21(&self) -> bool {
        *self == SEL32_A::_21
    }
    #[doc = "Checks if the value of the field is `_22`"]
    #[inline(always)]
    pub fn is_22(&self) -> bool {
        *self == SEL32_A::_22
    }
    #[doc = "Checks if the value of the field is `_23`"]
    #[inline(always)]
    pub fn is_23(&self) -> bool {
        *self == SEL32_A::_23
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == SEL32_A::_24
    }
    #[doc = "Checks if the value of the field is `_25`"]
    #[inline(always)]
    pub fn is_25(&self) -> bool {
        *self == SEL32_A::_25
    }
    #[doc = "Checks if the value of the field is `_26`"]
    #[inline(always)]
    pub fn is_26(&self) -> bool {
        *self == SEL32_A::_26
    }
    #[doc = "Checks if the value of the field is `_27`"]
    #[inline(always)]
    pub fn is_27(&self) -> bool {
        *self == SEL32_A::_27
    }
    #[doc = "Checks if the value of the field is `_28`"]
    #[inline(always)]
    pub fn is_28(&self) -> bool {
        *self == SEL32_A::_28
    }
    #[doc = "Checks if the value of the field is `_29`"]
    #[inline(always)]
    pub fn is_29(&self) -> bool {
        *self == SEL32_A::_29
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline(always)]
    pub fn is_30(&self) -> bool {
        *self == SEL32_A::_30
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline(always)]
    pub fn is_31(&self) -> bool {
        *self == SEL32_A::_31
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == SEL32_A::_32
    }
    #[doc = "Checks if the value of the field is `_33`"]
    #[inline(always)]
    pub fn is_33(&self) -> bool {
        *self == SEL32_A::_33
    }
    #[doc = "Checks if the value of the field is `_34`"]
    #[inline(always)]
    pub fn is_34(&self) -> bool {
        *self == SEL32_A::_34
    }
    #[doc = "Checks if the value of the field is `_35`"]
    #[inline(always)]
    pub fn is_35(&self) -> bool {
        *self == SEL32_A::_35
    }
    #[doc = "Checks if the value of the field is `_36`"]
    #[inline(always)]
    pub fn is_36(&self) -> bool {
        *self == SEL32_A::_36
    }
    #[doc = "Checks if the value of the field is `_37`"]
    #[inline(always)]
    pub fn is_37(&self) -> bool {
        *self == SEL32_A::_37
    }
    #[doc = "Checks if the value of the field is `_38`"]
    #[inline(always)]
    pub fn is_38(&self) -> bool {
        *self == SEL32_A::_38
    }
    #[doc = "Checks if the value of the field is `_39`"]
    #[inline(always)]
    pub fn is_39(&self) -> bool {
        *self == SEL32_A::_39
    }
    #[doc = "Checks if the value of the field is `_40`"]
    #[inline(always)]
    pub fn is_40(&self) -> bool {
        *self == SEL32_A::_40
    }
    #[doc = "Checks if the value of the field is `_41`"]
    #[inline(always)]
    pub fn is_41(&self) -> bool {
        *self == SEL32_A::_41
    }
    #[doc = "Checks if the value of the field is `_42`"]
    #[inline(always)]
    pub fn is_42(&self) -> bool {
        *self == SEL32_A::_42
    }
    #[doc = "Checks if the value of the field is `_43`"]
    #[inline(always)]
    pub fn is_43(&self) -> bool {
        *self == SEL32_A::_43
    }
    #[doc = "Checks if the value of the field is `_44`"]
    #[inline(always)]
    pub fn is_44(&self) -> bool {
        *self == SEL32_A::_44
    }
    #[doc = "Checks if the value of the field is `_45`"]
    #[inline(always)]
    pub fn is_45(&self) -> bool {
        *self == SEL32_A::_45
    }
    #[doc = "Checks if the value of the field is `_46`"]
    #[inline(always)]
    pub fn is_46(&self) -> bool {
        *self == SEL32_A::_46
    }
    #[doc = "Checks if the value of the field is `_47`"]
    #[inline(always)]
    pub fn is_47(&self) -> bool {
        *self == SEL32_A::_47
    }
    #[doc = "Checks if the value of the field is `_48`"]
    #[inline(always)]
    pub fn is_48(&self) -> bool {
        *self == SEL32_A::_48
    }
    #[doc = "Checks if the value of the field is `_49`"]
    #[inline(always)]
    pub fn is_49(&self) -> bool {
        *self == SEL32_A::_49
    }
    #[doc = "Checks if the value of the field is `_50`"]
    #[inline(always)]
    pub fn is_50(&self) -> bool {
        *self == SEL32_A::_50
    }
    #[doc = "Checks if the value of the field is `_51`"]
    #[inline(always)]
    pub fn is_51(&self) -> bool {
        *self == SEL32_A::_51
    }
}
#[doc = "Write proxy for field `SEL32`"]
pub struct SEL32_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL32_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL32_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Logic 1 (VDD)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEL32_A::_0)
    }
    #[doc = "Logic 0 (VSS)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEL32_A::_1)
    }
    #[doc = "AFE modulator clock output"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(SEL32_A::_2)
    }
    #[doc = "AFE modulator 0 data output"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(SEL32_A::_3)
    }
    #[doc = "LPTimer Output"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(SEL32_A::_4)
    }
    #[doc = "Clock Output"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(SEL32_A::_5)
    }
    #[doc = "Quad Timer channel 0 output"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(SEL32_A::_6)
    }
    #[doc = "Quad Timer channel 1 output"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(SEL32_A::_7)
    }
    #[doc = "Quad Timer channel 2 output"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(SEL32_A::_8)
    }
    #[doc = "Quad Timer channel 3 output"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(SEL32_A::_9)
    }
    #[doc = "iRTC Clock Output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SEL32_A::_10)
    }
    #[doc = "CMP0 Output"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SEL32_A::_11)
    }
    #[doc = "CMP1 Output"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(SEL32_A::_12)
    }
    #[doc = "iRTC Alarm Output"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(SEL32_A::_13)
    }
    #[doc = "UART TX Output (after modulation)"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(SEL32_A::_14)
    }
    #[doc = "EWM Output (EWM_OUT)"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(SEL32_A::_15)
    }
    #[doc = "PIT 0 Timer Interrupt Flag 0 (Timeout 0)"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(SEL32_A::_16)
    }
    #[doc = "XBAR Input pin 0"]
    #[inline(always)]
    pub fn _17(self) -> &'a mut W {
        self.variant(SEL32_A::_17)
    }
    #[doc = "XBAR Input pin 1"]
    #[inline(always)]
    pub fn _18(self) -> &'a mut W {
        self.variant(SEL32_A::_18)
    }
    #[doc = "XBAR Input pin 2"]
    #[inline(always)]
    pub fn _19(self) -> &'a mut W {
        self.variant(SEL32_A::_19)
    }
    #[doc = "XBAR Input pin 3"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut W {
        self.variant(SEL32_A::_20)
    }
    #[doc = "XBAR Input pin 4"]
    #[inline(always)]
    pub fn _21(self) -> &'a mut W {
        self.variant(SEL32_A::_21)
    }
    #[doc = "XBAR Input pin 5"]
    #[inline(always)]
    pub fn _22(self) -> &'a mut W {
        self.variant(SEL32_A::_22)
    }
    #[doc = "XBAR Input pin 6"]
    #[inline(always)]
    pub fn _23(self) -> &'a mut W {
        self.variant(SEL32_A::_23)
    }
    #[doc = "XBAR Input pin 7"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(SEL32_A::_24)
    }
    #[doc = "XBAR Input pin 8"]
    #[inline(always)]
    pub fn _25(self) -> &'a mut W {
        self.variant(SEL32_A::_25)
    }
    #[doc = "ORed conversion complete flag for all SAR ADC channels"]
    #[inline(always)]
    pub fn _26(self) -> &'a mut W {
        self.variant(SEL32_A::_26)
    }
    #[doc = "ORed conversion complete flag for all AFE channels"]
    #[inline(always)]
    pub fn _27(self) -> &'a mut W {
        self.variant(SEL32_A::_27)
    }
    #[doc = "AFE Channel 0 conversion complete"]
    #[inline(always)]
    pub fn _28(self) -> &'a mut W {
        self.variant(SEL32_A::_28)
    }
    #[doc = "AFE Channel 1 conversion complete"]
    #[inline(always)]
    pub fn _29(self) -> &'a mut W {
        self.variant(SEL32_A::_29)
    }
    #[doc = "AFE Channel 2 conversion complete"]
    #[inline(always)]
    pub fn _30(self) -> &'a mut W {
        self.variant(SEL32_A::_30)
    }
    #[doc = "AFE Channel 3 conversion complete"]
    #[inline(always)]
    pub fn _31(self) -> &'a mut W {
        self.variant(SEL32_A::_31)
    }
    #[doc = "DMA Done Signal"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(SEL32_A::_32)
    }
    #[doc = "XBAR Input pin 9"]
    #[inline(always)]
    pub fn _33(self) -> &'a mut W {
        self.variant(SEL32_A::_33)
    }
    #[doc = "XBAR Input pin 10"]
    #[inline(always)]
    pub fn _34(self) -> &'a mut W {
        self.variant(SEL32_A::_34)
    }
    #[doc = "CMP2 Output"]
    #[inline(always)]
    pub fn _35(self) -> &'a mut W {
        self.variant(SEL32_A::_35)
    }
    #[doc = "PIT 0 Timer Interrupt Flag 1 (Timeout 1)"]
    #[inline(always)]
    pub fn _36(self) -> &'a mut W {
        self.variant(SEL32_A::_36)
    }
    #[doc = "PIT 1 Timer Interrupt Flag 0 (Timeout 0)"]
    #[inline(always)]
    pub fn _37(self) -> &'a mut W {
        self.variant(SEL32_A::_37)
    }
    #[doc = "PIT 1 Timer Interrupt Flag 1 (Timeout 1)"]
    #[inline(always)]
    pub fn _38(self) -> &'a mut W {
        self.variant(SEL32_A::_38)
    }
    #[doc = "AFE modulator 1 data output"]
    #[inline(always)]
    pub fn _39(self) -> &'a mut W {
        self.variant(SEL32_A::_39)
    }
    #[doc = "AFE modulator 2 data output"]
    #[inline(always)]
    pub fn _40(self) -> &'a mut W {
        self.variant(SEL32_A::_40)
    }
    #[doc = "AFE modulator 3 data output"]
    #[inline(always)]
    pub fn _41(self) -> &'a mut W {
        self.variant(SEL32_A::_41)
    }
    #[doc = "SAR ADC conversion complete A"]
    #[inline(always)]
    pub fn _42(self) -> &'a mut W {
        self.variant(SEL32_A::_42)
    }
    #[doc = "SAR ADC conversion complete B"]
    #[inline(always)]
    pub fn _43(self) -> &'a mut W {
        self.variant(SEL32_A::_43)
    }
    #[doc = "SAR ADC conversion complete C"]
    #[inline(always)]
    pub fn _44(self) -> &'a mut W {
        self.variant(SEL32_A::_44)
    }
    #[doc = "SAR ADC conversion complete D"]
    #[inline(always)]
    pub fn _45(self) -> &'a mut W {
        self.variant(SEL32_A::_45)
    }
    #[doc = "PDB0 CH0 Pre-trigger 0"]
    #[inline(always)]
    pub fn _46(self) -> &'a mut W {
        self.variant(SEL32_A::_46)
    }
    #[doc = "PDB0 CH0 Pre-trigger 1"]
    #[inline(always)]
    pub fn _47(self) -> &'a mut W {
        self.variant(SEL32_A::_47)
    }
    #[doc = "PDB0 CH0 Pre-trigger 2"]
    #[inline(always)]
    pub fn _48(self) -> &'a mut W {
        self.variant(SEL32_A::_48)
    }
    #[doc = "PDB0 CH0 Pre-trigger 3"]
    #[inline(always)]
    pub fn _49(self) -> &'a mut W {
        self.variant(SEL32_A::_49)
    }
    #[doc = "PDB0 CH0 Trigger"]
    #[inline(always)]
    pub fn _50(self) -> &'a mut W {
        self.variant(SEL32_A::_50)
    }
    #[doc = "PDB0 Pulse-Out 0"]
    #[inline(always)]
    pub fn _51(self) -> &'a mut W {
        self.variant(SEL32_A::_51)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u16) & 0x3f);
        self.w
    }
}
#[doc = "Input (XBAR_INn) to be muxed to XBAR_OUT33\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL33_A {
    #[doc = "0: Logic 1 (VDD)"]
    _0 = 0,
    #[doc = "1: Logic 0 (VSS)"]
    _1 = 1,
    #[doc = "2: AFE modulator clock output"]
    _2 = 2,
    #[doc = "3: AFE modulator 0 data output"]
    _3 = 3,
    #[doc = "4: LPTimer Output"]
    _4 = 4,
    #[doc = "5: Clock Output"]
    _5 = 5,
    #[doc = "6: Quad Timer channel 0 output"]
    _6 = 6,
    #[doc = "7: Quad Timer channel 1 output"]
    _7 = 7,
    #[doc = "8: Quad Timer channel 2 output"]
    _8 = 8,
    #[doc = "9: Quad Timer channel 3 output"]
    _9 = 9,
    #[doc = "10: iRTC Clock Output"]
    _10 = 10,
    #[doc = "11: CMP0 Output"]
    _11 = 11,
    #[doc = "12: CMP1 Output"]
    _12 = 12,
    #[doc = "13: iRTC Alarm Output"]
    _13 = 13,
    #[doc = "14: UART TX Output (after modulation)"]
    _14 = 14,
    #[doc = "15: EWM Output (EWM_OUT)"]
    _15 = 15,
    #[doc = "16: PIT 0 Timer Interrupt Flag 0 (Timeout 0)"]
    _16 = 16,
    #[doc = "17: XBAR Input pin 0"]
    _17 = 17,
    #[doc = "18: XBAR Input pin 1"]
    _18 = 18,
    #[doc = "19: XBAR Input pin 2"]
    _19 = 19,
    #[doc = "20: XBAR Input pin 3"]
    _20 = 20,
    #[doc = "21: XBAR Input pin 4"]
    _21 = 21,
    #[doc = "22: XBAR Input pin 5"]
    _22 = 22,
    #[doc = "23: XBAR Input pin 6"]
    _23 = 23,
    #[doc = "24: XBAR Input pin 7"]
    _24 = 24,
    #[doc = "25: XBAR Input pin 8"]
    _25 = 25,
    #[doc = "26: ORed conversion complete flag for all SAR ADC channels"]
    _26 = 26,
    #[doc = "27: ORed conversion complete flag for all AFE channels"]
    _27 = 27,
    #[doc = "28: AFE Channel 0 conversion complete"]
    _28 = 28,
    #[doc = "29: AFE Channel 1 conversion complete"]
    _29 = 29,
    #[doc = "30: AFE Channel 2 conversion complete"]
    _30 = 30,
    #[doc = "31: AFE Channel 3 conversion complete"]
    _31 = 31,
    #[doc = "32: DMA Done Signal"]
    _32 = 32,
    #[doc = "33: XBAR Input pin 9"]
    _33 = 33,
    #[doc = "34: XBAR Input pin 10"]
    _34 = 34,
    #[doc = "35: CMP2 Output"]
    _35 = 35,
    #[doc = "36: PIT 0 Timer Interrupt Flag 1 (Timeout 1)"]
    _36 = 36,
    #[doc = "37: PIT 1 Timer Interrupt Flag 0 (Timeout 0)"]
    _37 = 37,
    #[doc = "38: PIT 1 Timer Interrupt Flag 1 (Timeout 1)"]
    _38 = 38,
    #[doc = "39: AFE modulator 1 data output"]
    _39 = 39,
    #[doc = "40: AFE modulator 2 data output"]
    _40 = 40,
    #[doc = "41: AFE modulator 3 data output"]
    _41 = 41,
    #[doc = "42: SAR ADC conversion complete A"]
    _42 = 42,
    #[doc = "43: SAR ADC conversion complete B"]
    _43 = 43,
    #[doc = "44: SAR ADC conversion complete C"]
    _44 = 44,
    #[doc = "45: SAR ADC conversion complete D"]
    _45 = 45,
    #[doc = "46: PDB0 CH0 Pre-trigger 0"]
    _46 = 46,
    #[doc = "47: PDB0 CH0 Pre-trigger 1"]
    _47 = 47,
    #[doc = "48: PDB0 CH0 Pre-trigger 2"]
    _48 = 48,
    #[doc = "49: PDB0 CH0 Pre-trigger 3"]
    _49 = 49,
    #[doc = "50: PDB0 CH0 Trigger"]
    _50 = 50,
    #[doc = "51: PDB0 Pulse-Out 0"]
    _51 = 51,
}
impl From<SEL33_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL33_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEL33`"]
pub type SEL33_R = crate::R<u8, SEL33_A>;
impl SEL33_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEL33_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEL33_A::_0),
            1 => Val(SEL33_A::_1),
            2 => Val(SEL33_A::_2),
            3 => Val(SEL33_A::_3),
            4 => Val(SEL33_A::_4),
            5 => Val(SEL33_A::_5),
            6 => Val(SEL33_A::_6),
            7 => Val(SEL33_A::_7),
            8 => Val(SEL33_A::_8),
            9 => Val(SEL33_A::_9),
            10 => Val(SEL33_A::_10),
            11 => Val(SEL33_A::_11),
            12 => Val(SEL33_A::_12),
            13 => Val(SEL33_A::_13),
            14 => Val(SEL33_A::_14),
            15 => Val(SEL33_A::_15),
            16 => Val(SEL33_A::_16),
            17 => Val(SEL33_A::_17),
            18 => Val(SEL33_A::_18),
            19 => Val(SEL33_A::_19),
            20 => Val(SEL33_A::_20),
            21 => Val(SEL33_A::_21),
            22 => Val(SEL33_A::_22),
            23 => Val(SEL33_A::_23),
            24 => Val(SEL33_A::_24),
            25 => Val(SEL33_A::_25),
            26 => Val(SEL33_A::_26),
            27 => Val(SEL33_A::_27),
            28 => Val(SEL33_A::_28),
            29 => Val(SEL33_A::_29),
            30 => Val(SEL33_A::_30),
            31 => Val(SEL33_A::_31),
            32 => Val(SEL33_A::_32),
            33 => Val(SEL33_A::_33),
            34 => Val(SEL33_A::_34),
            35 => Val(SEL33_A::_35),
            36 => Val(SEL33_A::_36),
            37 => Val(SEL33_A::_37),
            38 => Val(SEL33_A::_38),
            39 => Val(SEL33_A::_39),
            40 => Val(SEL33_A::_40),
            41 => Val(SEL33_A::_41),
            42 => Val(SEL33_A::_42),
            43 => Val(SEL33_A::_43),
            44 => Val(SEL33_A::_44),
            45 => Val(SEL33_A::_45),
            46 => Val(SEL33_A::_46),
            47 => Val(SEL33_A::_47),
            48 => Val(SEL33_A::_48),
            49 => Val(SEL33_A::_49),
            50 => Val(SEL33_A::_50),
            51 => Val(SEL33_A::_51),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEL33_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEL33_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == SEL33_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == SEL33_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == SEL33_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == SEL33_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == SEL33_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == SEL33_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == SEL33_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        *self == SEL33_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SEL33_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SEL33_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        *self == SEL33_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        *self == SEL33_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        *self == SEL33_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        *self == SEL33_A::_15
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == SEL33_A::_16
    }
    #[doc = "Checks if the value of the field is `_17`"]
    #[inline(always)]
    pub fn is_17(&self) -> bool {
        *self == SEL33_A::_17
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline(always)]
    pub fn is_18(&self) -> bool {
        *self == SEL33_A::_18
    }
    #[doc = "Checks if the value of the field is `_19`"]
    #[inline(always)]
    pub fn is_19(&self) -> bool {
        *self == SEL33_A::_19
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        *self == SEL33_A::_20
    }
    #[doc = "Checks if the value of the field is `_21`"]
    #[inline(always)]
    pub fn is_21(&self) -> bool {
        *self == SEL33_A::_21
    }
    #[doc = "Checks if the value of the field is `_22`"]
    #[inline(always)]
    pub fn is_22(&self) -> bool {
        *self == SEL33_A::_22
    }
    #[doc = "Checks if the value of the field is `_23`"]
    #[inline(always)]
    pub fn is_23(&self) -> bool {
        *self == SEL33_A::_23
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == SEL33_A::_24
    }
    #[doc = "Checks if the value of the field is `_25`"]
    #[inline(always)]
    pub fn is_25(&self) -> bool {
        *self == SEL33_A::_25
    }
    #[doc = "Checks if the value of the field is `_26`"]
    #[inline(always)]
    pub fn is_26(&self) -> bool {
        *self == SEL33_A::_26
    }
    #[doc = "Checks if the value of the field is `_27`"]
    #[inline(always)]
    pub fn is_27(&self) -> bool {
        *self == SEL33_A::_27
    }
    #[doc = "Checks if the value of the field is `_28`"]
    #[inline(always)]
    pub fn is_28(&self) -> bool {
        *self == SEL33_A::_28
    }
    #[doc = "Checks if the value of the field is `_29`"]
    #[inline(always)]
    pub fn is_29(&self) -> bool {
        *self == SEL33_A::_29
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline(always)]
    pub fn is_30(&self) -> bool {
        *self == SEL33_A::_30
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline(always)]
    pub fn is_31(&self) -> bool {
        *self == SEL33_A::_31
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == SEL33_A::_32
    }
    #[doc = "Checks if the value of the field is `_33`"]
    #[inline(always)]
    pub fn is_33(&self) -> bool {
        *self == SEL33_A::_33
    }
    #[doc = "Checks if the value of the field is `_34`"]
    #[inline(always)]
    pub fn is_34(&self) -> bool {
        *self == SEL33_A::_34
    }
    #[doc = "Checks if the value of the field is `_35`"]
    #[inline(always)]
    pub fn is_35(&self) -> bool {
        *self == SEL33_A::_35
    }
    #[doc = "Checks if the value of the field is `_36`"]
    #[inline(always)]
    pub fn is_36(&self) -> bool {
        *self == SEL33_A::_36
    }
    #[doc = "Checks if the value of the field is `_37`"]
    #[inline(always)]
    pub fn is_37(&self) -> bool {
        *self == SEL33_A::_37
    }
    #[doc = "Checks if the value of the field is `_38`"]
    #[inline(always)]
    pub fn is_38(&self) -> bool {
        *self == SEL33_A::_38
    }
    #[doc = "Checks if the value of the field is `_39`"]
    #[inline(always)]
    pub fn is_39(&self) -> bool {
        *self == SEL33_A::_39
    }
    #[doc = "Checks if the value of the field is `_40`"]
    #[inline(always)]
    pub fn is_40(&self) -> bool {
        *self == SEL33_A::_40
    }
    #[doc = "Checks if the value of the field is `_41`"]
    #[inline(always)]
    pub fn is_41(&self) -> bool {
        *self == SEL33_A::_41
    }
    #[doc = "Checks if the value of the field is `_42`"]
    #[inline(always)]
    pub fn is_42(&self) -> bool {
        *self == SEL33_A::_42
    }
    #[doc = "Checks if the value of the field is `_43`"]
    #[inline(always)]
    pub fn is_43(&self) -> bool {
        *self == SEL33_A::_43
    }
    #[doc = "Checks if the value of the field is `_44`"]
    #[inline(always)]
    pub fn is_44(&self) -> bool {
        *self == SEL33_A::_44
    }
    #[doc = "Checks if the value of the field is `_45`"]
    #[inline(always)]
    pub fn is_45(&self) -> bool {
        *self == SEL33_A::_45
    }
    #[doc = "Checks if the value of the field is `_46`"]
    #[inline(always)]
    pub fn is_46(&self) -> bool {
        *self == SEL33_A::_46
    }
    #[doc = "Checks if the value of the field is `_47`"]
    #[inline(always)]
    pub fn is_47(&self) -> bool {
        *self == SEL33_A::_47
    }
    #[doc = "Checks if the value of the field is `_48`"]
    #[inline(always)]
    pub fn is_48(&self) -> bool {
        *self == SEL33_A::_48
    }
    #[doc = "Checks if the value of the field is `_49`"]
    #[inline(always)]
    pub fn is_49(&self) -> bool {
        *self == SEL33_A::_49
    }
    #[doc = "Checks if the value of the field is `_50`"]
    #[inline(always)]
    pub fn is_50(&self) -> bool {
        *self == SEL33_A::_50
    }
    #[doc = "Checks if the value of the field is `_51`"]
    #[inline(always)]
    pub fn is_51(&self) -> bool {
        *self == SEL33_A::_51
    }
}
#[doc = "Write proxy for field `SEL33`"]
pub struct SEL33_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL33_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL33_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Logic 1 (VDD)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEL33_A::_0)
    }
    #[doc = "Logic 0 (VSS)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEL33_A::_1)
    }
    #[doc = "AFE modulator clock output"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(SEL33_A::_2)
    }
    #[doc = "AFE modulator 0 data output"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(SEL33_A::_3)
    }
    #[doc = "LPTimer Output"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(SEL33_A::_4)
    }
    #[doc = "Clock Output"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(SEL33_A::_5)
    }
    #[doc = "Quad Timer channel 0 output"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(SEL33_A::_6)
    }
    #[doc = "Quad Timer channel 1 output"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(SEL33_A::_7)
    }
    #[doc = "Quad Timer channel 2 output"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(SEL33_A::_8)
    }
    #[doc = "Quad Timer channel 3 output"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(SEL33_A::_9)
    }
    #[doc = "iRTC Clock Output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SEL33_A::_10)
    }
    #[doc = "CMP0 Output"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SEL33_A::_11)
    }
    #[doc = "CMP1 Output"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(SEL33_A::_12)
    }
    #[doc = "iRTC Alarm Output"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(SEL33_A::_13)
    }
    #[doc = "UART TX Output (after modulation)"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(SEL33_A::_14)
    }
    #[doc = "EWM Output (EWM_OUT)"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(SEL33_A::_15)
    }
    #[doc = "PIT 0 Timer Interrupt Flag 0 (Timeout 0)"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(SEL33_A::_16)
    }
    #[doc = "XBAR Input pin 0"]
    #[inline(always)]
    pub fn _17(self) -> &'a mut W {
        self.variant(SEL33_A::_17)
    }
    #[doc = "XBAR Input pin 1"]
    #[inline(always)]
    pub fn _18(self) -> &'a mut W {
        self.variant(SEL33_A::_18)
    }
    #[doc = "XBAR Input pin 2"]
    #[inline(always)]
    pub fn _19(self) -> &'a mut W {
        self.variant(SEL33_A::_19)
    }
    #[doc = "XBAR Input pin 3"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut W {
        self.variant(SEL33_A::_20)
    }
    #[doc = "XBAR Input pin 4"]
    #[inline(always)]
    pub fn _21(self) -> &'a mut W {
        self.variant(SEL33_A::_21)
    }
    #[doc = "XBAR Input pin 5"]
    #[inline(always)]
    pub fn _22(self) -> &'a mut W {
        self.variant(SEL33_A::_22)
    }
    #[doc = "XBAR Input pin 6"]
    #[inline(always)]
    pub fn _23(self) -> &'a mut W {
        self.variant(SEL33_A::_23)
    }
    #[doc = "XBAR Input pin 7"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(SEL33_A::_24)
    }
    #[doc = "XBAR Input pin 8"]
    #[inline(always)]
    pub fn _25(self) -> &'a mut W {
        self.variant(SEL33_A::_25)
    }
    #[doc = "ORed conversion complete flag for all SAR ADC channels"]
    #[inline(always)]
    pub fn _26(self) -> &'a mut W {
        self.variant(SEL33_A::_26)
    }
    #[doc = "ORed conversion complete flag for all AFE channels"]
    #[inline(always)]
    pub fn _27(self) -> &'a mut W {
        self.variant(SEL33_A::_27)
    }
    #[doc = "AFE Channel 0 conversion complete"]
    #[inline(always)]
    pub fn _28(self) -> &'a mut W {
        self.variant(SEL33_A::_28)
    }
    #[doc = "AFE Channel 1 conversion complete"]
    #[inline(always)]
    pub fn _29(self) -> &'a mut W {
        self.variant(SEL33_A::_29)
    }
    #[doc = "AFE Channel 2 conversion complete"]
    #[inline(always)]
    pub fn _30(self) -> &'a mut W {
        self.variant(SEL33_A::_30)
    }
    #[doc = "AFE Channel 3 conversion complete"]
    #[inline(always)]
    pub fn _31(self) -> &'a mut W {
        self.variant(SEL33_A::_31)
    }
    #[doc = "DMA Done Signal"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(SEL33_A::_32)
    }
    #[doc = "XBAR Input pin 9"]
    #[inline(always)]
    pub fn _33(self) -> &'a mut W {
        self.variant(SEL33_A::_33)
    }
    #[doc = "XBAR Input pin 10"]
    #[inline(always)]
    pub fn _34(self) -> &'a mut W {
        self.variant(SEL33_A::_34)
    }
    #[doc = "CMP2 Output"]
    #[inline(always)]
    pub fn _35(self) -> &'a mut W {
        self.variant(SEL33_A::_35)
    }
    #[doc = "PIT 0 Timer Interrupt Flag 1 (Timeout 1)"]
    #[inline(always)]
    pub fn _36(self) -> &'a mut W {
        self.variant(SEL33_A::_36)
    }
    #[doc = "PIT 1 Timer Interrupt Flag 0 (Timeout 0)"]
    #[inline(always)]
    pub fn _37(self) -> &'a mut W {
        self.variant(SEL33_A::_37)
    }
    #[doc = "PIT 1 Timer Interrupt Flag 1 (Timeout 1)"]
    #[inline(always)]
    pub fn _38(self) -> &'a mut W {
        self.variant(SEL33_A::_38)
    }
    #[doc = "AFE modulator 1 data output"]
    #[inline(always)]
    pub fn _39(self) -> &'a mut W {
        self.variant(SEL33_A::_39)
    }
    #[doc = "AFE modulator 2 data output"]
    #[inline(always)]
    pub fn _40(self) -> &'a mut W {
        self.variant(SEL33_A::_40)
    }
    #[doc = "AFE modulator 3 data output"]
    #[inline(always)]
    pub fn _41(self) -> &'a mut W {
        self.variant(SEL33_A::_41)
    }
    #[doc = "SAR ADC conversion complete A"]
    #[inline(always)]
    pub fn _42(self) -> &'a mut W {
        self.variant(SEL33_A::_42)
    }
    #[doc = "SAR ADC conversion complete B"]
    #[inline(always)]
    pub fn _43(self) -> &'a mut W {
        self.variant(SEL33_A::_43)
    }
    #[doc = "SAR ADC conversion complete C"]
    #[inline(always)]
    pub fn _44(self) -> &'a mut W {
        self.variant(SEL33_A::_44)
    }
    #[doc = "SAR ADC conversion complete D"]
    #[inline(always)]
    pub fn _45(self) -> &'a mut W {
        self.variant(SEL33_A::_45)
    }
    #[doc = "PDB0 CH0 Pre-trigger 0"]
    #[inline(always)]
    pub fn _46(self) -> &'a mut W {
        self.variant(SEL33_A::_46)
    }
    #[doc = "PDB0 CH0 Pre-trigger 1"]
    #[inline(always)]
    pub fn _47(self) -> &'a mut W {
        self.variant(SEL33_A::_47)
    }
    #[doc = "PDB0 CH0 Pre-trigger 2"]
    #[inline(always)]
    pub fn _48(self) -> &'a mut W {
        self.variant(SEL33_A::_48)
    }
    #[doc = "PDB0 CH0 Pre-trigger 3"]
    #[inline(always)]
    pub fn _49(self) -> &'a mut W {
        self.variant(SEL33_A::_49)
    }
    #[doc = "PDB0 CH0 Trigger"]
    #[inline(always)]
    pub fn _50(self) -> &'a mut W {
        self.variant(SEL33_A::_50)
    }
    #[doc = "PDB0 Pulse-Out 0"]
    #[inline(always)]
    pub fn _51(self) -> &'a mut W {
        self.variant(SEL33_A::_51)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u16) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Input (XBAR_INn) to be muxed to XBAR_OUT32"]
    #[inline(always)]
    pub fn sel32(&self) -> SEL32_R {
        SEL32_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Input (XBAR_INn) to be muxed to XBAR_OUT33"]
    #[inline(always)]
    pub fn sel33(&self) -> SEL33_R {
        SEL33_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Input (XBAR_INn) to be muxed to XBAR_OUT32"]
    #[inline(always)]
    pub fn sel32(&mut self) -> SEL32_W {
        SEL32_W { w: self }
    }
    #[doc = "Bits 8:13 - Input (XBAR_INn) to be muxed to XBAR_OUT33"]
    #[inline(always)]
    pub fn sel33(&mut self) -> SEL33_W {
        SEL33_W { w: self }
    }
}
