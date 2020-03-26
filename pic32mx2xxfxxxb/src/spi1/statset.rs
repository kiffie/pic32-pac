#[doc = "Reader of register STATSET"]
pub type R = crate::R<u32, super::STATSET>;
#[doc = "Writer for register STATSET"]
pub type W = crate::W<u32, super::STATSET>;
#[doc = "Register STATSET `reset()`'s with value 0"]
impl crate::ResetValue for super::STATSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPIRBF`"]
pub type SPIRBF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIRBF`"]
pub struct SPIRBF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIRBF_W<'a> {
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
#[doc = "Reader of field `SPITBF`"]
pub type SPITBF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPITBF`"]
pub struct SPITBF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPITBF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SPITBE`"]
pub type SPITBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPITBE`"]
pub struct SPITBE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPITBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SPIRBE`"]
pub type SPIRBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIRBE`"]
pub struct SPIRBE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIRBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SPIROV`"]
pub type SPIROV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIROV`"]
pub struct SPIROV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIROV_W<'a> {
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
#[doc = "Reader of field `SRMT`"]
pub type SRMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRMT`"]
pub struct SRMT_W<'a> {
    w: &'a mut W,
}
impl<'a> SRMT_W<'a> {
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
#[doc = "Reader of field `SPITUR`"]
pub type SPITUR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPITUR`"]
pub struct SPITUR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPITUR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPIBUSY`"]
pub type SPIBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIBUSY`"]
pub struct SPIBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIBUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `FRMERR`"]
pub type FRMERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRMERR`"]
pub struct FRMERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TXBUFELM`"]
pub type TXBUFELM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXBUFELM`"]
pub struct TXBUFELM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBUFELM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RXBUFELM`"]
pub type RXBUFELM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXBUFELM`"]
pub struct RXBUFELM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBUFELM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spirbf(&self) -> SPIRBF_R {
        SPIRBF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spitbf(&self) -> SPITBF_R {
        SPITBF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spitbe(&self) -> SPITBE_R {
        SPITBE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spirbe(&self) -> SPIRBE_R {
        SPIRBE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spirov(&self) -> SPIROV_R {
        SPIROV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn srmt(&self) -> SRMT_R {
        SRMT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spitur(&self) -> SPITUR_R {
        SPITUR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn spibusy(&self) -> SPIBUSY_R {
        SPIBUSY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn frmerr(&self) -> FRMERR_R {
        FRMERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn txbufelm(&self) -> TXBUFELM_R {
        TXBUFELM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn rxbufelm(&self) -> RXBUFELM_R {
        RXBUFELM_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spirbf(&mut self) -> SPIRBF_W {
        SPIRBF_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spitbf(&mut self) -> SPITBF_W {
        SPITBF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spitbe(&mut self) -> SPITBE_W {
        SPITBE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spirbe(&mut self) -> SPIRBE_W {
        SPIRBE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spirov(&mut self) -> SPIROV_W {
        SPIROV_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn srmt(&mut self) -> SRMT_W {
        SRMT_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spitur(&mut self) -> SPITUR_W {
        SPITUR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn spibusy(&mut self) -> SPIBUSY_W {
        SPIBUSY_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn frmerr(&mut self) -> FRMERR_W {
        FRMERR_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn txbufelm(&mut self) -> TXBUFELM_W {
        TXBUFELM_W { w: self }
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn rxbufelm(&mut self) -> RXBUFELM_W {
        RXBUFELM_W { w: self }
    }
}
