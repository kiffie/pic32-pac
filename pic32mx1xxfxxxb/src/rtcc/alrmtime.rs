#[doc = "Reader of register ALRMTIME"]
pub type R = crate::R<u32, super::ALRMTIME>;
#[doc = "Writer for register ALRMTIME"]
pub type W = crate::W<u32, super::ALRMTIME>;
#[doc = "Register ALRMTIME `reset()`'s with value 0"]
impl crate::ResetValue for super::ALRMTIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEC01`"]
pub type SEC01_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEC01`"]
pub struct SEC01_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SEC10`"]
pub type SEC10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEC10`"]
pub struct SEC10_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `MIN01`"]
pub type MIN01_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MIN01`"]
pub struct MIN01_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MIN10`"]
pub type MIN10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MIN10`"]
pub struct MIN10_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `HR01`"]
pub type HR01_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HR01`"]
pub struct HR01_W<'a> {
    w: &'a mut W,
}
impl<'a> HR01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `HR10`"]
pub type HR10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HR10`"]
pub struct HR10_W<'a> {
    w: &'a mut W,
}
impl<'a> HR10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn sec01(&self) -> SEC01_R {
        SEC01_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn min01(&self) -> MIN01_R {
        MIN01_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn min10(&self) -> MIN10_R {
        MIN10_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn hr01(&self) -> HR01_R {
        HR01_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn hr10(&self) -> HR10_R {
        HR10_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn sec01(&mut self) -> SEC01_W {
        SEC01_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn sec10(&mut self) -> SEC10_W {
        SEC10_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn min01(&mut self) -> MIN01_W {
        MIN01_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn min10(&mut self) -> MIN10_W {
        MIN10_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn hr01(&mut self) -> HR01_W {
        HR01_W { w: self }
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn hr10(&mut self) -> HR10_W {
        HR10_W { w: self }
    }
}
