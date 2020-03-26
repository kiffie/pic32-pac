#[doc = "Reader of register CONINV"]
pub type R = crate::R<u32, super::CONINV>;
#[doc = "Writer for register CONINV"]
pub type W = crate::W<u32, super::CONINV>;
#[doc = "Register CONINV `reset()`'s with value 0"]
impl crate::ResetValue for super::CONINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRXISEL`"]
pub type SRXISEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRXISEL`"]
pub struct SRXISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRXISEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `STXISEL`"]
pub type STXISEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STXISEL`"]
pub struct STXISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STXISEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `DISSDI`"]
pub type DISSDI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISSDI`"]
pub struct DISSDI_W<'a> {
    w: &'a mut W,
}
impl<'a> DISSDI_W<'a> {
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
#[doc = "Reader of field `MSTEN`"]
pub type MSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSTEN`"]
pub struct MSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTEN_W<'a> {
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
#[doc = "Reader of field `CKP`"]
pub type CKP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKP`"]
pub struct CKP_W<'a> {
    w: &'a mut W,
}
impl<'a> CKP_W<'a> {
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
#[doc = "Reader of field `SSEN`"]
pub type SSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSEN`"]
pub struct SSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEN_W<'a> {
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
#[doc = "Reader of field `CKE`"]
pub type CKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKE`"]
pub struct CKE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKE_W<'a> {
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
#[doc = "Reader of field `SMP`"]
pub type SMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMP`"]
pub struct SMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP_W<'a> {
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
#[doc = "Reader of field `MODE16`"]
pub type MODE16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODE16`"]
pub struct MODE16_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE16_W<'a> {
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
#[doc = "Reader of field `MODE32`"]
pub type MODE32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODE32`"]
pub struct MODE32_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE32_W<'a> {
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
#[doc = "Reader of field `DISSDO`"]
pub type DISSDO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISSDO`"]
pub struct DISSDO_W<'a> {
    w: &'a mut W,
}
impl<'a> DISSDO_W<'a> {
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
#[doc = "Reader of field `SIDL`"]
pub type SIDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIDL`"]
pub struct SIDL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIDL_W<'a> {
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
#[doc = "Reader of field `FRZ`"]
pub type FRZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRZ`"]
pub struct FRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_W<'a> {
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
#[doc = "Reader of field `ON`"]
pub type ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ON`"]
pub struct ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ON_W<'a> {
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
#[doc = "Reader of field `ENHBUF`"]
pub type ENHBUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENHBUF`"]
pub struct ENHBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> ENHBUF_W<'a> {
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
#[doc = "Reader of field `SPIFE`"]
pub type SPIFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIFE`"]
pub struct SPIFE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIFE_W<'a> {
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
#[doc = "Reader of field `MCLKSEL`"]
pub type MCLKSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCLKSEL`"]
pub struct MCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `FRMCNT`"]
pub type FRMCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRMCNT`"]
pub struct FRMCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `FRMSYPW`"]
pub type FRMSYPW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRMSYPW`"]
pub struct FRMSYPW_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMSYPW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `MSSEN`"]
pub type MSSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSSEN`"]
pub struct MSSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `FRMPOL`"]
pub type FRMPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRMPOL`"]
pub struct FRMPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMPOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `FRMSYNC`"]
pub type FRMSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRMSYNC`"]
pub struct FRMSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMSYNC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `FRMEN`"]
pub type FRMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRMEN`"]
pub struct FRMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn srxisel(&self) -> SRXISEL_R {
        SRXISEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn stxisel(&self) -> STXISEL_R {
        STXISEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dissdi(&self) -> DISSDI_R {
        DISSDI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn msten(&self) -> MSTEN_R {
        MSTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ckp(&self) -> CKP_R {
        CKP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ssen(&self) -> SSEN_R {
        SSEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cke(&self) -> CKE_R {
        CKE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mode16(&self) -> MODE16_R {
        MODE16_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn mode32(&self) -> MODE32_R {
        MODE32_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dissdo(&self) -> DISSDO_R {
        DISSDO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&self) -> SIDL_R {
        SIDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn enhbuf(&self) -> ENHBUF_R {
        ENHBUF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spife(&self) -> SPIFE_R {
        SPIFE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mclksel(&self) -> MCLKSEL_R {
        MCLKSEL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn frmcnt(&self) -> FRMCNT_R {
        FRMCNT_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn frmsypw(&self) -> FRMSYPW_R {
        FRMSYPW_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn mssen(&self) -> MSSEN_R {
        MSSEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn frmpol(&self) -> FRMPOL_R {
        FRMPOL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn frmsync(&self) -> FRMSYNC_R {
        FRMSYNC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn frmen(&self) -> FRMEN_R {
        FRMEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn srxisel(&mut self) -> SRXISEL_W {
        SRXISEL_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn stxisel(&mut self) -> STXISEL_W {
        STXISEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dissdi(&mut self) -> DISSDI_W {
        DISSDI_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn msten(&mut self) -> MSTEN_W {
        MSTEN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ckp(&mut self) -> CKP_W {
        CKP_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ssen(&mut self) -> SSEN_W {
        SSEN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cke(&mut self) -> CKE_W {
        CKE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn smp(&mut self) -> SMP_W {
        SMP_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mode16(&mut self) -> MODE16_W {
        MODE16_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn mode32(&mut self) -> MODE32_W {
        MODE32_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dissdo(&mut self) -> DISSDO_W {
        DISSDO_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&mut self) -> SIDL_W {
        SIDL_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&mut self) -> FRZ_W {
        FRZ_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn enhbuf(&mut self) -> ENHBUF_W {
        ENHBUF_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spife(&mut self) -> SPIFE_W {
        SPIFE_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mclksel(&mut self) -> MCLKSEL_W {
        MCLKSEL_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn frmcnt(&mut self) -> FRMCNT_W {
        FRMCNT_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn frmsypw(&mut self) -> FRMSYPW_W {
        FRMSYPW_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn mssen(&mut self) -> MSSEN_W {
        MSSEN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn frmpol(&mut self) -> FRMPOL_W {
        FRMPOL_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn frmsync(&mut self) -> FRMSYNC_W {
        FRMSYNC_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn frmen(&mut self) -> FRMEN_W {
        FRMEN_W { w: self }
    }
}
