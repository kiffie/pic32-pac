#[doc = "Reader of register PMD1SET"]
pub type R = crate::R<u32, super::PMD1SET>;
#[doc = "Writer for register PMD1SET"]
pub type W = crate::W<u32, super::PMD1SET>;
#[doc = "Register PMD1SET `reset()`'s with value 0"]
impl crate::ResetValue for super::PMD1SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AD1MD`"]
pub type AD1MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AD1MD`"]
pub struct AD1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1MD_W<'a> {
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
#[doc = "Reader of field `CTMUMD`"]
pub type CTMUMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMUMD`"]
pub struct CTMUMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMUMD_W<'a> {
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
#[doc = "Reader of field `CVRMD`"]
pub type CVRMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CVRMD`"]
pub struct CVRMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CVRMD_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ad1md(&self) -> AD1MD_R {
        AD1MD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ctmumd(&self) -> CTMUMD_R {
        CTMUMD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cvrmd(&self) -> CVRMD_R {
        CVRMD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ad1md(&mut self) -> AD1MD_W {
        AD1MD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ctmumd(&mut self) -> CTMUMD_W {
        CTMUMD_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cvrmd(&mut self) -> CVRMD_W {
        CVRMD_W { w: self }
    }
}
