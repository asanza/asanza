#[doc = "Reader of register MISC_CTL"]
pub type R = crate::R<u32, super::MISC_CTL>;
#[doc = "Writer for register MISC_CTL"]
pub type W = crate::W<u32, super::MISC_CTL>;
#[doc = "Register MISC_CTL `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::MISC_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "RTC oscillator status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCON_A {
    #[doc = "0: RTC oscillator is disabled."]
    _0 = 0,
    #[doc = "1: RTC oscillator is enabled."]
    _1 = 1,
}
impl From<OSCON_A> for bool {
    #[inline(always)]
    fn from(variant: OSCON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSCON`"]
pub type OSCON_R = crate::R<bool, OSCON_A>;
impl OSCON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCON_A {
        match self.bits {
            false => OSCON_A::_0,
            true => OSCON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSCON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSCON_A::_1
    }
}
#[doc = "PDB bypass XBAR as ADC trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDBADCTRG_A {
    #[doc = "0: XBAR to trigger ADC"]
    _0 = 0,
    #[doc = "1: PDB output to trigger ADC"]
    _1 = 1,
}
impl From<PDBADCTRG_A> for bool {
    #[inline(always)]
    fn from(variant: PDBADCTRG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDBADCTRG`"]
pub type PDBADCTRG_R = crate::R<bool, PDBADCTRG_A>;
impl PDBADCTRG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDBADCTRG_A {
        match self.bits {
            false => PDBADCTRG_A::_0,
            true => PDBADCTRG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDBADCTRG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDBADCTRG_A::_1
    }
}
#[doc = "Write proxy for field `PDBADCTRG`"]
pub struct PDBADCTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> PDBADCTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDBADCTRG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XBAR to trigger ADC"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDBADCTRG_A::_0)
    }
    #[doc = "PDB output to trigger ADC"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDBADCTRG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "DMA Done select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMADONESEL_A {
    #[doc = "0: DMA0"]
    _00 = 0,
    #[doc = "1: DMA1"]
    _01 = 1,
    #[doc = "2: DMA2"]
    _10 = 2,
    #[doc = "3: DMA3"]
    _11 = 3,
}
impl From<DMADONESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMADONESEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMADONESEL`"]
pub type DMADONESEL_R = crate::R<u8, DMADONESEL_A>;
impl DMADONESEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMADONESEL_A {
        match self.bits {
            0 => DMADONESEL_A::_00,
            1 => DMADONESEL_A::_01,
            2 => DMADONESEL_A::_10,
            3 => DMADONESEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DMADONESEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DMADONESEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DMADONESEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DMADONESEL_A::_11
    }
}
#[doc = "Write proxy for field `DMADONESEL`"]
pub struct DMADONESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADONESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMADONESEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DMA0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DMADONESEL_A::_00)
    }
    #[doc = "DMA1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DMADONESEL_A::_01)
    }
    #[doc = "DMA2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DMADONESEL_A::_10)
    }
    #[doc = "DMA3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DMADONESEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "AFE Clock Source Select (SIMAFECLK selection)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AFECLKSEL_A {
    #[doc = "0: MCG PLL Clock selected"]
    _00 = 0,
    #[doc = "1: MCG FLL Clock selected"]
    _01 = 1,
    #[doc = "2: OSC Clock selected"]
    _10 = 2,
    #[doc = "3: Disabled"]
    _11 = 3,
}
impl From<AFECLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: AFECLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AFECLKSEL`"]
pub type AFECLKSEL_R = crate::R<u8, AFECLKSEL_A>;
impl AFECLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFECLKSEL_A {
        match self.bits {
            0 => AFECLKSEL_A::_00,
            1 => AFECLKSEL_A::_01,
            2 => AFECLKSEL_A::_10,
            3 => AFECLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == AFECLKSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == AFECLKSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == AFECLKSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == AFECLKSEL_A::_11
    }
}
#[doc = "Write proxy for field `AFECLKSEL`"]
pub struct AFECLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> AFECLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFECLKSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "MCG PLL Clock selected"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(AFECLKSEL_A::_00)
    }
    #[doc = "MCG FLL Clock selected"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(AFECLKSEL_A::_01)
    }
    #[doc = "OSC Clock selected"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(AFECLKSEL_A::_10)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(AFECLKSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "AFE Clock Pad Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFECLKPADDIR_A {
    #[doc = "0: AFE CLK PAD is input"]
    _0 = 0,
    #[doc = "1: AFE CLK PAD is output"]
    _1 = 1,
}
impl From<AFECLKPADDIR_A> for bool {
    #[inline(always)]
    fn from(variant: AFECLKPADDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AFECLKPADDIR`"]
pub type AFECLKPADDIR_R = crate::R<bool, AFECLKPADDIR_A>;
impl AFECLKPADDIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFECLKPADDIR_A {
        match self.bits {
            false => AFECLKPADDIR_A::_0,
            true => AFECLKPADDIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AFECLKPADDIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AFECLKPADDIR_A::_1
    }
}
#[doc = "Write proxy for field `AFECLKPADDIR`"]
pub struct AFECLKPADDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> AFECLKPADDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFECLKPADDIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AFE CLK PAD is input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AFECLKPADDIR_A::_0)
    }
    #[doc = "AFE CLK PAD is output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AFECLKPADDIR_A::_1)
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
#[doc = "UART Modulation Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTMODTYPE_A {
    #[doc = "0: TypeA (OR'ed) Modulation selected for IrDA"]
    _0 = 0,
    #[doc = "1: TypeB (AND'ed) Modulation selected for IrDA"]
    _1 = 1,
}
impl From<UARTMODTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: UARTMODTYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UARTMODTYPE`"]
pub type UARTMODTYPE_R = crate::R<bool, UARTMODTYPE_A>;
impl UARTMODTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UARTMODTYPE_A {
        match self.bits {
            false => UARTMODTYPE_A::_0,
            true => UARTMODTYPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UARTMODTYPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UARTMODTYPE_A::_1
    }
}
#[doc = "Write proxy for field `UARTMODTYPE`"]
pub struct UARTMODTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTMODTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UARTMODTYPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TypeA (OR'ed) Modulation selected for IrDA"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UARTMODTYPE_A::_0)
    }
    #[doc = "TypeB (AND'ed) Modulation selected for IrDA"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UARTMODTYPE_A::_1)
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
#[doc = "UART0 IrDA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0IRSEL_A {
    #[doc = "0: Pad RX input (PTD\\[0\\], PTF\\[3\\]
or PTK\\[3\\], as selected in Pinmux control) selected for RX input of UART0 and UART0 TX signal is not used for modulation"]
    _0 = 0,
    #[doc = "1: UART0 selected for IrDA modulation. UART0 TX modulated by XBAR_OUT\\[14\\]
and UART0 RX input connected to XBAR_OUT\\[13\\]. UARTxIRSEL cannot configure XBAR_OUT\\[14\\]
and XBAR_OUT\\[13\\]
automatically, and they need extra configuration in XBAR. User should configure XBAR\\[14:13\\]
accordingly."]
    _1 = 1,
}
impl From<UART0IRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: UART0IRSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART0IRSEL`"]
pub type UART0IRSEL_R = crate::R<bool, UART0IRSEL_A>;
impl UART0IRSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0IRSEL_A {
        match self.bits {
            false => UART0IRSEL_A::_0,
            true => UART0IRSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART0IRSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART0IRSEL_A::_1
    }
}
#[doc = "Write proxy for field `UART0IRSEL`"]
pub struct UART0IRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0IRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0IRSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad RX input (PTD\\[0\\], PTF\\[3\\]
or PTK\\[3\\], as selected in Pinmux control) selected for RX input of UART0 and UART0 TX signal is not used for modulation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0IRSEL_A::_0)
    }
    #[doc = "UART0 selected for IrDA modulation. UART0 TX modulated by XBAR_OUT\\[14\\]
and UART0 RX input connected to XBAR_OUT\\[13\\]. UARTxIRSEL cannot configure XBAR_OUT\\[14\\]
and XBAR_OUT\\[13\\]
automatically, and they need extra configuration in XBAR. User should configure XBAR\\[14:13\\]
accordingly."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0IRSEL_A::_1)
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
#[doc = "UART1 IrDA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1IRSEL_A {
    #[doc = "0: Pad RX input (PTD\\[2\\], PTI\\[0\\]
or PTK\\[5\\], as selected in Pinmux control) selected for RX input of UART1 and UART1 TX signal is not used for modulation"]
    _0 = 0,
    #[doc = "1: UART1 selected for IrDA modulation. UART1 TX modulated by XBAR_OUT\\[14\\]
and UART1 RX input connected to XBAR_OUT\\[13\\].UARTxIRSEL cannot configure XBAR_OUT\\[14\\]
and XBAR_OUT\\[13\\]
automatically, and they need extra configuration in XBAR. User should configure XBAR\\[14:13\\]
accordingly."]
    _1 = 1,
}
impl From<UART1IRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: UART1IRSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART1IRSEL`"]
pub type UART1IRSEL_R = crate::R<bool, UART1IRSEL_A>;
impl UART1IRSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1IRSEL_A {
        match self.bits {
            false => UART1IRSEL_A::_0,
            true => UART1IRSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART1IRSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART1IRSEL_A::_1
    }
}
#[doc = "Write proxy for field `UART1IRSEL`"]
pub struct UART1IRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1IRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1IRSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad RX input (PTD\\[2\\], PTI\\[0\\]
or PTK\\[5\\], as selected in Pinmux control) selected for RX input of UART1 and UART1 TX signal is not used for modulation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART1IRSEL_A::_0)
    }
    #[doc = "UART1 selected for IrDA modulation. UART1 TX modulated by XBAR_OUT\\[14\\]
and UART1 RX input connected to XBAR_OUT\\[13\\].UARTxIRSEL cannot configure XBAR_OUT\\[14\\]
and XBAR_OUT\\[13\\]
automatically, and they need extra configuration in XBAR. User should configure XBAR\\[14:13\\]
accordingly."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART1IRSEL_A::_1)
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
#[doc = "UART2 IrDA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2IRSEL_A {
    #[doc = "0: Pad RX input PTI\\[6\\]
or PTE\\[6\\]
selected for RX input of UART2 and UART2 TX signal is not used for modulation"]
    _0 = 0,
    #[doc = "1: UART2 selected for IrDA modulation. UART2 TX modulated by XBAR_OUT\\[14\\]
and UART2 RX input connected to XBAR_OUT\\[13\\].UARTxIRSEL cannot configure XBAR_OUT\\[14\\]
and XBAR_OUT\\[13\\]
automatically, and they need extra configuration in XBAR. User should configure XBAR\\[14:13\\]
accordingly."]
    _1 = 1,
}
impl From<UART2IRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: UART2IRSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART2IRSEL`"]
pub type UART2IRSEL_R = crate::R<bool, UART2IRSEL_A>;
impl UART2IRSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2IRSEL_A {
        match self.bits {
            false => UART2IRSEL_A::_0,
            true => UART2IRSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART2IRSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART2IRSEL_A::_1
    }
}
#[doc = "Write proxy for field `UART2IRSEL`"]
pub struct UART2IRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2IRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART2IRSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad RX input PTI\\[6\\]
or PTE\\[6\\]
selected for RX input of UART2 and UART2 TX signal is not used for modulation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART2IRSEL_A::_0)
    }
    #[doc = "UART2 selected for IrDA modulation. UART2 TX modulated by XBAR_OUT\\[14\\]
and UART2 RX input connected to XBAR_OUT\\[13\\].UARTxIRSEL cannot configure XBAR_OUT\\[14\\]
and XBAR_OUT\\[13\\]
automatically, and they need extra configuration in XBAR. User should configure XBAR\\[14:13\\]
accordingly."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART2IRSEL_A::_1)
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
#[doc = "UART3 IrDA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART3IRSEL_A {
    #[doc = "0: Pad RX input (PTC\\[3\\]
or PTD\\[7\\], as selected in Pinmux control) selected for RX input of UART3 and UART3 TX signal is not used for modulation"]
    _0 = 0,
    #[doc = "1: UART3 selected for IrDA modulation. UART3 TX modulated by XBAR_OUT\\[14\\]
and UART3 RX input connected to XBAR_OUT\\[13\\]. UARTxIRSEL cannot configure XBAR_OUT\\[14\\]
and XBAR_OUT\\[13\\]
automatically, and they need extra configuration in XBAR. User should configure XBAR\\[14:13\\]
accordingly."]
    _1 = 1,
}
impl From<UART3IRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: UART3IRSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART3IRSEL`"]
pub type UART3IRSEL_R = crate::R<bool, UART3IRSEL_A>;
impl UART3IRSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART3IRSEL_A {
        match self.bits {
            false => UART3IRSEL_A::_0,
            true => UART3IRSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART3IRSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART3IRSEL_A::_1
    }
}
#[doc = "Write proxy for field `UART3IRSEL`"]
pub struct UART3IRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3IRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART3IRSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad RX input (PTC\\[3\\]
or PTD\\[7\\], as selected in Pinmux control) selected for RX input of UART3 and UART3 TX signal is not used for modulation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART3IRSEL_A::_0)
    }
    #[doc = "UART3 selected for IrDA modulation. UART3 TX modulated by XBAR_OUT\\[14\\]
and UART3 RX input connected to XBAR_OUT\\[13\\]. UARTxIRSEL cannot configure XBAR_OUT\\[14\\]
and XBAR_OUT\\[13\\]
automatically, and they need extra configuration in XBAR. User should configure XBAR\\[14:13\\]
accordingly."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART3IRSEL_A::_1)
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
#[doc = "External Watchdog Monitor Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWMINSEL_A {
    #[doc = "0: Input from PAD (PTL\\[3\\], PTE\\[2\\]
or PTE\\[3\\]
as selected from Pinmux control )"]
    _0 = 0,
    #[doc = "1: Peripheral Crossbar (XBAR) Output\\[32\\]"]
    _1 = 1,
}
impl From<EWMINSEL_A> for bool {
    #[inline(always)]
    fn from(variant: EWMINSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EWMINSEL`"]
pub type EWMINSEL_R = crate::R<bool, EWMINSEL_A>;
impl EWMINSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWMINSEL_A {
        match self.bits {
            false => EWMINSEL_A::_0,
            true => EWMINSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EWMINSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EWMINSEL_A::_1
    }
}
#[doc = "Write proxy for field `EWMINSEL`"]
pub struct EWMINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EWMINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EWMINSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input from PAD (PTL\\[3\\], PTE\\[2\\]
or PTE\\[3\\]
as selected from Pinmux control )"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EWMINSEL_A::_0)
    }
    #[doc = "Peripheral Crossbar (XBAR) Output\\[32\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EWMINSEL_A::_1)
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
#[doc = "Timer CH0 PLL clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR0PLLSEL_A {
    #[doc = "0: Selects Bus Clock as source for the Timer CH0"]
    _0 = 0,
    #[doc = "1: Selects the PLL_AFE clock as the source for Timer CH0. The PLL_AFE clock source is itself selected using the MISC_CTL\\[5:4\\]"]
    _1 = 1,
}
impl From<TMR0PLLSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TMR0PLLSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMR0PLLSEL`"]
pub type TMR0PLLSEL_R = crate::R<bool, TMR0PLLSEL_A>;
impl TMR0PLLSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR0PLLSEL_A {
        match self.bits {
            false => TMR0PLLSEL_A::_0,
            true => TMR0PLLSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMR0PLLSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMR0PLLSEL_A::_1
    }
}
#[doc = "Write proxy for field `TMR0PLLSEL`"]
pub struct TMR0PLLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR0PLLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR0PLLSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selects Bus Clock as source for the Timer CH0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR0PLLSEL_A::_0)
    }
    #[doc = "Selects the PLL_AFE clock as the source for Timer CH0. The PLL_AFE clock source is itself selected using the MISC_CTL\\[5:4\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR0PLLSEL_A::_1)
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
#[doc = "Quadtimer Channel0 Secondary Count source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR0SCSEL_A {
    #[doc = "0: Pad PTF1 or PTD5, depending upon PCTL configuration."]
    _0 = 0,
    #[doc = "1: Peripheral Crossbar (XBAR) Output\\[5\\]"]
    _1 = 1,
}
impl From<TMR0SCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TMR0SCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMR0SCSEL`"]
pub type TMR0SCSEL_R = crate::R<bool, TMR0SCSEL_A>;
impl TMR0SCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR0SCSEL_A {
        match self.bits {
            false => TMR0SCSEL_A::_0,
            true => TMR0SCSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMR0SCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMR0SCSEL_A::_1
    }
}
#[doc = "Write proxy for field `TMR0SCSEL`"]
pub struct TMR0SCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR0SCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR0SCSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad PTF1 or PTD5, depending upon PCTL configuration."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR0SCSEL_A::_0)
    }
    #[doc = "Peripheral Crossbar (XBAR) Output\\[5\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR0SCSEL_A::_1)
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
#[doc = "Quadtimer Channel1 Secondary Count source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR1SCSEL_A {
    #[doc = "0: Pad PTG0 or PTC6, depending upon PCTL configuration."]
    _0 = 0,
    #[doc = "1: Peripheral Crossbar (XBAR) Output\\[6\\]"]
    _1 = 1,
}
impl From<TMR1SCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TMR1SCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMR1SCSEL`"]
pub type TMR1SCSEL_R = crate::R<bool, TMR1SCSEL_A>;
impl TMR1SCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR1SCSEL_A {
        match self.bits {
            false => TMR1SCSEL_A::_0,
            true => TMR1SCSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMR1SCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMR1SCSEL_A::_1
    }
}
#[doc = "Write proxy for field `TMR1SCSEL`"]
pub struct TMR1SCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR1SCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR1SCSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad PTG0 or PTC6, depending upon PCTL configuration."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR1SCSEL_A::_0)
    }
    #[doc = "Peripheral Crossbar (XBAR) Output\\[6\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR1SCSEL_A::_1)
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
#[doc = "Quadtimer Channel2 Secondary Count source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR2SCSEL_A {
    #[doc = "0: Pad PTF7 or PTF0, depending upon PCTL configuration."]
    _0 = 0,
    #[doc = "1: Peripheral Crossbar (XBAR) Output\\[7\\]"]
    _1 = 1,
}
impl From<TMR2SCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TMR2SCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMR2SCSEL`"]
pub type TMR2SCSEL_R = crate::R<bool, TMR2SCSEL_A>;
impl TMR2SCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR2SCSEL_A {
        match self.bits {
            false => TMR2SCSEL_A::_0,
            true => TMR2SCSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMR2SCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMR2SCSEL_A::_1
    }
}
#[doc = "Write proxy for field `TMR2SCSEL`"]
pub struct TMR2SCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR2SCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR2SCSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad PTF7 or PTF0, depending upon PCTL configuration."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR2SCSEL_A::_0)
    }
    #[doc = "Peripheral Crossbar (XBAR) Output\\[7\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR2SCSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Quadtimer Channel3 Secondary Count source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR3SCSEL_A {
    #[doc = "0: Pad PTE5 or PTD1, depending upon PCTL configuration."]
    _0 = 0,
    #[doc = "1: Peripheral Crossbar (XBAR) Output\\[8\\]"]
    _1 = 1,
}
impl From<TMR3SCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TMR3SCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMR3SCSEL`"]
pub type TMR3SCSEL_R = crate::R<bool, TMR3SCSEL_A>;
impl TMR3SCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR3SCSEL_A {
        match self.bits {
            false => TMR3SCSEL_A::_0,
            true => TMR3SCSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMR3SCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMR3SCSEL_A::_1
    }
}
#[doc = "Write proxy for field `TMR3SCSEL`"]
pub struct TMR3SCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR3SCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR3SCSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad PTE5 or PTD1, depending upon PCTL configuration."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR3SCSEL_A::_0)
    }
    #[doc = "Peripheral Crossbar (XBAR) Output\\[8\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR3SCSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Quadtimer Channel0 Primary Count Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMR0PCSSEL_A {
    #[doc = "0: Bus Clock"]
    _00 = 0,
    #[doc = "1: Peripheral Crossbar Output \\[9\\]"]
    _01 = 1,
    #[doc = "2: Peripheral Crossbar Output \\[10\\]"]
    _10 = 2,
    #[doc = "3: Disabled"]
    _11 = 3,
}
impl From<TMR0PCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR0PCSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMR0PCSSEL`"]
pub type TMR0PCSSEL_R = crate::R<u8, TMR0PCSSEL_A>;
impl TMR0PCSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR0PCSSEL_A {
        match self.bits {
            0 => TMR0PCSSEL_A::_00,
            1 => TMR0PCSSEL_A::_01,
            2 => TMR0PCSSEL_A::_10,
            3 => TMR0PCSSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TMR0PCSSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TMR0PCSSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TMR0PCSSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TMR0PCSSEL_A::_11
    }
}
#[doc = "Write proxy for field `TMR0PCSSEL`"]
pub struct TMR0PCSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR0PCSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR0PCSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bus Clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TMR0PCSSEL_A::_00)
    }
    #[doc = "Peripheral Crossbar Output \\[9\\]"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TMR0PCSSEL_A::_01)
    }
    #[doc = "Peripheral Crossbar Output \\[10\\]"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TMR0PCSSEL_A::_10)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TMR0PCSSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Quadtimer Channel1 Primary Count Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMR1PCSSEL_A {
    #[doc = "0: Bus Clock"]
    _00 = 0,
    #[doc = "1: Peripheral Crossbar Output \\[9\\]"]
    _01 = 1,
    #[doc = "2: Peripheral Crossbar Output \\[10\\]"]
    _10 = 2,
    #[doc = "3: Disabled"]
    _11 = 3,
}
impl From<TMR1PCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR1PCSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMR1PCSSEL`"]
pub type TMR1PCSSEL_R = crate::R<u8, TMR1PCSSEL_A>;
impl TMR1PCSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR1PCSSEL_A {
        match self.bits {
            0 => TMR1PCSSEL_A::_00,
            1 => TMR1PCSSEL_A::_01,
            2 => TMR1PCSSEL_A::_10,
            3 => TMR1PCSSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TMR1PCSSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TMR1PCSSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TMR1PCSSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TMR1PCSSEL_A::_11
    }
}
#[doc = "Write proxy for field `TMR1PCSSEL`"]
pub struct TMR1PCSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR1PCSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR1PCSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bus Clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TMR1PCSSEL_A::_00)
    }
    #[doc = "Peripheral Crossbar Output \\[9\\]"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TMR1PCSSEL_A::_01)
    }
    #[doc = "Peripheral Crossbar Output \\[10\\]"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TMR1PCSSEL_A::_10)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TMR1PCSSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Quadtimer Channel2 Primary Count Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMR2PCSSEL_A {
    #[doc = "0: Bus Clock"]
    _00 = 0,
    #[doc = "1: Peripheral Crossbar Output \\[9\\]"]
    _01 = 1,
    #[doc = "2: Peripheral Crossbar Output \\[10\\]"]
    _10 = 2,
    #[doc = "3: Disabled"]
    _11 = 3,
}
impl From<TMR2PCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR2PCSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMR2PCSSEL`"]
pub type TMR2PCSSEL_R = crate::R<u8, TMR2PCSSEL_A>;
impl TMR2PCSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR2PCSSEL_A {
        match self.bits {
            0 => TMR2PCSSEL_A::_00,
            1 => TMR2PCSSEL_A::_01,
            2 => TMR2PCSSEL_A::_10,
            3 => TMR2PCSSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TMR2PCSSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TMR2PCSSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TMR2PCSSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TMR2PCSSEL_A::_11
    }
}
#[doc = "Write proxy for field `TMR2PCSSEL`"]
pub struct TMR2PCSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR2PCSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR2PCSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bus Clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TMR2PCSSEL_A::_00)
    }
    #[doc = "Peripheral Crossbar Output \\[9\\]"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TMR2PCSSEL_A::_01)
    }
    #[doc = "Peripheral Crossbar Output \\[10\\]"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TMR2PCSSEL_A::_10)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TMR2PCSSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Quadtimer Channel3 Primary Count Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMR3PCSSEL_A {
    #[doc = "0: Bus Clock"]
    _00 = 0,
    #[doc = "1: Peripheral Crossbar Output \\[9\\]"]
    _01 = 1,
    #[doc = "2: Peripheral Crossbar Output \\[10\\]"]
    _10 = 2,
    #[doc = "3: Disabled"]
    _11 = 3,
}
impl From<TMR3PCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR3PCSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMR3PCSSEL`"]
pub type TMR3PCSSEL_R = crate::R<u8, TMR3PCSSEL_A>;
impl TMR3PCSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR3PCSSEL_A {
        match self.bits {
            0 => TMR3PCSSEL_A::_00,
            1 => TMR3PCSSEL_A::_01,
            2 => TMR3PCSSEL_A::_10,
            3 => TMR3PCSSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TMR3PCSSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TMR3PCSSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TMR3PCSSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TMR3PCSSEL_A::_11
    }
}
#[doc = "Write proxy for field `TMR3PCSSEL`"]
pub struct TMR3PCSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR3PCSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR3PCSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bus Clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TMR3PCSSEL_A::_00)
    }
    #[doc = "Peripheral Crossbar Output \\[9\\]"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TMR3PCSSEL_A::_01)
    }
    #[doc = "Peripheral Crossbar Output \\[10\\]"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TMR3PCSSEL_A::_10)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TMR3PCSSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "RTC Clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCCLKSEL_A {
    #[doc = "0: RTC OSC_32K clock selected"]
    _0 = 0,
    #[doc = "1: MCGIRCLK selected"]
    _1 = 1,
}
impl From<RTCCLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RTCCLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCCLKSEL`"]
pub type RTCCLKSEL_R = crate::R<bool, RTCCLKSEL_A>;
impl RTCCLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCLKSEL_A {
        match self.bits {
            false => RTCCLKSEL_A::_0,
            true => RTCCLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCCLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCCLKSEL_A::_1
    }
}
#[doc = "Write proxy for field `RTCCLKSEL`"]
pub struct RTCCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCCLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTC OSC_32K clock selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCCLKSEL_A::_0)
    }
    #[doc = "MCGIRCLK selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCCLKSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "VrefBuffer Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFBUFOUTEN_A {
    #[doc = "0: Buffer does not drive PAD"]
    _0 = 0,
    #[doc = "1: Buffer drives selected voltage (selected by vref_buffer_sel) on pad"]
    _1 = 1,
}
impl From<VREFBUFOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFBUFOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VREFBUFOUTEN`"]
pub type VREFBUFOUTEN_R = crate::R<bool, VREFBUFOUTEN_A>;
impl VREFBUFOUTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFBUFOUTEN_A {
        match self.bits {
            false => VREFBUFOUTEN_A::_0,
            true => VREFBUFOUTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VREFBUFOUTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VREFBUFOUTEN_A::_1
    }
}
#[doc = "Write proxy for field `VREFBUFOUTEN`"]
pub struct VREFBUFOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFBUFOUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFBUFOUTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Buffer does not drive PAD"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VREFBUFOUTEN_A::_0)
    }
    #[doc = "Buffer drives selected voltage (selected by vref_buffer_sel) on pad"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VREFBUFOUTEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "VrefBuffer Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFBUFINSEL_A {
    #[doc = "0: Internal Reference selected as Buffer Input"]
    _0 = 0,
    #[doc = "1: External Reference selected as Buffer Input"]
    _1 = 1,
}
impl From<VREFBUFINSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VREFBUFINSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VREFBUFINSEL`"]
pub type VREFBUFINSEL_R = crate::R<bool, VREFBUFINSEL_A>;
impl VREFBUFINSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFBUFINSEL_A {
        match self.bits {
            false => VREFBUFINSEL_A::_0,
            true => VREFBUFINSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VREFBUFINSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VREFBUFINSEL_A::_1
    }
}
#[doc = "Write proxy for field `VREFBUFINSEL`"]
pub struct VREFBUFINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFBUFINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFBUFINSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal Reference selected as Buffer Input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VREFBUFINSEL_A::_0)
    }
    #[doc = "External Reference selected as Buffer Input"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VREFBUFINSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "VrefBuffer Power Down\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFBUFPD_A {
    #[doc = "0: Buffer Enabled"]
    _0 = 0,
    #[doc = "1: Buffer Powered Down"]
    _1 = 1,
}
impl From<VREFBUFPD_A> for bool {
    #[inline(always)]
    fn from(variant: VREFBUFPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VREFBUFPD`"]
pub type VREFBUFPD_R = crate::R<bool, VREFBUFPD_A>;
impl VREFBUFPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFBUFPD_A {
        match self.bits {
            false => VREFBUFPD_A::_0,
            true => VREFBUFPD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VREFBUFPD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VREFBUFPD_A::_1
    }
}
#[doc = "Write proxy for field `VREFBUFPD`"]
pub struct VREFBUFPD_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFBUFPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFBUFPD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Buffer Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VREFBUFPD_A::_0)
    }
    #[doc = "Buffer Powered Down"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VREFBUFPD_A::_1)
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
    #[doc = "Bit 0 - RTC oscillator status"]
    #[inline(always)]
    pub fn oscon(&self) -> OSCON_R {
        OSCON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PDB bypass XBAR as ADC trigger"]
    #[inline(always)]
    pub fn pdbadctrg(&self) -> PDBADCTRG_R {
        PDBADCTRG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - DMA Done select"]
    #[inline(always)]
    pub fn dmadonesel(&self) -> DMADONESEL_R {
        DMADONESEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - AFE Clock Source Select (SIMAFECLK selection)"]
    #[inline(always)]
    pub fn afeclksel(&self) -> AFECLKSEL_R {
        AFECLKSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - AFE Clock Pad Direction"]
    #[inline(always)]
    pub fn afeclkpaddir(&self) -> AFECLKPADDIR_R {
        AFECLKPADDIR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART Modulation Type"]
    #[inline(always)]
    pub fn uartmodtype(&self) -> UARTMODTYPE_R {
        UARTMODTYPE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART0 IrDA Select"]
    #[inline(always)]
    pub fn uart0irsel(&self) -> UART0IRSEL_R {
        UART0IRSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART1 IrDA Select"]
    #[inline(always)]
    pub fn uart1irsel(&self) -> UART1IRSEL_R {
        UART1IRSEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART2 IrDA Select"]
    #[inline(always)]
    pub fn uart2irsel(&self) -> UART2IRSEL_R {
        UART2IRSEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - UART3 IrDA Select"]
    #[inline(always)]
    pub fn uart3irsel(&self) -> UART3IRSEL_R {
        UART3IRSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - External Watchdog Monitor Input Select"]
    #[inline(always)]
    pub fn ewminsel(&self) -> EWMINSEL_R {
        EWMINSEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Timer CH0 PLL clock Select"]
    #[inline(always)]
    pub fn tmr0pllsel(&self) -> TMR0PLLSEL_R {
        TMR0PLLSEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Quadtimer Channel0 Secondary Count source Select"]
    #[inline(always)]
    pub fn tmr0scsel(&self) -> TMR0SCSEL_R {
        TMR0SCSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Quadtimer Channel1 Secondary Count source Select"]
    #[inline(always)]
    pub fn tmr1scsel(&self) -> TMR1SCSEL_R {
        TMR1SCSEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Quadtimer Channel2 Secondary Count source Select"]
    #[inline(always)]
    pub fn tmr2scsel(&self) -> TMR2SCSEL_R {
        TMR2SCSEL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Quadtimer Channel3 Secondary Count source Select"]
    #[inline(always)]
    pub fn tmr3scsel(&self) -> TMR3SCSEL_R {
        TMR3SCSEL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Quadtimer Channel0 Primary Count Source Select"]
    #[inline(always)]
    pub fn tmr0pcssel(&self) -> TMR0PCSSEL_R {
        TMR0PCSSEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Quadtimer Channel1 Primary Count Source Select"]
    #[inline(always)]
    pub fn tmr1pcssel(&self) -> TMR1PCSSEL_R {
        TMR1PCSSEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Quadtimer Channel2 Primary Count Source Select"]
    #[inline(always)]
    pub fn tmr2pcssel(&self) -> TMR2PCSSEL_R {
        TMR2PCSSEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Quadtimer Channel3 Primary Count Source Select"]
    #[inline(always)]
    pub fn tmr3pcssel(&self) -> TMR3PCSSEL_R {
        TMR3PCSSEL_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 28 - RTC Clock select"]
    #[inline(always)]
    pub fn rtcclksel(&self) -> RTCCLKSEL_R {
        RTCCLKSEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - VrefBuffer Output Enable"]
    #[inline(always)]
    pub fn vrefbufouten(&self) -> VREFBUFOUTEN_R {
        VREFBUFOUTEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - VrefBuffer Input Select"]
    #[inline(always)]
    pub fn vrefbufinsel(&self) -> VREFBUFINSEL_R {
        VREFBUFINSEL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - VrefBuffer Power Down"]
    #[inline(always)]
    pub fn vrefbufpd(&self) -> VREFBUFPD_R {
        VREFBUFPD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - PDB bypass XBAR as ADC trigger"]
    #[inline(always)]
    pub fn pdbadctrg(&mut self) -> PDBADCTRG_W {
        PDBADCTRG_W { w: self }
    }
    #[doc = "Bits 2:3 - DMA Done select"]
    #[inline(always)]
    pub fn dmadonesel(&mut self) -> DMADONESEL_W {
        DMADONESEL_W { w: self }
    }
    #[doc = "Bits 4:5 - AFE Clock Source Select (SIMAFECLK selection)"]
    #[inline(always)]
    pub fn afeclksel(&mut self) -> AFECLKSEL_W {
        AFECLKSEL_W { w: self }
    }
    #[doc = "Bit 6 - AFE Clock Pad Direction"]
    #[inline(always)]
    pub fn afeclkpaddir(&mut self) -> AFECLKPADDIR_W {
        AFECLKPADDIR_W { w: self }
    }
    #[doc = "Bit 7 - UART Modulation Type"]
    #[inline(always)]
    pub fn uartmodtype(&mut self) -> UARTMODTYPE_W {
        UARTMODTYPE_W { w: self }
    }
    #[doc = "Bit 8 - UART0 IrDA Select"]
    #[inline(always)]
    pub fn uart0irsel(&mut self) -> UART0IRSEL_W {
        UART0IRSEL_W { w: self }
    }
    #[doc = "Bit 9 - UART1 IrDA Select"]
    #[inline(always)]
    pub fn uart1irsel(&mut self) -> UART1IRSEL_W {
        UART1IRSEL_W { w: self }
    }
    #[doc = "Bit 10 - UART2 IrDA Select"]
    #[inline(always)]
    pub fn uart2irsel(&mut self) -> UART2IRSEL_W {
        UART2IRSEL_W { w: self }
    }
    #[doc = "Bit 11 - UART3 IrDA Select"]
    #[inline(always)]
    pub fn uart3irsel(&mut self) -> UART3IRSEL_W {
        UART3IRSEL_W { w: self }
    }
    #[doc = "Bit 14 - External Watchdog Monitor Input Select"]
    #[inline(always)]
    pub fn ewminsel(&mut self) -> EWMINSEL_W {
        EWMINSEL_W { w: self }
    }
    #[doc = "Bit 15 - Timer CH0 PLL clock Select"]
    #[inline(always)]
    pub fn tmr0pllsel(&mut self) -> TMR0PLLSEL_W {
        TMR0PLLSEL_W { w: self }
    }
    #[doc = "Bit 16 - Quadtimer Channel0 Secondary Count source Select"]
    #[inline(always)]
    pub fn tmr0scsel(&mut self) -> TMR0SCSEL_W {
        TMR0SCSEL_W { w: self }
    }
    #[doc = "Bit 17 - Quadtimer Channel1 Secondary Count source Select"]
    #[inline(always)]
    pub fn tmr1scsel(&mut self) -> TMR1SCSEL_W {
        TMR1SCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Quadtimer Channel2 Secondary Count source Select"]
    #[inline(always)]
    pub fn tmr2scsel(&mut self) -> TMR2SCSEL_W {
        TMR2SCSEL_W { w: self }
    }
    #[doc = "Bit 19 - Quadtimer Channel3 Secondary Count source Select"]
    #[inline(always)]
    pub fn tmr3scsel(&mut self) -> TMR3SCSEL_W {
        TMR3SCSEL_W { w: self }
    }
    #[doc = "Bits 20:21 - Quadtimer Channel0 Primary Count Source Select"]
    #[inline(always)]
    pub fn tmr0pcssel(&mut self) -> TMR0PCSSEL_W {
        TMR0PCSSEL_W { w: self }
    }
    #[doc = "Bits 22:23 - Quadtimer Channel1 Primary Count Source Select"]
    #[inline(always)]
    pub fn tmr1pcssel(&mut self) -> TMR1PCSSEL_W {
        TMR1PCSSEL_W { w: self }
    }
    #[doc = "Bits 24:25 - Quadtimer Channel2 Primary Count Source Select"]
    #[inline(always)]
    pub fn tmr2pcssel(&mut self) -> TMR2PCSSEL_W {
        TMR2PCSSEL_W { w: self }
    }
    #[doc = "Bits 26:27 - Quadtimer Channel3 Primary Count Source Select"]
    #[inline(always)]
    pub fn tmr3pcssel(&mut self) -> TMR3PCSSEL_W {
        TMR3PCSSEL_W { w: self }
    }
    #[doc = "Bit 28 - RTC Clock select"]
    #[inline(always)]
    pub fn rtcclksel(&mut self) -> RTCCLKSEL_W {
        RTCCLKSEL_W { w: self }
    }
    #[doc = "Bit 29 - VrefBuffer Output Enable"]
    #[inline(always)]
    pub fn vrefbufouten(&mut self) -> VREFBUFOUTEN_W {
        VREFBUFOUTEN_W { w: self }
    }
    #[doc = "Bit 30 - VrefBuffer Input Select"]
    #[inline(always)]
    pub fn vrefbufinsel(&mut self) -> VREFBUFINSEL_W {
        VREFBUFINSEL_W { w: self }
    }
    #[doc = "Bit 31 - VrefBuffer Power Down"]
    #[inline(always)]
    pub fn vrefbufpd(&mut self) -> VREFBUFPD_W {
        VREFBUFPD_W { w: self }
    }
}
