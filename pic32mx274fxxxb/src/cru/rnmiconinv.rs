#[doc = "Reader of register RNMICONINV"]
pub type R = crate::R<u32, super::RNMICONINV>;
#[doc = "Writer for register RNMICONINV"]
pub type W = crate::W<u32, super::RNMICONINV>;
#[doc = "Register RNMICONINV `reset()`'s with value 0"]
impl crate::ResetValue for super::RNMICONINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NMICNT`"]
pub type NMICNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NMICNT`"]
pub struct NMICNT_W<'a> {
    w: &'a mut W,
}
impl<'a> NMICNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `WDTS`"]
pub type WDTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTS`"]
pub struct WDTS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTS_W<'a> {
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
#[doc = "Reader of field `CF`"]
pub type CF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CF`"]
pub struct CF_W<'a> {
    w: &'a mut W,
}
impl<'a> CF_W<'a> {
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
#[doc = "Reader of field `HLVD`"]
pub type HLVD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HLVD`"]
pub struct HLVD_W<'a> {
    w: &'a mut W,
}
impl<'a> HLVD_W<'a> {
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
#[doc = "Reader of field `GNMI`"]
pub type GNMI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GNMI`"]
pub struct GNMI_W<'a> {
    w: &'a mut W,
}
impl<'a> GNMI_W<'a> {
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
#[doc = "Reader of field `SWNMI`"]
pub type SWNMI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWNMI`"]
pub struct SWNMI_W<'a> {
    w: &'a mut W,
}
impl<'a> SWNMI_W<'a> {
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
#[doc = "Reader of field `WDTO`"]
pub type WDTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTO`"]
pub struct WDTO_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTO_W<'a> {
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
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn nmicnt(&self) -> NMICNT_R {
        NMICNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wdts(&self) -> WDTS_R {
        WDTS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn hlvd(&self) -> HLVD_R {
        HLVD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gnmi(&self) -> GNMI_R {
        GNMI_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn swnmi(&self) -> SWNMI_R {
        SWNMI_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wdto(&self) -> WDTO_R {
        WDTO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn nmicnt(&mut self) -> NMICNT_W {
        NMICNT_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wdts(&mut self) -> WDTS_W {
        WDTS_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cf(&mut self) -> CF_W {
        CF_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn hlvd(&mut self) -> HLVD_W {
        HLVD_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gnmi(&mut self) -> GNMI_W {
        GNMI_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn swnmi(&mut self) -> SWNMI_W {
        SWNMI_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wdto(&mut self) -> WDTO_W {
        WDTO_W { w: self }
    }
}
