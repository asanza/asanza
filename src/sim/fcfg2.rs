#[doc = "Reader of register FCFG2"]
pub type R = crate::R<u32, super::FCFG2>;
#[doc = "Reader of field `MAXADDR`"]
pub type MAXADDR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:30 - Max address"]
    #[inline(always)]
    pub fn maxaddr(&self) -> MAXADDR_R {
        MAXADDR_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
