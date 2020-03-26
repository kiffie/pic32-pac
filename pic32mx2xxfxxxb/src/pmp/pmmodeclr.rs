#[doc = "Reader of register PMMODECLR"]
pub type R = crate::R<u32, super::PMMODECLR>;
#[doc = "Writer for register PMMODECLR"]
pub type W = crate::W<u32, super::PMMODECLR>;
#[doc = "Register PMMODECLR `reset()`'s with value 0"]
impl crate::ResetValue for super::PMMODECLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WAITE`"]
pub type WAITE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAITE`"]
pub struct WAITE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `WAITM`"]
pub type WAITM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAITM`"]
pub struct WAITM_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `WAITB`"]
pub type WAITB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAITB`"]
pub struct WAITB_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `INCM`"]
pub type INCM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INCM`"]
pub struct INCM_W<'a> {
    w: &'a mut W,
}
impl<'a> INCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `IRQM`"]
pub type IRQM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRQM`"]
pub struct IRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSY`"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn waite(&self) -> WAITE_R {
        WAITE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn waitm(&self) -> WAITM_R {
        WAITM_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn waitb(&self) -> WAITB_R {
        WAITB_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn incm(&self) -> INCM_R {
        INCM_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn irqm(&self) -> IRQM_R {
        IRQM_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn waite(&mut self) -> WAITE_W {
        WAITE_W { w: self }
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn waitm(&mut self) -> WAITM_W {
        WAITM_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn waitb(&mut self) -> WAITB_W {
        WAITB_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn incm(&mut self) -> INCM_W {
        INCM_W { w: self }
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn irqm(&mut self) -> IRQM_W {
        IRQM_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
}
