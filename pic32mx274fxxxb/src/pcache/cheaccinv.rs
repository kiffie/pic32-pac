#[doc = "Reader of register CHEACCINV"]
pub type R = crate::R<u32, super::CHEACCINV>;
#[doc = "Writer for register CHEACCINV"]
pub type W = crate::W<u32, super::CHEACCINV>;
#[doc = "Register CHEACCINV `reset()`'s with value 0"]
impl crate::ResetValue for super::CHEACCINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHEIDX`"]
pub type CHEIDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHEIDX`"]
pub struct CHEIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEIDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CHEWEN`"]
pub type CHEWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHEWEN`"]
pub struct CHEWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEWEN_W<'a> {
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
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cheidx(&self) -> CHEIDX_R {
        CHEIDX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn chewen(&self) -> CHEWEN_R {
        CHEWEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cheidx(&mut self) -> CHEIDX_W {
        CHEIDX_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn chewen(&mut self) -> CHEWEN_W {
        CHEWEN_W { w: self }
    }
}
