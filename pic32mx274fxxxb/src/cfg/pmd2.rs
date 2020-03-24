#[doc = "Reader of register PMD2"]
pub type R = crate::R<u32, super::PMD2>;
#[doc = "Writer for register PMD2"]
pub type W = crate::W<u32, super::PMD2>;
#[doc = "Register PMD2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PMD2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMP1MD`"]
pub type CMP1MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP1MD`"]
pub struct CMP1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1MD_W<'a> {
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
#[doc = "Reader of field `CMP2MD`"]
pub type CMP2MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP2MD`"]
pub struct CMP2MD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2MD_W<'a> {
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
#[doc = "Reader of field `CMP3MD`"]
pub type CMP3MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP3MD`"]
pub struct CMP3MD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3MD_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmp1md(&self) -> CMP1MD_R {
        CMP1MD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmp2md(&self) -> CMP2MD_R {
        CMP2MD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmp3md(&self) -> CMP3MD_R {
        CMP3MD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmp1md(&mut self) -> CMP1MD_W {
        CMP1MD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmp2md(&mut self) -> CMP2MD_W {
        CMP2MD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmp3md(&mut self) -> CMP3MD_W {
        CMP3MD_W { w: self }
    }
}
