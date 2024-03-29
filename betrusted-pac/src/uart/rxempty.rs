#[doc = "Reader of register RXEMPTY"]
pub type R = crate::R<u32, super::RXEMPTY>;
#[doc = "Writer for register RXEMPTY"]
pub type W = crate::W<u32, super::RXEMPTY>;
#[doc = "Register RXEMPTY `reset()`'s with value 0"]
impl crate::ResetValue for super::RXEMPTY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rxempty`"]
pub type RXEMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rxempty`"]
pub struct RXEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEMPTY_W<'a> {
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
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxempty(&mut self) -> RXEMPTY_W {
        RXEMPTY_W { w: self }
    }
}
