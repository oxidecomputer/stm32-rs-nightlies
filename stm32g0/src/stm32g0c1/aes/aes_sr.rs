#[doc = "Reader of register AES_SR"]
pub type R = crate::R<u32, super::AES_SR>;
#[doc = "Computation completed flag This flag indicates whether the computation is completed: The flag is set by hardware upon the completion of the computation. It is cleared by software, upon setting the CCFC bit of the AES_CR register. Upon the flag setting, an interrupt is generated if enabled through the CCFIE bit of the AES_CR register. The flag is significant only when the DMAOUTEN bit is 0. It may stay high when DMA_EN is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCF_A {
    #[doc = "0: Not completed"]
    B_0X0 = 0,
    #[doc = "1: Completed"]
    B_0X1 = 1,
}
impl From<CCF_A> for bool {
    #[inline(always)]
    fn from(variant: CCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCF`"]
pub type CCF_R = crate::R<bool, CCF_A>;
impl CCF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCF_A {
        match self.bits {
            false => CCF_A::B_0X0,
            true => CCF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CCF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CCF_A::B_0X1
    }
}
#[doc = "Read error flag This flag indicates the detection of an unexpected read operation from the AES_DOUTR register (during computation or data input phase): The flag is set by hardware. It is cleared by software upon setting the ERRC bit of the AES_CR register. Upon the flag setting, an interrupt is generated if enabled through the ERRIE bit of the AES_CR register. The flag setting has no impact on the AES operation. Unexpected read returns zero.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDERR_A {
    #[doc = "0: Not detected"]
    B_0X0 = 0,
    #[doc = "1: Detected"]
    B_0X1 = 1,
}
impl From<RDERR_A> for bool {
    #[inline(always)]
    fn from(variant: RDERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDERR`"]
pub type RDERR_R = crate::R<bool, RDERR_A>;
impl RDERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDERR_A {
        match self.bits {
            false => RDERR_A::B_0X0,
            true => RDERR_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RDERR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RDERR_A::B_0X1
    }
}
#[doc = "Write error This flag indicates the detection of an unexpected write operation to the AES_DINR register (during computation or data output phase): The flag is set by hardware. It is cleared by software upon setting the ERRC bit of the AES_CR register. Upon the flag setting, an interrupt is generated if enabled through the ERRIE bit of the AES_CR register. The flag setting has no impact on the AES operation. Unexpected write is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRERR_A {
    #[doc = "0: Not detected"]
    B_0X0 = 0,
    #[doc = "1: Detected"]
    B_0X1 = 1,
}
impl From<WRERR_A> for bool {
    #[inline(always)]
    fn from(variant: WRERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WRERR`"]
pub type WRERR_R = crate::R<bool, WRERR_A>;
impl WRERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRERR_A {
        match self.bits {
            false => WRERR_A::B_0X0,
            true => WRERR_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WRERR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WRERR_A::B_0X1
    }
}
#[doc = "Busy This flag indicates whether AES is idle or busy during GCM payload encryption phase: When the flag indicates “idle”, the current GCM encryption processing may be suspended to process a higher-priority message. In other chaining modes, or in GCM phases other than payload encryption, the flag must be ignored for the suspend process.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: Idle"]
    B_0X0 = 0,
    #[doc = "1: Busy"]
    B_0X1 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::B_0X0,
            true => BUSY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BUSY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BUSY_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - Computation completed flag This flag indicates whether the computation is completed: The flag is set by hardware upon the completion of the computation. It is cleared by software, upon setting the CCFC bit of the AES_CR register. Upon the flag setting, an interrupt is generated if enabled through the CCFIE bit of the AES_CR register. The flag is significant only when the DMAOUTEN bit is 0. It may stay high when DMA_EN is 1."]
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read error flag This flag indicates the detection of an unexpected read operation from the AES_DOUTR register (during computation or data input phase): The flag is set by hardware. It is cleared by software upon setting the ERRC bit of the AES_CR register. Upon the flag setting, an interrupt is generated if enabled through the ERRIE bit of the AES_CR register. The flag setting has no impact on the AES operation. Unexpected read returns zero."]
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write error This flag indicates the detection of an unexpected write operation to the AES_DINR register (during computation or data output phase): The flag is set by hardware. It is cleared by software upon setting the ERRC bit of the AES_CR register. Upon the flag setting, an interrupt is generated if enabled through the ERRIE bit of the AES_CR register. The flag setting has no impact on the AES operation. Unexpected write is ignored."]
    #[inline(always)]
    pub fn wrerr(&self) -> WRERR_R {
        WRERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Busy This flag indicates whether AES is idle or busy during GCM payload encryption phase: When the flag indicates “idle”, the current GCM encryption processing may be suspended to process a higher-priority message. In other chaining modes, or in GCM phases other than payload encryption, the flag must be ignored for the suspend process."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
