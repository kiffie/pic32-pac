#[doc = "Reader of register AD1CON3SET"]
pub type R = crate::R<u32, super::AD1CON3SET>;
#[doc = "Writer for register AD1CON3SET"]
pub type W = crate::W<u32, super::AD1CON3SET>;
#[doc = "Register AD1CON3SET `reset()`'s with value 0"]
impl crate::ResetValue for super::AD1CON3SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADCS`"]
pub type ADCS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCS`"]
pub struct ADCS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `SAMC`"]
pub type SAMC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAMC`"]
pub struct SAMC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADRC`"]
pub type ADRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADRC`"]
pub struct ADRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRC_W<'a> {
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
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn adcs(&self) -> ADCS_R {
        ADCS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn samc(&self) -> SAMC_R {
        SAMC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adrc(&self) -> ADRC_R {
        ADRC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn adcs(&mut self) -> ADCS_W {
        ADCS_W { w: self }
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn samc(&mut self) -> SAMC_W {
        SAMC_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adrc(&mut self) -> ADRC_W {
        ADRC_W { w: self }
    }
}
