#[doc = "Register `ADC1BUFF` reader"]
pub struct R(crate::R<ADC1BUFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1BUFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADC1BUFF_SPEC>> for R {
    fn from(reader: crate::R<ADC1BUFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1BUFF` writer"]
pub struct W(crate::W<ADC1BUFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1BUFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<ADC1BUFF_SPEC>> for W {
    fn from(writer: crate::W<ADC1BUFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC1BUFF` reader - "]
pub struct ADC1BUFF_R(crate::FieldReader<u32, u32>);
impl ADC1BUFF_R {
    pub(crate) fn new(bits: u32) -> Self {
        ADC1BUFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1BUFF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1BUFF` writer - "]
pub struct ADC1BUFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1BUFF_W<'a> {
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
    pub fn adc1buff(&self) -> ADC1BUFF_R {
        ADC1BUFF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn adc1buff(&mut self) -> ADC1BUFF_W {
        ADC1BUFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1BUFF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1buff](index.html) module"]
pub struct ADC1BUFF_SPEC;
impl crate::RegisterSpec for ADC1BUFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1buff::R](R) reader structure"]
impl crate::Readable for ADC1BUFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1buff::W](W) writer structure"]
impl crate::Writable for ADC1BUFF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC1BUFF to value 0"]
impl crate::Resettable for ADC1BUFF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
