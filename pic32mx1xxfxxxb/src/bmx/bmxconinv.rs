#[doc = "Reader of register BMXCONINV"]
pub type R = crate::R<u32, super::BMXCONINV>;
#[doc = "Writer for register BMXCONINV"]
pub type W = crate::W<u32, super::BMXCONINV>;
#[doc = "Register BMXCONINV `reset()`'s with value 0"]
impl crate::ResetValue for super::BMXCONINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BMXARB`"]
pub type BMXARB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BMXARB`"]
pub struct BMXARB_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXARB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `BMXWSDRM`"]
pub type BMXWSDRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BMXWSDRM`"]
pub struct BMXWSDRM_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXWSDRM_W<'a> {
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
#[doc = "Reader of field `BMXERRIS`"]
pub type BMXERRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BMXERRIS`"]
pub struct BMXERRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXERRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `BMXERRDS`"]
pub type BMXERRDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BMXERRDS`"]
pub struct BMXERRDS_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXERRDS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `BMXERRDMA`"]
pub type BMXERRDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BMXERRDMA`"]
pub struct BMXERRDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXERRDMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `BMXERRICD`"]
pub type BMXERRICD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BMXERRICD`"]
pub struct BMXERRICD_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXERRICD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `BMXERRIXI`"]
pub type BMXERRIXI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BMXERRIXI`"]
pub struct BMXERRIXI_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXERRIXI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `BMXCHEDMA`"]
pub type BMXCHEDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BMXCHEDMA`"]
pub struct BMXCHEDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXCHEDMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn bmxarb(&self) -> BMXARB_R {
        BMXARB_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bmxwsdrm(&self) -> BMXWSDRM_R {
        BMXWSDRM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn bmxerris(&self) -> BMXERRIS_R {
        BMXERRIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn bmxerrds(&self) -> BMXERRDS_R {
        BMXERRDS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn bmxerrdma(&self) -> BMXERRDMA_R {
        BMXERRDMA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn bmxerricd(&self) -> BMXERRICD_R {
        BMXERRICD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn bmxerrixi(&self) -> BMXERRIXI_R {
        BMXERRIXI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bmxchedma(&self) -> BMXCHEDMA_R {
        BMXCHEDMA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn bmxarb(&mut self) -> BMXARB_W {
        BMXARB_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bmxwsdrm(&mut self) -> BMXWSDRM_W {
        BMXWSDRM_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn bmxerris(&mut self) -> BMXERRIS_W {
        BMXERRIS_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn bmxerrds(&mut self) -> BMXERRDS_W {
        BMXERRDS_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn bmxerrdma(&mut self) -> BMXERRDMA_W {
        BMXERRDMA_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn bmxerricd(&mut self) -> BMXERRICD_W {
        BMXERRICD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn bmxerrixi(&mut self) -> BMXERRIXI_W {
        BMXERRIXI_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bmxchedma(&mut self) -> BMXCHEDMA_W {
        BMXCHEDMA_W { w: self }
    }
}
