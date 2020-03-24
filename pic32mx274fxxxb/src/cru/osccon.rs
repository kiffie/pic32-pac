#[doc = "Reader of register OSCCON"]
pub type R = crate::R<u32, super::OSCCON>;
#[doc = "Writer for register OSCCON"]
pub type W = crate::W<u32, super::OSCCON>;
#[doc = "Register OSCCON `reset()`'s with value 0"]
impl crate::ResetValue for super::OSCCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OSWEN`"]
pub type OSWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSWEN`"]
pub struct OSWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSWEN_W<'a> {
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
#[doc = "Reader of field `SOSCEN`"]
pub type SOSCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOSCEN`"]
pub struct SOSCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCEN_W<'a> {
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
#[doc = "Reader of field `UFRCEN`"]
pub type UFRCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UFRCEN`"]
pub struct UFRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UFRCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SLPEN`"]
pub type SLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLPEN`"]
pub struct SLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLPEN_W<'a> {
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
#[doc = "Reader of field `CLKLOCK`"]
pub type CLKLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKLOCK`"]
pub struct CLKLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKLOCK_W<'a> {
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
#[doc = "Reader of field `NOSC`"]
pub type NOSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NOSC`"]
pub struct NOSC_W<'a> {
    w: &'a mut W,
}
impl<'a> NOSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `COSC`"]
pub type COSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COSC`"]
pub struct COSC_W<'a> {
    w: &'a mut W,
}
impl<'a> COSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `SLP2SPD`"]
pub type SLP2SPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLP2SPD`"]
pub struct SLP2SPD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP2SPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `DRMEN`"]
pub type DRMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRMEN`"]
pub struct DRMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DRMEN_W<'a> {
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
#[doc = "Reader of field `FRCDIV`"]
pub type FRCDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRCDIV`"]
pub struct FRCDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn oswen(&self) -> OSWEN_R {
        OSWEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn soscen(&self) -> SOSCEN_R {
        SOSCEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ufrcen(&self) -> UFRCEN_R {
        UFRCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slpen(&self) -> SLPEN_R {
        SLPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clklock(&self) -> CLKLOCK_R {
        CLKLOCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn nosc(&self) -> NOSC_R {
        NOSC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn cosc(&self) -> COSC_R {
        COSC_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slp2spd(&self) -> SLP2SPD_R {
        SLP2SPD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn drmen(&self) -> DRMEN_R {
        DRMEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn frcdiv(&self) -> FRCDIV_R {
        FRCDIV_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn oswen(&mut self) -> OSWEN_W {
        OSWEN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn soscen(&mut self) -> SOSCEN_W {
        SOSCEN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ufrcen(&mut self) -> UFRCEN_W {
        UFRCEN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cf(&mut self) -> CF_W {
        CF_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slpen(&mut self) -> SLPEN_W {
        SLPEN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clklock(&mut self) -> CLKLOCK_W {
        CLKLOCK_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn nosc(&mut self) -> NOSC_W {
        NOSC_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn cosc(&mut self) -> COSC_W {
        COSC_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slp2spd(&mut self) -> SLP2SPD_W {
        SLP2SPD_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn drmen(&mut self) -> DRMEN_W {
        DRMEN_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn frcdiv(&mut self) -> FRCDIV_W {
        FRCDIV_W { w: self }
    }
}
