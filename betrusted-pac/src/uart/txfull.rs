#[doc = "Reader of register TXFULL"]
pub type R = crate::R<u32, super::TXFULL>;
#[doc = "Writer for register TXFULL"]
pub type W = crate::W<u32, super::TXFULL>;
#[doc = "Register TXFULL `reset()`'s with value 0"]
impl crate::ResetValue for super::TXFULL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `txfull`"]
pub type TXFULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `txfull`"]
pub struct TXFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFULL_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txfull(&mut self) -> TXFULL_W {
        TXFULL_W { w: self }
    }
}
