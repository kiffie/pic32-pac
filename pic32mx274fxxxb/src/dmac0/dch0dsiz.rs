#[doc = "Reader of register DCH0DSIZ"]
pub type R = crate::R<u32, super::DCH0DSIZ>;
#[doc = "Writer for register DCH0DSIZ"]
pub type W = crate::W<u32, super::DCH0DSIZ>;
#[doc = "Register DCH0DSIZ `reset()`'s with value 0"]
impl crate::ResetValue for super::DCH0DSIZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHDSIZ`"]
pub type CHDSIZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CHDSIZ`"]
pub struct CHDSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDSIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chdsiz(&self) -> CHDSIZ_R {
        CHDSIZ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chdsiz(&mut self) -> CHDSIZ_W {
        CHDSIZ_W { w: self }
    }
}
