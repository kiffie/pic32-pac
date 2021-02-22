#[doc = "Register `INT1R` reader"]
pub struct R(crate::R<INT1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INT1R_SPEC>> for R {
    fn from(reader: crate::R<INT1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT1R` writer"]
pub struct W(crate::W<INT1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT1R_SPEC>;
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
impl core::convert::From<crate::W<INT1R_SPEC>> for W {
    fn from(writer: crate::W<INT1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT1R` reader - "]
pub struct INT1R_R(crate::FieldReader<u8, u8>);
impl INT1R_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT1R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT1R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT1R` writer - "]
pub struct INT1R_W<'a> {
    w: &'a mut W,
}
impl<'a> INT1R_W<'a> {
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
    pub fn int1r(&self) -> INT1R_R {
        INT1R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn int1r(&mut self) -> INT1R_W {
        INT1R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INT1R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int1r](index.html) module"]
pub struct INT1R_SPEC;
impl crate::RegisterSpec for INT1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int1r::R](R) reader structure"]
impl crate::Readable for INT1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int1r::W](W) writer structure"]
impl crate::Writable for INT1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT1R to value 0"]
impl crate::Resettable for INT1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
