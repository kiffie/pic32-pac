#[doc = "Register `BRGSET` reader"]
pub struct R(crate::R<BRGSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRGSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BRGSET_SPEC>> for R {
    fn from(reader: crate::R<BRGSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRGSET` writer"]
pub struct W(crate::W<BRGSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRGSET_SPEC>;
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
impl core::convert::From<crate::W<BRGSET_SPEC>> for W {
    fn from(writer: crate::W<BRGSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRG` reader - "]
pub struct BRG_R(crate::FieldReader<u16, u16>);
impl BRG_R {
    pub(crate) fn new(bits: u16) -> Self {
        BRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRG` writer - "]
pub struct BRG_W<'a> {
    w: &'a mut W,
}
impl<'a> BRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn brg(&self) -> BRG_R {
        BRG_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn brg(&mut self) -> BRG_W {
        BRG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1BRGSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brgset](index.html) module"]
pub struct BRGSET_SPEC;
impl crate::RegisterSpec for BRGSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brgset::R](R) reader structure"]
impl crate::Readable for BRGSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brgset::W](W) writer structure"]
impl crate::Writable for BRGSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRGSET to value 0"]
impl crate::Resettable for BRGSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
