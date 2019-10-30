#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IF`"]
pub type IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF`"]
pub struct IF_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_W<'a> {
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
#[doc = "Reader of field `TIP`"]
pub type TIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIP`"]
pub struct TIP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIP_W<'a> {
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
#[doc = "Reader of field `Resvd`"]
pub type RESVD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Resvd`"]
pub struct RESVD_W<'a> {
    w: &'a mut W,
}
impl<'a> RESVD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `ArbLost`"]
pub type ARBLOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ArbLost`"]
pub struct ARBLOST_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBLOST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `Busy`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Busy`"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
#[doc = "Reader of field `RxACK`"]
pub type RXACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RxACK`"]
pub struct RXACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RXACK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Interrupt flag, This bit is set when an interrupt is pending, which will cause a processor interrupt request if the IEN bit is set. The Interrupt Flag is set upon the completion of one byte of data transfer."]
    #[inline(always)]
    pub fn if_(&self) -> IF_R {
        IF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - transfer in progress"]
    #[inline(always)]
    pub fn tip(&self) -> TIP_R {
        TIP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - reserved for cross-compatibility with OpenCores drivers"]
    #[inline(always)]
    pub fn resvd(&self) -> RESVD_R {
        RESVD_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 5 - Set when arbitration for the bus is lost"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARBLOST_R {
        ARBLOST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C block is busy processing the latest command"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Received acknowledge from slave. 1 = no ack received, 0 = ack received"]
    #[inline(always)]
    pub fn rx_ack(&self) -> RXACK_R {
        RXACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag, This bit is set when an interrupt is pending, which will cause a processor interrupt request if the IEN bit is set. The Interrupt Flag is set upon the completion of one byte of data transfer."]
    #[inline(always)]
    pub fn if_(&mut self) -> IF_W {
        IF_W { w: self }
    }
    #[doc = "Bit 1 - transfer in progress"]
    #[inline(always)]
    pub fn tip(&mut self) -> TIP_W {
        TIP_W { w: self }
    }
    #[doc = "Bits 2:4 - reserved for cross-compatibility with OpenCores drivers"]
    #[inline(always)]
    pub fn resvd(&mut self) -> RESVD_W {
        RESVD_W { w: self }
    }
    #[doc = "Bit 5 - Set when arbitration for the bus is lost"]
    #[inline(always)]
    pub fn arb_lost(&mut self) -> ARBLOST_W {
        ARBLOST_W { w: self }
    }
    #[doc = "Bit 6 - I2C block is busy processing the latest command"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bit 7 - Received acknowledge from slave. 1 = no ack received, 0 = ack received"]
    #[inline(always)]
    pub fn rx_ack(&mut self) -> RXACK_W {
        RXACK_W { w: self }
    }
}
