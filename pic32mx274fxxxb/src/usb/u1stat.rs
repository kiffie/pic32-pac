#[doc = "Reader of register U1STAT"]
pub type R = crate::R<u32, super::U1STAT>;
#[doc = "Writer for register U1STAT"]
pub type W = crate::W<u32, super::U1STAT>;
#[doc = "Register U1STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::U1STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PPBI`"]
pub type PPBI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PPBI`"]
pub struct PPBI_W<'a> {
    w: &'a mut W,
}
impl<'a> PPBI_W<'a> {
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
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
#[doc = "Reader of field `ENDPT`"]
pub type ENDPT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ENDPT`"]
pub struct ENDPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ppbi(&self) -> PPBI_R {
        PPBI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn endpt(&self) -> ENDPT_R {
        ENDPT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ppbi(&mut self) -> PPBI_W {
        PPBI_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn endpt(&mut self) -> ENDPT_W {
        ENDPT_W { w: self }
    }
}
