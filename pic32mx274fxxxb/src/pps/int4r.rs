#[doc = "Register `INT4R` reader"]
pub struct R(crate::R<INT4R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT4R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INT4R_SPEC>> for R {
    fn from(reader: crate::R<INT4R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT4R` writer"]
pub struct W(crate::W<INT4R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT4R_SPEC>;
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
impl core::convert::From<crate::W<INT4R_SPEC>> for W {
    fn from(writer: crate::W<INT4R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT4R` reader - "]
pub struct INT4R_R(crate::FieldReader<u8, u8>);
impl INT4R_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT4R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT4R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT4R` writer - "]
pub struct INT4R_W<'a> {
    w: &'a mut W,
}
impl<'a> INT4R_W<'a> {
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
    pub fn int4r(&self) -> INT4R_R {
        INT4R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn int4r(&mut self) -> INT4R_W {
        INT4R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INT4R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int4r](index.html) module"]
pub struct INT4R_SPEC;
impl crate::RegisterSpec for INT4R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int4r::R](R) reader structure"]
impl crate::Readable for INT4R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int4r::W](W) writer structure"]
impl crate::Writable for INT4R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT4R to value 0"]
impl crate::Resettable for INT4R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
