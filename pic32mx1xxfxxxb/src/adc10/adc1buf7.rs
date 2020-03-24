#[doc = "Reader of register ADC1BUF7"]
pub type R = crate::R<u32, super::ADC1BUF7>;
#[doc = "Writer for register ADC1BUF7"]
pub type W = crate::W<u32, super::ADC1BUF7>;
#[doc = "Register ADC1BUF7 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC1BUF7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC1BUF7`"]
pub type ADC1BUF7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADC1BUF7`"]
pub struct ADC1BUF7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1BUF7_W<'a> {
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
    pub fn adc1buf7(&self) -> ADC1BUF7_R {
        ADC1BUF7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn adc1buf7(&mut self) -> ADC1BUF7_W {
        ADC1BUF7_W { w: self }
    }
}
