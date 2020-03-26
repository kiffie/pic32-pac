#[doc = "Reader of register RPC8R"]
pub type R = crate::R<u32, super::RPC8R>;
#[doc = "Writer for register RPC8R"]
pub type W = crate::W<u32, super::RPC8R>;
#[doc = "Register RPC8R `reset()`'s with value 0"]
impl crate::ResetValue for super::RPC8R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPC8R`"]
pub type RPC8R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPC8R`"]
pub struct RPC8R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPC8R_W<'a> {
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
    pub fn rpc8r(&self) -> RPC8R_R {
        RPC8R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpc8r(&mut self) -> RPC8R_W {
        RPC8R_W { w: self }
    }
}