#[doc = "Reader of register CONTROL"]
pub type R = crate::R<u32, super::CONTROL>;
#[doc = "Writer for register CONTROL"]
pub type W = crate::W<u32, super::CONTROL>;
#[doc = "Register CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `IEN`"]
pub type IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEN`"]
pub struct IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IEN_W<'a> {
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
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
    #[doc = "Bits 0:5 - Reserved (for cross-compatibility with OpenCores drivers)"]
    #[inline(always)]
    pub fn resvd(&self) -> RESVD_R {
        RESVD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - When set to `1`, interrupts are enabled."]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When set to `1`, the core is enabled."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Reserved (for cross-compatibility with OpenCores drivers)"]
    #[inline(always)]
    pub fn resvd(&mut self) -> RESVD_W {
        RESVD_W { w: self }
    }
    #[doc = "Bit 6 - When set to `1`, interrupts are enabled."]
    #[inline(always)]
    pub fn ien(&mut self) -> IEN_W {
        IEN_W { w: self }
    }
    #[doc = "Bit 7 - When set to `1`, the core is enabled."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
