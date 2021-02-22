#[doc = "Register `BUF` reader"]
pub struct R(crate::R<BUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BUF_SPEC>> for R {
    fn from(reader: crate::R<BUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF` writer"]
pub struct W(crate::W<BUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_SPEC>;
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
impl core::convert::From<crate::W<BUF_SPEC>> for W {
    fn from(writer: crate::W<BUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF` reader - "]
pub struct BUF_R(crate::FieldReader<u32, u32>);
impl BUF_R {
    pub(crate) fn new(bits: u32) -> Self {
        BUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF` writer - "]
pub struct BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_W<'a> {
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
    pub fn buf(&self) -> BUF_R {
        BUF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn buf(&mut self) -> BUF_W {
        BUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1BUF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf](index.html) module"]
pub struct BUF_SPEC;
impl crate::RegisterSpec for BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf::R](R) reader structure"]
impl crate::Readable for BUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf::W](W) writer structure"]
impl crate::Writable for BUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUF to value 0"]
impl crate::Resettable for BUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
