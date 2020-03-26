#[doc = "Reader of register RTCALRM"]
pub type R = crate::R<u32, super::RTCALRM>;
#[doc = "Writer for register RTCALRM"]
pub type W = crate::W<u32, super::RTCALRM>;
#[doc = "Register RTCALRM `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCALRM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ARPT`"]
pub type ARPT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ARPT`"]
pub struct ARPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ARPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `AMASK`"]
pub type AMASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMASK`"]
pub struct AMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> AMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ALRMSYNC`"]
pub type ALRMSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALRMSYNC`"]
pub struct ALRMSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRMSYNC_W<'a> {
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
#[doc = "Reader of field `PIV`"]
pub type PIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIV`"]
pub struct PIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PIV_W<'a> {
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
#[doc = "Reader of field `CHIME`"]
pub type CHIME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHIME`"]
pub struct CHIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIME_W<'a> {
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
#[doc = "Reader of field `ALRMEN`"]
pub type ALRMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALRMEN`"]
pub struct ALRMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRMEN_W<'a> {
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
    pub fn arpt(&self) -> ARPT_R {
        ARPT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn amask(&self) -> AMASK_R {
        AMASK_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn alrmsync(&self) -> ALRMSYNC_R {
        ALRMSYNC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn piv(&self) -> PIV_R {
        PIV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn chime(&self) -> CHIME_R {
        CHIME_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn alrmen(&self) -> ALRMEN_R {
        ALRMEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn arpt(&mut self) -> ARPT_W {
        ARPT_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn amask(&mut self) -> AMASK_W {
        AMASK_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn alrmsync(&mut self) -> ALRMSYNC_W {
        ALRMSYNC_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn piv(&mut self) -> PIV_W {
        PIV_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn chime(&mut self) -> CHIME_W {
        CHIME_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn alrmen(&mut self) -> ALRMEN_W {
        ALRMEN_W { w: self }
    }
}
