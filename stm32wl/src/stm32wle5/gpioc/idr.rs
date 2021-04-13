#[doc = "Reader of register IDR"]
pub type R = crate::R<u32, super::IDR>;
#[doc = "Reader of field `IDR15`"]
pub type IDR15_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDR14`"]
pub type IDR14_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDR13`"]
pub type IDR13_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDR6`"]
pub type IDR6_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDR5`"]
pub type IDR5_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDR4`"]
pub type IDR4_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDR3`"]
pub type IDR3_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDR2`"]
pub type IDR2_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDR1`"]
pub type IDR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDR0`"]
pub type IDR0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 15 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr15(&self) -> IDR15_R {
        IDR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr14(&self) -> IDR14_R {
        IDR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr13(&self) -> IDR13_R {
        IDR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr6(&self) -> IDR6_R {
        IDR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr5(&self) -> IDR5_R {
        IDR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr4(&self) -> IDR4_R {
        IDR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr3(&self) -> IDR3_R {
        IDR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr2(&self) -> IDR2_R {
        IDR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr1(&self) -> IDR1_R {
        IDR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr0(&self) -> IDR0_R {
        IDR0_R::new((self.bits & 0x01) != 0)
    }
}
