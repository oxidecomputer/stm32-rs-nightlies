#[doc = "Reader of register IDR"]
pub type R = crate::R<u32, super::IDR>;
#[doc = "Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDR0_A {
    #[doc = "1: Input is logic high"]
    HIGH = 1,
    #[doc = "0: Input is logic low"]
    LOW = 0,
}
impl From<IDR0_A> for bool {
    #[inline(always)]
    fn from(variant: IDR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IDR0`"]
pub type IDR0_R = crate::R<bool, IDR0_A>;
impl IDR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDR0_A {
        match self.bits {
            true => IDR0_A::HIGH,
            false => IDR0_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDR0_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDR0_A::LOW
    }
}
#[doc = "Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
pub type IDR1_A = IDR0_A;
#[doc = "Reader of field `IDR1`"]
pub type IDR1_R = crate::R<bool, IDR0_A>;
#[doc = "Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
pub type IDR2_A = IDR0_A;
#[doc = "Reader of field `IDR2`"]
pub type IDR2_R = crate::R<bool, IDR0_A>;
#[doc = "Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
pub type IDR3_A = IDR0_A;
#[doc = "Reader of field `IDR3`"]
pub type IDR3_R = crate::R<bool, IDR0_A>;
#[doc = "Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
pub type IDR4_A = IDR0_A;
#[doc = "Reader of field `IDR4`"]
pub type IDR4_R = crate::R<bool, IDR0_A>;
#[doc = "Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
pub type IDR5_A = IDR0_A;
#[doc = "Reader of field `IDR5`"]
pub type IDR5_R = crate::R<bool, IDR0_A>;
#[doc = "Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
pub type IDR6_A = IDR0_A;
#[doc = "Reader of field `IDR6`"]
pub type IDR6_R = crate::R<bool, IDR0_A>;
#[doc = "Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
pub type IDR7_A = IDR0_A;
#[doc = "Reader of field `IDR7`"]
pub type IDR7_R = crate::R<bool, IDR0_A>;
#[doc = "Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
pub type IDR8_A = IDR0_A;
#[doc = "Reader of field `IDR8`"]
pub type IDR8_R = crate::R<bool, IDR0_A>;
#[doc = "Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
pub type IDR9_A = IDR0_A;
#[doc = "Reader of field `IDR9`"]
pub type IDR9_R = crate::R<bool, IDR0_A>;
#[doc = "Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
pub type IDR10_A = IDR0_A;
#[doc = "Reader of field `IDR10`"]
pub type IDR10_R = crate::R<bool, IDR0_A>;
#[doc = "Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
pub type IDR11_A = IDR0_A;
#[doc = "Reader of field `IDR11`"]
pub type IDR11_R = crate::R<bool, IDR0_A>;
#[doc = "Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
pub type IDR12_A = IDR0_A;
#[doc = "Reader of field `IDR12`"]
pub type IDR12_R = crate::R<bool, IDR0_A>;
#[doc = "Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
pub type IDR13_A = IDR0_A;
#[doc = "Reader of field `IDR13`"]
pub type IDR13_R = crate::R<bool, IDR0_A>;
#[doc = "Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
pub type IDR14_A = IDR0_A;
#[doc = "Reader of field `IDR14`"]
pub type IDR14_R = crate::R<bool, IDR0_A>;
#[doc = "Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
pub type IDR15_A = IDR0_A;
#[doc = "Reader of field `IDR15`"]
pub type IDR15_R = crate::R<bool, IDR0_A>;
impl R {
    #[doc = "Bit 0 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
    #[inline(always)]
    pub fn idr0(&self) -> IDR0_R {
        IDR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
    #[inline(always)]
    pub fn idr1(&self) -> IDR1_R {
        IDR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
    #[inline(always)]
    pub fn idr2(&self) -> IDR2_R {
        IDR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
    #[inline(always)]
    pub fn idr3(&self) -> IDR3_R {
        IDR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
    #[inline(always)]
    pub fn idr4(&self) -> IDR4_R {
        IDR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
    #[inline(always)]
    pub fn idr5(&self) -> IDR5_R {
        IDR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
    #[inline(always)]
    pub fn idr6(&self) -> IDR6_R {
        IDR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
    #[inline(always)]
    pub fn idr7(&self) -> IDR7_R {
        IDR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
    #[inline(always)]
    pub fn idr8(&self) -> IDR8_R {
        IDR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
    #[inline(always)]
    pub fn idr9(&self) -> IDR9_R {
        IDR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
    #[inline(always)]
    pub fn idr10(&self) -> IDR10_R {
        IDR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
    #[inline(always)]
    pub fn idr11(&self) -> IDR11_R {
        IDR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
    #[inline(always)]
    pub fn idr12(&self) -> IDR12_R {
        IDR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
    #[inline(always)]
    pub fn idr13(&self) -> IDR13_R {
        IDR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
    #[inline(always)]
    pub fn idr14(&self) -> IDR14_R {
        IDR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port."]
    #[inline(always)]
    pub fn idr15(&self) -> IDR15_R {
        IDR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
