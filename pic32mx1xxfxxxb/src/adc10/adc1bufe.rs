#[doc = "Reader of register ADC1BUFE"]
pub type R = crate::R<u32, super::ADC1BUFE>;
#[doc = "Writer for register ADC1BUFE"]
pub type W = crate::W<u32, super::ADC1BUFE>;
#[doc = "Register ADC1BUFE `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC1BUFE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC1BUFE`"]
pub type ADC1BUFE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADC1BUFE`"]
pub struct ADC1BUFE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1BUFE_W<'a> {
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
    pub fn adc1bufe(&self) -> ADC1BUFE_R {
        ADC1BUFE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn adc1bufe(&mut self) -> ADC1BUFE_W {
        ADC1BUFE_W { w: self }
    }
}
