#[doc = "Reader of register DCH3ECONINV"]
pub type R = crate::R<u32, super::DCH3ECONINV>;
#[doc = "Writer for register DCH3ECONINV"]
pub type W = crate::W<u32, super::DCH3ECONINV>;
#[doc = "Register DCH3ECONINV `reset()`'s with value 0"]
impl crate::ResetValue for super::DCH3ECONINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AIRQEN`"]
pub type AIRQEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AIRQEN`"]
pub struct AIRQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AIRQEN_W<'a> {
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
#[doc = "Reader of field `SIRQEN`"]
pub type SIRQEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIRQEN`"]
pub struct SIRQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIRQEN_W<'a> {
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
#[doc = "Reader of field `PATEN`"]
pub type PATEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PATEN`"]
pub struct PATEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PATEN_W<'a> {
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
#[doc = "Reader of field `CABORT`"]
pub type CABORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CABORT`"]
pub struct CABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> CABORT_W<'a> {
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
#[doc = "Reader of field `CFORCE`"]
pub type CFORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFORCE`"]
pub struct CFORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFORCE_W<'a> {
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
#[doc = "Reader of field `CHSIRQ`"]
pub type CHSIRQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHSIRQ`"]
pub struct CHSIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSIRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CHAIRQ`"]
pub type CHAIRQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHAIRQ`"]
pub struct CHAIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAIRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn airqen(&self) -> AIRQEN_R {
        AIRQEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sirqen(&self) -> SIRQEN_R {
        SIRQEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn paten(&self) -> PATEN_R {
        PATEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cabort(&self) -> CABORT_R {
        CABORT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cforce(&self) -> CFORCE_R {
        CFORCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn chsirq(&self) -> CHSIRQ_R {
        CHSIRQ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn chairq(&self) -> CHAIRQ_R {
        CHAIRQ_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn airqen(&mut self) -> AIRQEN_W {
        AIRQEN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sirqen(&mut self) -> SIRQEN_W {
        SIRQEN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn paten(&mut self) -> PATEN_W {
        PATEN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cabort(&mut self) -> CABORT_W {
        CABORT_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cforce(&mut self) -> CFORCE_W {
        CFORCE_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn chsirq(&mut self) -> CHSIRQ_W {
        CHSIRQ_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn chairq(&mut self) -> CHAIRQ_W {
        CHAIRQ_W { w: self }
    }
}
