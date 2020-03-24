#[doc = "Reader of register AD1CON2"]
pub type R = crate::R<u32, super::AD1CON2>;
#[doc = "Writer for register AD1CON2"]
pub type W = crate::W<u32, super::AD1CON2>;
#[doc = "Register AD1CON2 `reset()`'s with value 0"]
impl crate::ResetValue for super::AD1CON2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALTS`"]
pub type ALTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALTS`"]
pub struct ALTS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTS_W<'a> {
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
#[doc = "Reader of field `BUFM`"]
pub type BUFM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFM`"]
pub struct BUFM_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFM_W<'a> {
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
#[doc = "Reader of field `SMPI`"]
pub type SMPI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMPI`"]
pub struct SMPI_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `BUFS`"]
pub type BUFS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFS`"]
pub struct BUFS_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFS_W<'a> {
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
#[doc = "Reader of field `CSCNA`"]
pub type CSCNA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSCNA`"]
pub struct CSCNA_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCNA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `OFFCAL`"]
pub type OFFCAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OFFCAL`"]
pub struct OFFCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFCAL_W<'a> {
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
#[doc = "Reader of field `VCFG`"]
pub type VCFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VCFG`"]
pub struct VCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> VCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn alts(&self) -> ALTS_R {
        ALTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bufm(&self) -> BUFM_R {
        BUFM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn smpi(&self) -> SMPI_R {
        SMPI_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bufs(&self) -> BUFS_R {
        BUFS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cscna(&self) -> CSCNA_R {
        CSCNA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn offcal(&self) -> OFFCAL_R {
        OFFCAL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn vcfg(&self) -> VCFG_R {
        VCFG_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn alts(&mut self) -> ALTS_W {
        ALTS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bufm(&mut self) -> BUFM_W {
        BUFM_W { w: self }
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn smpi(&mut self) -> SMPI_W {
        SMPI_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bufs(&mut self) -> BUFS_W {
        BUFS_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cscna(&mut self) -> CSCNA_W {
        CSCNA_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn offcal(&mut self) -> OFFCAL_W {
        OFFCAL_W { w: self }
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn vcfg(&mut self) -> VCFG_W {
        VCFG_W { w: self }
    }
}
