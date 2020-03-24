#[doc = "Reader of register T5CKR"]
pub type R = crate::R<u32, super::T5CKR>;
#[doc = "Writer for register T5CKR"]
pub type W = crate::W<u32, super::T5CKR>;
#[doc = "Register T5CKR `reset()`'s with value 0"]
impl crate::ResetValue for super::T5CKR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T5CKR`"]
pub type T5CKR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T5CKR`"]
pub struct T5CKR_W<'a> {
    w: &'a mut W,
}
impl<'a> T5CKR_W<'a> {
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
    pub fn t5ckr(&self) -> T5CKR_R {
        T5CKR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn t5ckr(&mut self) -> T5CKR_W {
        T5CKR_W { w: self }
    }
}
