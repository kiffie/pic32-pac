#[doc = "Reader of register AD1CHSCLR"]
pub type R = crate::R<u32, super::AD1CHSCLR>;
#[doc = "Writer for register AD1CHSCLR"]
pub type W = crate::W<u32, super::AD1CHSCLR>;
#[doc = "Register AD1CHSCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::AD1CHSCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0SA`"]
pub type CH0SA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH0SA`"]
pub struct CH0SA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CH0NA`"]
pub type CH0NA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0NA`"]
pub struct CH0NA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0NA_W<'a> {
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
#[doc = "Reader of field `CH0SB`"]
pub type CH0SB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH0SB`"]
pub struct CH0SB_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0SB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `CH0NB`"]
pub type CH0NB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0NB`"]
pub struct CH0NB_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0NB_W<'a> {
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
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ch0sa(&self) -> CH0SA_R {
        CH0SA_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ch0na(&self) -> CH0NA_R {
        CH0NA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn ch0sb(&self) -> CH0SB_R {
        CH0SB_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ch0nb(&self) -> CH0NB_R {
        CH0NB_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ch0sa(&mut self) -> CH0SA_W {
        CH0SA_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ch0na(&mut self) -> CH0NA_W {
        CH0NA_W { w: self }
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn ch0sb(&mut self) -> CH0SB_W {
        CH0SB_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ch0nb(&mut self) -> CH0NB_W {
        CH0NB_W { w: self }
    }
}
