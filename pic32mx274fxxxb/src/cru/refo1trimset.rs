#[doc = "Reader of register REFO1TRIMSET"]
pub type R = crate::R<u32, super::REFO1TRIMSET>;
#[doc = "Writer for register REFO1TRIMSET"]
pub type W = crate::W<u32, super::REFO1TRIMSET>;
#[doc = "Register REFO1TRIMSET `reset()`'s with value 0"]
impl crate::ResetValue for super::REFO1TRIMSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ROTRIM`"]
pub type ROTRIM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ROTRIM`"]
pub struct ROTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 23)) | (((value as u32) & 0x01ff) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 23:31"]
    #[inline(always)]
    pub fn rotrim(&self) -> ROTRIM_R {
        ROTRIM_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 23:31"]
    #[inline(always)]
    pub fn rotrim(&mut self) -> ROTRIM_W {
        ROTRIM_W { w: self }
    }
}
