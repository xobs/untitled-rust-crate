#[doc = "Reader of register MAJOR"]
pub type R = crate::R<u32, super::MAJOR>;
#[doc = "Writer for register MAJOR"]
pub type W = crate::W<u32, super::MAJOR>;
#[doc = "Register MAJOR `reset()`'s with value 0"]
impl crate::ResetValue for super::MAJOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `major`"]
pub type MAJOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `major`"]
pub struct MAJOR_W<'a> {
    w: &'a mut W,
}
impl<'a> MAJOR_W<'a> {
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
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn major(&mut self) -> MAJOR_W {
        MAJOR_W { w: self }
    }
}
