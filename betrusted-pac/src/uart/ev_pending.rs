#[doc = "Reader of register EV_PENDING"]
pub type R = crate::R<u32, super::EV_PENDING>;
#[doc = "Writer for register EV_PENDING"]
pub type W = crate::W<u32, super::EV_PENDING>;
#[doc = "Register EV_PENDING `reset()`'s with value 0"]
impl crate::ResetValue for super::EV_PENDING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pending`"]
pub type PENDING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pending`"]
pub struct PENDING_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pending(&mut self) -> PENDING_W {
        PENDING_W { w: self }
    }
}
