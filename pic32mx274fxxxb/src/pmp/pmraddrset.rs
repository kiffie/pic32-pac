#[doc = "Reader of register PMRADDRSET"]
pub type R = crate::R<u32, super::PMRADDRSET>;
#[doc = "Writer for register PMRADDRSET"]
pub type W = crate::W<u32, super::PMRADDRSET>;
#[doc = "Register PMRADDRSET `reset()`'s with value 0"]
impl crate::ResetValue for super::PMRADDRSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RADDR`"]
pub type RADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RADDR`"]
pub struct RADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Reader of field `RADDR14`"]
pub type RADDR14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RADDR14`"]
pub struct RADDR14_W<'a> {
    w: &'a mut W,
}
impl<'a> RADDR14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `RADDR15`"]
pub type RADDR15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RADDR15`"]
pub struct RADDR15_W<'a> {
    w: &'a mut W,
}
impl<'a> RADDR15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn raddr(&self) -> RADDR_R {
        RADDR_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn raddr14(&self) -> RADDR14_R {
        RADDR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn raddr15(&self) -> RADDR15_R {
        RADDR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn raddr(&mut self) -> RADDR_W {
        RADDR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn raddr14(&mut self) -> RADDR14_W {
        RADDR14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn raddr15(&mut self) -> RADDR15_W {
        RADDR15_W { w: self }
    }
}
