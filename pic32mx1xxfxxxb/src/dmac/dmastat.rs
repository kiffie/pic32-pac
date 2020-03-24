#[doc = "Reader of register DMASTAT"]
pub type R = crate::R<u32, super::DMASTAT>;
#[doc = "Writer for register DMASTAT"]
pub type W = crate::W<u32, super::DMASTAT>;
#[doc = "Register DMASTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::DMASTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMACH`"]
pub type DMACH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMACH`"]
pub struct DMACH_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `RDWR`"]
pub type RDWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDWR`"]
pub struct RDWR_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWR_W<'a> {
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
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dmach(&self) -> DMACH_R {
        DMACH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rdwr(&self) -> RDWR_R {
        RDWR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dmach(&mut self) -> DMACH_W {
        DMACH_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rdwr(&mut self) -> RDWR_W {
        RDWR_W { w: self }
    }
}
