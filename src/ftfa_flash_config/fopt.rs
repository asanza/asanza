#[doc = "Reader of register FOPT"]
pub type R = crate::R<u8, super::FOPT>;
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPBOOT_A {
    #[doc = "0: Low-power boot"]
    _00 = 0,
    #[doc = "1: Normal boot"]
    _01 = 1,
}
impl From<LPBOOT_A> for bool {
    #[inline(always)]
    fn from(variant: LPBOOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPBOOT`"]
pub type LPBOOT_R = crate::R<bool, LPBOOT_A>;
impl LPBOOT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPBOOT_A {
        match self.bits {
            false => LPBOOT_A::_00,
            true => LPBOOT_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPBOOT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPBOOT_A::_01
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMI_EN_A {
    #[doc = "0: NMI interrupts are always blocked"]
    _00 = 0,
    #[doc = "1: NMI_b pin/interrupts reset default to enabled"]
    _01 = 1,
}
impl From<NMI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: NMI_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NMI_EN`"]
pub type NMI_EN_R = crate::R<bool, NMI_EN_A>;
impl NMI_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMI_EN_A {
        match self.bits {
            false => NMI_EN_A::_00,
            true => NMI_EN_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NMI_EN_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NMI_EN_A::_01
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXE_MODE_A {
    #[doc = "0: Execution Mode is RUN Mode"]
    _00 = 0,
    #[doc = "1: Execution Mode is VLPR Mode"]
    _01 = 1,
}
impl From<EXE_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: EXE_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXE_MODE`"]
pub type EXE_MODE_R = crate::R<bool, EXE_MODE_A>;
impl EXE_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXE_MODE_A {
        match self.bits {
            false => EXE_MODE_A::_00,
            true => EXE_MODE_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == EXE_MODE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == EXE_MODE_A::_01
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_SRC_A {
    #[doc = "0: Externally supplied clock used by Flash"]
    _00 = 0,
    #[doc = "1: Internal clock source used by Flash"]
    _01 = 1,
}
impl From<CLK_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLK_SRC`"]
pub type CLK_SRC_R = crate::R<bool, CLK_SRC_A>;
impl CLK_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_SRC_A {
        match self.bits {
            false => CLK_SRC_A::_00,
            true => CLK_SRC_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CLK_SRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CLK_SRC_A::_01
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn lpboot(&self) -> LPBOOT_R {
        LPBOOT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn nmi_en(&self) -> NMI_EN_R {
        NMI_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn exe_mode(&self) -> EXE_MODE_R {
        EXE_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn clk_src(&self) -> CLK_SRC_R {
        CLK_SRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
