#[doc = "Reader of register PMCONINV"]
pub type R = crate::R<u32, super::PMCONINV>;
#[doc = "Writer for register PMCONINV"]
pub type W = crate::W<u32, super::PMCONINV>;
#[doc = "Register PMCONINV `reset()`'s with value 0"]
impl crate::ResetValue for super::PMCONINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDSP`"]
pub type RDSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDSP`"]
pub struct RDSP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDSP_W<'a> {
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
#[doc = "Reader of field `WRSP`"]
pub type WRSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRSP`"]
pub struct WRSP_W<'a> {
    w: &'a mut W,
}
impl<'a> WRSP_W<'a> {
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
#[doc = "Reader of field `CS1P`"]
pub type CS1P_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS1P`"]
pub struct CS1P_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1P_W<'a> {
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
#[doc = "Reader of field `ALP`"]
pub type ALP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALP`"]
pub struct ALP_W<'a> {
    w: &'a mut W,
}
impl<'a> ALP_W<'a> {
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
#[doc = "Reader of field `CSF`"]
pub type CSF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSF`"]
pub struct CSF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `PTRDEN`"]
pub type PTRDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTRDEN`"]
pub struct PTRDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTRDEN_W<'a> {
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
#[doc = "Reader of field `PTWREN`"]
pub type PTWREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTWREN`"]
pub struct PTWREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTWREN_W<'a> {
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
#[doc = "Reader of field `PMPTTL`"]
pub type PMPTTL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMPTTL`"]
pub struct PMPTTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PMPTTL_W<'a> {
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
#[doc = "Reader of field `ADRMUX`"]
pub type ADRMUX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADRMUX`"]
pub struct ADRMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRMUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
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
#[doc = "Reader of field `DUALBUF`"]
pub type DUALBUF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUALBUF`"]
pub struct DUALBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DUALBUF_W<'a> {
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
#[doc = "Reader of field `RDSTART`"]
pub type RDSTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDSTART`"]
pub struct RDSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RDSTART_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rdsp(&self) -> RDSP_R {
        RDSP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wrsp(&self) -> WRSP_R {
        WRSP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cs1p(&self) -> CS1P_R {
        CS1P_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn alp(&self) -> ALP_R {
        ALP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn csf(&self) -> CSF_R {
        CSF_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ptrden(&self) -> PTRDEN_R {
        PTRDEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ptwren(&self) -> PTWREN_R {
        PTWREN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pmpttl(&self) -> PMPTTL_R {
        PMPTTL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn adrmux(&self) -> ADRMUX_R {
        ADRMUX_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&self) -> SIDL_R {
        SIDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dualbuf(&self) -> DUALBUF_R {
        DUALBUF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rdstart(&self) -> RDSTART_R {
        RDSTART_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rdsp(&mut self) -> RDSP_W {
        RDSP_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wrsp(&mut self) -> WRSP_W {
        WRSP_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cs1p(&mut self) -> CS1P_W {
        CS1P_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn alp(&mut self) -> ALP_W {
        ALP_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn csf(&mut self) -> CSF_W {
        CSF_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ptrden(&mut self) -> PTRDEN_W {
        PTRDEN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ptwren(&mut self) -> PTWREN_W {
        PTWREN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pmpttl(&mut self) -> PMPTTL_W {
        PMPTTL_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn adrmux(&mut self) -> ADRMUX_W {
        ADRMUX_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&mut self) -> SIDL_W {
        SIDL_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dualbuf(&mut self) -> DUALBUF_W {
        DUALBUF_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rdstart(&mut self) -> RDSTART_W {
        RDSTART_W { w: self }
    }
}
