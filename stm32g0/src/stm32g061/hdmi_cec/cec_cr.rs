#[doc = "Reader of register CEC_CR"]
pub type R = crate::R<u32, super::CEC_CR>;
#[doc = "Writer for register CEC_CR"]
pub type W = crate::W<u32, super::CEC_CR>;
#[doc = "Register CEC_CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CEC_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CEC enable The CECEN bit is set and cleared by software. CECEN = 1 starts message reception and enables the TXSOM control. CECEN = 0 disables the CEC peripheral, clears all bits of CEC_CR register and aborts any on-going reception or transmission.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CECEN_A {
    #[doc = "0: CEC peripheral is off."]
    B_0X0 = 0,
    #[doc = "1: CEC peripheral is on."]
    B_0X1 = 1,
}
impl From<CECEN_A> for bool {
    #[inline(always)]
    fn from(variant: CECEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CECEN`"]
pub type CECEN_R = crate::R<bool, CECEN_A>;
impl CECEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CECEN_A {
        match self.bits {
            false => CECEN_A::B_0X0,
            true => CECEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CECEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CECEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `CECEN`"]
pub struct CECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CECEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CECEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CEC peripheral is off."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CECEN_A::B_0X0)
    }
    #[doc = "CEC peripheral is on."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CECEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Tx start of message TXSOM is set by software to command transmission of the first byte of a CEC message. If the CEC message consists of only one byte, TXEOM must be set before of TXSOM. Start-bit is effectively started on the CEC line after SFT is counted. If TXSOM is set while a message reception is ongoing, transmission starts after the end of reception. TXSOM is cleared by hardware after the last byte of the message is sent with a positive acknowledge (TXEND = 1), in case of transmission underrun (TXUDR = 1), negative acknowledge (TXACKE = 1), and transmission error (TXERR = 1). It is also cleared by CECEN = 0. It is not cleared and transmission is automatically retried in case of arbitration lost (ARBLST = 1). TXSOM can be also used as a status bit informing application whether any transmission request is pending or under execution. The application can abort a transmission request at any time by clearing the CECEN bit. Note: TXSOM must be set when CECEN = 1. TXSOM must be set when transmission data is available into TXDR. HEADER first four bits containing own peripheral address are taken from TXDR\\[7:4\\], not from CEC_CFGR.OAR that is used only for reception.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSOM_A {
    #[doc = "0: No CEC transmission is on-going"]
    B_0X0 = 0,
    #[doc = "1: CEC transmission command"]
    B_0X1 = 1,
}
impl From<TXSOM_A> for bool {
    #[inline(always)]
    fn from(variant: TXSOM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXSOM`"]
pub type TXSOM_R = crate::R<bool, TXSOM_A>;
impl TXSOM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSOM_A {
        match self.bits {
            false => TXSOM_A::B_0X0,
            true => TXSOM_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXSOM_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXSOM_A::B_0X1
    }
}
#[doc = "Write proxy for field `TXSOM`"]
pub struct TXSOM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSOM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No CEC transmission is on-going"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXSOM_A::B_0X0)
    }
    #[doc = "CEC transmission command"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXSOM_A::B_0X1)
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
#[doc = "Tx end of message The TXEOM bit is set by software to command transmission of the last byte of a CEC message. TXEOM is cleared by hardware at the same time and under the same conditions as for TXSOM. Note: TXEOM must be set when CECEN = 1. TXEOM must be set before writing transmission data to TXDR. If TXEOM is set when TXSOM = 0, transmitted message consists of 1 byte (HEADER) only (PING message).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEOM_A {
    #[doc = "0: TXDR data byte is transmitted with EOM = 0  "]
    B_0X0 = 0,
    #[doc = "1: TXDR data byte is transmitted with EOM = 1  "]
    B_0X1 = 1,
}
impl From<TXEOM_A> for bool {
    #[inline(always)]
    fn from(variant: TXEOM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXEOM`"]
pub type TXEOM_R = crate::R<bool, TXEOM_A>;
impl TXEOM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEOM_A {
        match self.bits {
            false => TXEOM_A::B_0X0,
            true => TXEOM_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXEOM_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXEOM_A::B_0X1
    }
}
#[doc = "Write proxy for field `TXEOM`"]
pub struct TXEOM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXEOM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TXDR data byte is transmitted with EOM = 0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXEOM_A::B_0X0)
    }
    #[doc = "TXDR data byte is transmitted with EOM = 1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXEOM_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CEC enable The CECEN bit is set and cleared by software. CECEN = 1 starts message reception and enables the TXSOM control. CECEN = 0 disables the CEC peripheral, clears all bits of CEC_CR register and aborts any on-going reception or transmission."]
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Tx start of message TXSOM is set by software to command transmission of the first byte of a CEC message. If the CEC message consists of only one byte, TXEOM must be set before of TXSOM. Start-bit is effectively started on the CEC line after SFT is counted. If TXSOM is set while a message reception is ongoing, transmission starts after the end of reception. TXSOM is cleared by hardware after the last byte of the message is sent with a positive acknowledge (TXEND = 1), in case of transmission underrun (TXUDR = 1), negative acknowledge (TXACKE = 1), and transmission error (TXERR = 1). It is also cleared by CECEN = 0. It is not cleared and transmission is automatically retried in case of arbitration lost (ARBLST = 1). TXSOM can be also used as a status bit informing application whether any transmission request is pending or under execution. The application can abort a transmission request at any time by clearing the CECEN bit. Note: TXSOM must be set when CECEN = 1. TXSOM must be set when transmission data is available into TXDR. HEADER first four bits containing own peripheral address are taken from TXDR\\[7:4\\], not from CEC_CFGR.OAR that is used only for reception."]
    #[inline(always)]
    pub fn txsom(&self) -> TXSOM_R {
        TXSOM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Tx end of message The TXEOM bit is set by software to command transmission of the last byte of a CEC message. TXEOM is cleared by hardware at the same time and under the same conditions as for TXSOM. Note: TXEOM must be set when CECEN = 1. TXEOM must be set before writing transmission data to TXDR. If TXEOM is set when TXSOM = 0, transmitted message consists of 1 byte (HEADER) only (PING message)."]
    #[inline(always)]
    pub fn txeom(&self) -> TXEOM_R {
        TXEOM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CEC enable The CECEN bit is set and cleared by software. CECEN = 1 starts message reception and enables the TXSOM control. CECEN = 0 disables the CEC peripheral, clears all bits of CEC_CR register and aborts any on-going reception or transmission."]
    #[inline(always)]
    pub fn cecen(&mut self) -> CECEN_W {
        CECEN_W { w: self }
    }
    #[doc = "Bit 1 - Tx start of message TXSOM is set by software to command transmission of the first byte of a CEC message. If the CEC message consists of only one byte, TXEOM must be set before of TXSOM. Start-bit is effectively started on the CEC line after SFT is counted. If TXSOM is set while a message reception is ongoing, transmission starts after the end of reception. TXSOM is cleared by hardware after the last byte of the message is sent with a positive acknowledge (TXEND = 1), in case of transmission underrun (TXUDR = 1), negative acknowledge (TXACKE = 1), and transmission error (TXERR = 1). It is also cleared by CECEN = 0. It is not cleared and transmission is automatically retried in case of arbitration lost (ARBLST = 1). TXSOM can be also used as a status bit informing application whether any transmission request is pending or under execution. The application can abort a transmission request at any time by clearing the CECEN bit. Note: TXSOM must be set when CECEN = 1. TXSOM must be set when transmission data is available into TXDR. HEADER first four bits containing own peripheral address are taken from TXDR\\[7:4\\], not from CEC_CFGR.OAR that is used only for reception."]
    #[inline(always)]
    pub fn txsom(&mut self) -> TXSOM_W {
        TXSOM_W { w: self }
    }
    #[doc = "Bit 2 - Tx end of message The TXEOM bit is set by software to command transmission of the last byte of a CEC message. TXEOM is cleared by hardware at the same time and under the same conditions as for TXSOM. Note: TXEOM must be set when CECEN = 1. TXEOM must be set before writing transmission data to TXDR. If TXEOM is set when TXSOM = 0, transmitted message consists of 1 byte (HEADER) only (PING message)."]
    #[inline(always)]
    pub fn txeom(&mut self) -> TXEOM_W {
        TXEOM_W { w: self }
    }
}
