#[doc = "Reader of register DCRCDATA"]
pub type R = crate::R<u32, super::DCRCDATA>;
#[doc = "Writer for register DCRCDATA"]
pub type W = crate::W<u32, super::DCRCDATA>;
#[doc = "Register DCRCDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::DCRCDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCRCDATA`"]
pub type DCRCDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DCRCDATA`"]
pub struct DCRCDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRCDATA_W<'a> {
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
    pub fn dcrcdata(&self) -> DCRCDATA_R {
        DCRCDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dcrcdata(&mut self) -> DCRCDATA_W {
        DCRCDATA_W { w: self }
    }
}
