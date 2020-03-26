#[doc = "Reader of register OSCCONCLR"]
pub type R = crate::R<u32, super::OSCCONCLR>;
#[doc = "Writer for register OSCCONCLR"]
pub type W = crate::W<u32, super::OSCCONCLR>;
#[doc = "Register OSCCONCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::OSCCONCLR {
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
#[doc = "Reader of field `SLOCK`"]
pub type SLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOCK`"]
pub struct SLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOCK_W<'a> {
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
#[doc = "Reader of field `ULOCK`"]
pub type ULOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ULOCK`"]
pub struct ULOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ULOCK_W<'a> {
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
#[doc = "Reader of field `PLLMULT`"]
pub type PLLMULT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLMULT`"]
pub struct PLLMULT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLMULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `PBDIV`"]
pub type PBDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBDIV`"]
pub struct PBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PBDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Reader of field `PBDIVRDY`"]
pub type PBDIVRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBDIVRDY`"]
pub struct PBDIVRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> PBDIVRDY_W<'a> {
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
#[doc = "Reader of field `SOSCRDY`"]
pub type SOSCRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOSCRDY`"]
pub struct SOSCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
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
#[doc = "Reader of field `PLLODIV`"]
pub type PLLODIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLODIV`"]
pub struct PLLODIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLODIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
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
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slock(&self) -> SLOCK_R {
        SLOCK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ulock(&self) -> ULOCK_R {
        ULOCK_R::new(((self.bits >> 6) & 0x01) != 0)
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
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn pllmult(&self) -> PLLMULT_R {
        PLLMULT_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn pbdiv(&self) -> PBDIV_R {
        PBDIV_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pbdivrdy(&self) -> PBDIVRDY_R {
        PBDIVRDY_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn soscrdy(&self) -> SOSCRDY_R {
        SOSCRDY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn frcdiv(&self) -> FRCDIV_R {
        FRCDIV_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn pllodiv(&self) -> PLLODIV_R {
        PLLODIV_R::new(((self.bits >> 27) & 0x07) as u8)
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
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slock(&mut self) -> SLOCK_W {
        SLOCK_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ulock(&mut self) -> ULOCK_W {
        ULOCK_W { w: self }
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
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn pllmult(&mut self) -> PLLMULT_W {
        PLLMULT_W { w: self }
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn pbdiv(&mut self) -> PBDIV_W {
        PBDIV_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pbdivrdy(&mut self) -> PBDIVRDY_W {
        PBDIVRDY_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn soscrdy(&mut self) -> SOSCRDY_W {
        SOSCRDY_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn frcdiv(&mut self) -> FRCDIV_W {
        FRCDIV_W { w: self }
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn pllodiv(&mut self) -> PLLODIV_W {
        PLLODIV_W { w: self }
    }
}
