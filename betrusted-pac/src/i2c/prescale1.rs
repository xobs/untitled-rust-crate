#[doc = "Reader of register PRESCALE1"]
pub type R = crate::R<u32, super::PRESCALE1>;
#[doc = "Writer for register PRESCALE1"]
pub type W = crate::W<u32, super::PRESCALE1>;
#[doc = "Register PRESCALE1 `reset()`'s with value 0x7f"]
impl crate::ResetValue for super::PRESCALE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7f
    }
}
#[doc = "Reader of field `prescale`"]
pub type PRESCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `prescale`"]
pub struct PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_W<'a> {
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
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W {
        PRESCALE_W { w: self }
    }
}
