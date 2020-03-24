#[doc = "Reader of register ADC1BUFA"]
pub type R = crate::R<u32, super::ADC1BUFA>;
#[doc = "Writer for register ADC1BUFA"]
pub type W = crate::W<u32, super::ADC1BUFA>;
#[doc = "Register ADC1BUFA `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC1BUFA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC1BUFA`"]
pub type ADC1BUFA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADC1BUFA`"]
pub struct ADC1BUFA_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1BUFA_W<'a> {
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
    pub fn adc1bufa(&self) -> ADC1BUFA_R {
        ADC1BUFA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn adc1bufa(&mut self) -> ADC1BUFA_W {
        ADC1BUFA_W { w: self }
    }
}
