#[doc = "Reader of register ADDR1"]
pub type R = crate::R<u32, super::ADDR1>;
#[doc = "Writer for register ADDR1"]
pub type W = crate::W<u32, super::ADDR1>;
#[doc = "Register ADDR1 `reset()`'s with value 0x20"]
impl crate::ResetValue for super::ADDR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
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
