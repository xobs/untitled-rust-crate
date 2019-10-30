#[doc = "Reader of register STAT4"]
pub type R = crate::R<u32, super::STAT4>;
#[doc = "Writer for register STAT4"]
pub type W = crate::W<u32, super::STAT4>;
#[doc = "Register STAT4 `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `stat4`"]
pub type STAT4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `stat4`"]
pub struct STAT4_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT4_W<'a> {
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
    pub fn stat4(&self) -> STAT4_R {
        STAT4_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn stat4(&mut self) -> STAT4_W {
        STAT4_W { w: self }
    }
}
