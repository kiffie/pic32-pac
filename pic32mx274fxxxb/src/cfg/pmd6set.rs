#[doc = "Reader of register PMD6SET"]
pub type R = crate::R<u32, super::PMD6SET>;
#[doc = "Writer for register PMD6SET"]
pub type W = crate::W<u32, super::PMD6SET>;
#[doc = "Register PMD6SET `reset()`'s with value 0"]
impl crate::ResetValue for super::PMD6SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCCMD`"]
pub type RTCCMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCCMD`"]
pub struct RTCCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCMD_W<'a> {
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
#[doc = "Reader of field `REFOMD`"]
pub type REFOMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFOMD`"]
pub struct REFOMD_W<'a> {
    w: &'a mut W,
}
impl<'a> REFOMD_W<'a> {
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
#[doc = "Reader of field `PMPMD`"]
pub type PMPMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMPMD`"]
pub struct PMPMD_W<'a> {
    w: &'a mut W,
}
impl<'a> PMPMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtccmd(&self) -> RTCCMD_R {
        RTCCMD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn refomd(&self) -> REFOMD_R {
        REFOMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pmpmd(&self) -> PMPMD_R {
        PMPMD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtccmd(&mut self) -> RTCCMD_W {
        RTCCMD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn refomd(&mut self) -> REFOMD_W {
        REFOMD_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pmpmd(&mut self) -> PMPMD_W {
        PMPMD_W { w: self }
    }
}
