#[doc = "Reader of register U1IRCLR"]
pub type R = crate::R<u32, super::U1IRCLR>;
#[doc = "Writer for register U1IRCLR"]
pub type W = crate::W<u32, super::U1IRCLR>;
#[doc = "Register U1IRCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::U1IRCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `URSTIF_DETACHIF`"]
pub type URSTIF_DETACHIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URSTIF_DETACHIF`"]
pub struct URSTIF_DETACHIF_W<'a> {
    w: &'a mut W,
}
impl<'a> URSTIF_DETACHIF_W<'a> {
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
#[doc = "Reader of field `UERRIF`"]
pub type UERRIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UERRIF`"]
pub struct UERRIF_W<'a> {
    w: &'a mut W,
}
impl<'a> UERRIF_W<'a> {
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
#[doc = "Reader of field `SOFIF`"]
pub type SOFIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFIF`"]
pub struct SOFIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIF_W<'a> {
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
#[doc = "Reader of field `TRNIF`"]
pub type TRNIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRNIF`"]
pub struct TRNIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNIF_W<'a> {
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
#[doc = "Reader of field `IDLEIF`"]
pub type IDLEIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDLEIF`"]
pub struct IDLEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLEIF_W<'a> {
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
#[doc = "Reader of field `RESUMEIF`"]
pub type RESUMEIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUMEIF`"]
pub struct RESUMEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUMEIF_W<'a> {
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
#[doc = "Reader of field `ATTACHIF`"]
pub type ATTACHIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATTACHIF`"]
pub struct ATTACHIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTACHIF_W<'a> {
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
#[doc = "Reader of field `STALLIF`"]
pub type STALLIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALLIF`"]
pub struct STALLIF_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLIF_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn urstif_detachif(&self) -> URSTIF_DETACHIF_R {
        URSTIF_DETACHIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uerrif(&self) -> UERRIF_R {
        UERRIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sofif(&self) -> SOFIF_R {
        SOFIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trnif(&self) -> TRNIF_R {
        TRNIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn idleif(&self) -> IDLEIF_R {
        IDLEIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn resumeif(&self) -> RESUMEIF_R {
        RESUMEIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn attachif(&self) -> ATTACHIF_R {
        ATTACHIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn stallif(&self) -> STALLIF_R {
        STALLIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn urstif_detachif(&mut self) -> URSTIF_DETACHIF_W {
        URSTIF_DETACHIF_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uerrif(&mut self) -> UERRIF_W {
        UERRIF_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sofif(&mut self) -> SOFIF_W {
        SOFIF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trnif(&mut self) -> TRNIF_W {
        TRNIF_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn idleif(&mut self) -> IDLEIF_W {
        IDLEIF_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn resumeif(&mut self) -> RESUMEIF_W {
        RESUMEIF_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn attachif(&mut self) -> ATTACHIF_W {
        ATTACHIF_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn stallif(&mut self) -> STALLIF_W {
        STALLIF_W { w: self }
    }
}
