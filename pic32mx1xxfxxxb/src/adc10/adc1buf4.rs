#[doc = "Register `ADC1BUF4` reader"]
pub struct R(crate::R<ADC1BUF4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1BUF4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ADC1BUF4_SPEC>> for R {
    fn from(reader: crate::R<ADC1BUF4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1BUF4` writer"]
pub struct W(crate::W<ADC1BUF4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1BUF4_SPEC>;
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
impl core::convert::From<crate::W<ADC1BUF4_SPEC>> for W {
    fn from(writer: crate::W<ADC1BUF4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC1BUF4` reader - "]
pub struct ADC1BUF4_R(crate::FieldReader<u32, u32>);
impl ADC1BUF4_R {
    pub(crate) fn new(bits: u32) -> Self {
        ADC1BUF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1BUF4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1BUF4` writer - "]
pub struct ADC1BUF4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1BUF4_W<'a> {
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
    pub fn adc1buf4(&self) -> ADC1BUF4_R {
        ADC1BUF4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn adc1buf4(&mut self) -> ADC1BUF4_W {
        ADC1BUF4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1BUF4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1buf4](index.html) module"]
pub struct ADC1BUF4_SPEC;
impl crate::RegisterSpec for ADC1BUF4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1buf4::R](R) reader structure"]
impl crate::Readable for ADC1BUF4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1buf4::W](W) writer structure"]
impl crate::Writable for ADC1BUF4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC1BUF4 to value 0"]
impl crate::Resettable for ADC1BUF4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
