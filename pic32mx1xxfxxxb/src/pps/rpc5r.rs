#[doc = "Reader of register RPC5R"]
pub type R = crate::R<u32, super::RPC5R>;
#[doc = "Writer for register RPC5R"]
pub type W = crate::W<u32, super::RPC5R>;
#[doc = "Register RPC5R `reset()`'s with value 0"]
impl crate::ResetValue for super::RPC5R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPC5R`"]
pub type RPC5R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPC5R`"]
pub struct RPC5R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPC5R_W<'a> {
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
    pub fn rpc5r(&self) -> RPC5R_R {
        RPC5R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpc5r(&mut self) -> RPC5R_W {
        RPC5R_W { w: self }
    }
}
