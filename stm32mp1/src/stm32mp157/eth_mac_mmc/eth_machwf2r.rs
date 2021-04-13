#[doc = "Reader of register ETH_MACHWF2R"]
pub type R = crate::R<u32, super::ETH_MACHWF2R>;
#[doc = "Reader of field `RXQCNT`"]
pub type RXQCNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXQCNT`"]
pub type TXQCNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXCHCNT`"]
pub type RXCHCNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXCHCNT`"]
pub type TXCHCNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `PPSOUTNUM`"]
pub type PPSOUTNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `AUXSNAPNUM`"]
pub type AUXSNAPNUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - RXQCNT"]
    #[inline(always)]
    pub fn rxqcnt(&self) -> RXQCNT_R {
        RXQCNT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - TXQCNT"]
    #[inline(always)]
    pub fn txqcnt(&self) -> TXQCNT_R {
        TXQCNT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - RXCHCNT"]
    #[inline(always)]
    pub fn rxchcnt(&self) -> RXCHCNT_R {
        RXCHCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - TXCHCNT"]
    #[inline(always)]
    pub fn txchcnt(&self) -> TXCHCNT_R {
        TXCHCNT_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - PPSOUTNUM"]
    #[inline(always)]
    pub fn ppsoutnum(&self) -> PPSOUTNUM_R {
        PPSOUTNUM_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - AUXSNAPNUM"]
    #[inline(always)]
    pub fn auxsnapnum(&self) -> AUXSNAPNUM_R {
        AUXSNAPNUM_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
