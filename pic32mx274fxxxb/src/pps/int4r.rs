#[doc = "Reader of register INT4R"]
pub type R = crate::R<u32, super::INT4R>;
#[doc = "Writer for register INT4R"]
pub type W = crate::W<u32, super::INT4R>;
#[doc = "Register INT4R `reset()`'s with value 0"]
impl crate::ResetValue for super::INT4R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INT4R`"]
pub type INT4R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT4R`"]
pub struct INT4R_W<'a> {
    w: &'a mut W,
}
impl<'a> INT4R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn int4r(&self) -> INT4R_R {
        INT4R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn int4r(&mut self) -> INT4R_W {
        INT4R_W { w: self }
    }
}
