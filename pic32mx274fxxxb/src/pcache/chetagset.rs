#[doc = "Reader of register CHETAGSET"]
pub type R = crate::R<u32, super::CHETAGSET>;
#[doc = "Writer for register CHETAGSET"]
pub type W = crate::W<u32, super::CHETAGSET>;
#[doc = "Register CHETAGSET `reset()`'s with value 0"]
impl crate::ResetValue for super::CHETAGSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LTYPE`"]
pub type LTYPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LTYPE`"]
pub struct LTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> LTYPE_W<'a> {
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
#[doc = "Reader of field `LLOCK`"]
pub type LLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LLOCK`"]
pub struct LLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LLOCK_W<'a> {
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
#[doc = "Reader of field `LVALID`"]
pub type LVALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVALID`"]
pub struct LVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> LVALID_W<'a> {
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
#[doc = "Reader of field `LTAG`"]
pub type LTAG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LTAG`"]
pub struct LTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> LTAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 4)) | (((value as u32) & 0x000f_ffff) << 4);
        self.w
    }
}
#[doc = "Reader of field `LTAGBOOT`"]
pub type LTAGBOOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LTAGBOOT`"]
pub struct LTAGBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> LTAGBOOT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ltype(&self) -> LTYPE_R {
        LTYPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn llock(&self) -> LLOCK_R {
        LLOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lvalid(&self) -> LVALID_R {
        LVALID_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:23"]
    #[inline(always)]
    pub fn ltag(&self) -> LTAG_R {
        LTAG_R::new(((self.bits >> 4) & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ltagboot(&self) -> LTAGBOOT_R {
        LTAGBOOT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ltype(&mut self) -> LTYPE_W {
        LTYPE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn llock(&mut self) -> LLOCK_W {
        LLOCK_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lvalid(&mut self) -> LVALID_W {
        LVALID_W { w: self }
    }
    #[doc = "Bits 4:23"]
    #[inline(always)]
    pub fn ltag(&mut self) -> LTAG_W {
        LTAG_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ltagboot(&mut self) -> LTAGBOOT_W {
        LTAGBOOT_W { w: self }
    }
}
