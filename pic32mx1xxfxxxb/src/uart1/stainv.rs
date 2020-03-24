#[doc = "Reader of register STAINV"]
pub type R = crate::R<u32, super::STAINV>;
#[doc = "Writer for register STAINV"]
pub type W = crate::W<u32, super::STAINV>;
#[doc = "Register STAINV `reset()`'s with value 0"]
impl crate::ResetValue for super::STAINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `URXDA`"]
pub type URXDA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URXDA`"]
pub struct URXDA_W<'a> {
    w: &'a mut W,
}
impl<'a> URXDA_W<'a> {
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
#[doc = "Reader of field `OERR`"]
pub type OERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OERR`"]
pub struct OERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OERR_W<'a> {
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
#[doc = "Reader of field `FERR`"]
pub type FERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FERR`"]
pub struct FERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PERR`"]
pub type PERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PERR`"]
pub struct PERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PERR_W<'a> {
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
#[doc = "Reader of field `RIDLE`"]
pub type RIDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RIDLE`"]
pub struct RIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RIDLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADDEN`"]
pub type ADDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDEN`"]
pub struct ADDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDEN_W<'a> {
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
#[doc = "Reader of field `URXISEL`"]
pub type URXISEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `URXISEL`"]
pub struct URXISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> URXISEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `TRMT`"]
pub type TRMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRMT`"]
pub struct TRMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRMT_W<'a> {
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
#[doc = "Reader of field `UTXBF`"]
pub type UTXBF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UTXBF`"]
pub struct UTXBF_W<'a> {
    w: &'a mut W,
}
impl<'a> UTXBF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `UTXEN`"]
pub type UTXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UTXEN`"]
pub struct UTXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UTXEN_W<'a> {
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
#[doc = "Reader of field `UTXBRK`"]
pub type UTXBRK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UTXBRK`"]
pub struct UTXBRK_W<'a> {
    w: &'a mut W,
}
impl<'a> UTXBRK_W<'a> {
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
#[doc = "Reader of field `URXEN`"]
pub type URXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URXEN`"]
pub struct URXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> URXEN_W<'a> {
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
#[doc = "Reader of field `UTXINV`"]
pub type UTXINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UTXINV`"]
pub struct UTXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> UTXINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `UTXISEL`"]
pub type UTXISEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UTXISEL`"]
pub struct UTXISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UTXISEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADM_EN`"]
pub type ADM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADM_EN`"]
pub struct ADM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn urxda(&self) -> URXDA_R {
        URXDA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn oerr(&self) -> OERR_R {
        OERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ridle(&self) -> RIDLE_R {
        RIDLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn adden(&self) -> ADDEN_R {
        ADDEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn urxisel(&self) -> URXISEL_R {
        URXISEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trmt(&self) -> TRMT_R {
        TRMT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn utxbf(&self) -> UTXBF_R {
        UTXBF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn utxen(&self) -> UTXEN_R {
        UTXEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn utxbrk(&self) -> UTXBRK_R {
        UTXBRK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn urxen(&self) -> URXEN_R {
        URXEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn utxinv(&self) -> UTXINV_R {
        UTXINV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn utxisel(&self) -> UTXISEL_R {
        UTXISEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adm_en(&self) -> ADM_EN_R {
        ADM_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn urxda(&mut self) -> URXDA_W {
        URXDA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn oerr(&mut self) -> OERR_W {
        OERR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FERR_W {
        FERR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn perr(&mut self) -> PERR_W {
        PERR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ridle(&mut self) -> RIDLE_W {
        RIDLE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn adden(&mut self) -> ADDEN_W {
        ADDEN_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn urxisel(&mut self) -> URXISEL_W {
        URXISEL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trmt(&mut self) -> TRMT_W {
        TRMT_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn utxbf(&mut self) -> UTXBF_W {
        UTXBF_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn utxen(&mut self) -> UTXEN_W {
        UTXEN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn utxbrk(&mut self) -> UTXBRK_W {
        UTXBRK_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn urxen(&mut self) -> URXEN_W {
        URXEN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn utxinv(&mut self) -> UTXINV_W {
        UTXINV_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn utxisel(&mut self) -> UTXISEL_W {
        UTXISEL_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adm_en(&mut self) -> ADM_EN_W {
        ADM_EN_W { w: self }
    }
}
