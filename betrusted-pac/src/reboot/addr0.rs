#[doc = "Reader of register ADDR0"]
pub type R = crate::R<u32, super::ADDR0>;
#[doc = "Writer for register ADDR0"]
pub type W = crate::W<u32, super::ADDR0>;
#[doc = "Register ADDR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `addr`"]
pub type ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `addr`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
}
