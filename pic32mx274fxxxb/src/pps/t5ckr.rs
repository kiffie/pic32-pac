#[doc = "Register `T5CKR` reader"]
pub struct R(crate::R<T5CKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T5CKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<T5CKR_SPEC>> for R {
    fn from(reader: crate::R<T5CKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T5CKR` writer"]
pub struct W(crate::W<T5CKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T5CKR_SPEC>;
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
impl core::convert::From<crate::W<T5CKR_SPEC>> for W {
    fn from(writer: crate::W<T5CKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T5CKR` reader - "]
pub struct T5CKR_R(crate::FieldReader<u8, u8>);
impl T5CKR_R {
    pub(crate) fn new(bits: u8) -> Self {
        T5CKR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T5CKR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T5CKR` writer - "]
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "T5CKR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t5ckr](index.html) module"]
pub struct T5CKR_SPEC;
impl crate::RegisterSpec for T5CKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t5ckr::R](R) reader structure"]
impl crate::Readable for T5CKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t5ckr::W](W) writer structure"]
impl crate::Writable for T5CKR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T5CKR to value 0"]
impl crate::Resettable for T5CKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
