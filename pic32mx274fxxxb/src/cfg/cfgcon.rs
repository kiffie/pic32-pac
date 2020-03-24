#[doc = "Reader of register CFGCON"]
pub type R = crate::R<u32, super::CFGCON>;
#[doc = "Writer for register CFGCON"]
pub type W = crate::W<u32, super::CFGCON>;
#[doc = "Register CFGCON `reset()`'s with value 0x08"]
impl crate::ResetValue for super::CFGCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `TDOEN`"]
pub type TDOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TDOEN`"]
pub struct TDOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDOEN_W<'a> {
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
#[doc = "Reader of field `FAEN`"]
pub type FAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAEN`"]
pub struct FAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAEN_W<'a> {
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
#[doc = "Reader of field `JTAGEN`"]
pub type JTAGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JTAGEN`"]
pub struct JTAGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAGEN_W<'a> {
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
#[doc = "Reader of field `RPFA`"]
pub type RPFA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPFA`"]
pub struct RPFA_W<'a> {
    w: &'a mut W,
}
impl<'a> RPFA_W<'a> {
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
#[doc = "Reader of field `PMDLOCK`"]
pub type PMDLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMDLOCK`"]
pub struct PMDLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PMDLOCK_W<'a> {
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
#[doc = "Reader of field `IOLOCK`"]
pub type IOLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOLOCK`"]
pub struct IOLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> IOLOCK_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tdoen(&self) -> TDOEN_R {
        TDOEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn faen(&self) -> FAEN_R {
        FAEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn jtagen(&self) -> JTAGEN_R {
        JTAGEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rpfa(&self) -> RPFA_R {
        RPFA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pmdlock(&self) -> PMDLOCK_R {
        PMDLOCK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn iolock(&self) -> IOLOCK_R {
        IOLOCK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tdoen(&mut self) -> TDOEN_W {
        TDOEN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn faen(&mut self) -> FAEN_W {
        FAEN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn jtagen(&mut self) -> JTAGEN_W {
        JTAGEN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rpfa(&mut self) -> RPFA_W {
        RPFA_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pmdlock(&mut self) -> PMDLOCK_W {
        PMDLOCK_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn iolock(&mut self) -> IOLOCK_W {
        IOLOCK_W { w: self }
    }
}
