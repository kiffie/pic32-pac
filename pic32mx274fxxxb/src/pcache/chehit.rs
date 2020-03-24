#[doc = "Reader of register CHEHIT"]
pub type R = crate::R<u32, super::CHEHIT>;
#[doc = "Writer for register CHEHIT"]
pub type W = crate::W<u32, super::CHEHIT>;
#[doc = "Register CHEHIT `reset()`'s with value 0"]
impl crate::ResetValue for super::CHEHIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHEHIT`"]
pub type CHEHIT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CHEHIT`"]
pub struct CHEHIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEHIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn chehit(&self) -> CHEHIT_R {
        CHEHIT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn chehit(&mut self) -> CHEHIT_W {
        CHEHIT_W { w: self }
    }
}
