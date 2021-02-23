#[doc = "Register `RCV` reader"]
pub struct R(crate::R<RCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RCV_SPEC>> for R {
    fn from(reader: crate::R<RCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCV` writer"]
pub struct W(crate::W<RCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCV_SPEC>;
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
impl core::convert::From<crate::W<RCV_SPEC>> for W {
    fn from(writer: crate::W<RCV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCV` reader - "]
pub struct RCV_R(crate::FieldReader<u8, u8>);
impl RCV_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCV` writer - "]
pub struct RCV_W<'a> {
    w: &'a mut W,
}
impl<'a> RCV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rcv(&self) -> RCV_R {
        RCV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rcv(&mut self) -> RCV_W {
        RCV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C1RCV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcv](index.html) module"]
pub struct RCV_SPEC;
impl crate::RegisterSpec for RCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcv::R](R) reader structure"]
impl crate::Readable for RCV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcv::W](W) writer structure"]
impl crate::Writable for RCV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCV to value 0"]
impl crate::Resettable for RCV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
