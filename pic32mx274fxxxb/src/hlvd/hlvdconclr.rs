#[doc = "Reader of register HLVDCONCLR"]
pub type R = crate::R<u32, super::HLVDCONCLR>;
#[doc = "Writer for register HLVDCONCLR"]
pub type W = crate::W<u32, super::HLVDCONCLR>;
#[doc = "Register HLVDCONCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::HLVDCONCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HLVDL`"]
pub type HLVDL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HLVDL`"]
pub struct HLVDL_W<'a> {
    w: &'a mut W,
}
impl<'a> HLVDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `HLEVT`"]
pub type HLEVT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HLEVT`"]
pub struct HLEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> HLEVT_W<'a> {
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
#[doc = "Reader of field `BGVST`"]
pub type BGVST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BGVST`"]
pub struct BGVST_W<'a> {
    w: &'a mut W,
}
impl<'a> BGVST_W<'a> {
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
#[doc = "Reader of field `VDIR`"]
pub type VDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDIR`"]
pub struct VDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> VDIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
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
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn hlvdl(&self) -> HLVDL_R {
        HLVDL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn hlevt(&self) -> HLEVT_R {
        HLEVT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bgvst(&self) -> BGVST_R {
        BGVST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn vdir(&self) -> VDIR_R {
        VDIR_R::new(((self.bits >> 11) & 0x01) != 0)
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
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn hlvdl(&mut self) -> HLVDL_W {
        HLVDL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn hlevt(&mut self) -> HLEVT_W {
        HLEVT_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bgvst(&mut self) -> BGVST_W {
        BGVST_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn vdir(&mut self) -> VDIR_W {
        VDIR_W { w: self }
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
}
