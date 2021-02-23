#[doc = "Register `ECON` reader"]
pub struct R(crate::R<ECON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ECON_SPEC>> for R {
    fn from(reader: crate::R<ECON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECON` writer"]
pub struct W(crate::W<ECON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECON_SPEC>;
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
impl core::convert::From<crate::W<ECON_SPEC>> for W {
    fn from(writer: crate::W<ECON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AIRQEN` reader - "]
pub struct AIRQEN_R(crate::FieldReader<bool, bool>);
impl AIRQEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIRQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIRQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIRQEN` writer - "]
pub struct AIRQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AIRQEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `SIRQEN` reader - "]
pub struct SIRQEN_R(crate::FieldReader<bool, bool>);
impl SIRQEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIRQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIRQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIRQEN` writer - "]
pub struct SIRQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIRQEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `PATEN` reader - "]
pub struct PATEN_R(crate::FieldReader<bool, bool>);
impl PATEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PATEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATEN` writer - "]
pub struct PATEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PATEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CABORT` reader - "]
pub struct CABORT_R(crate::FieldReader<bool, bool>);
impl CABORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CABORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CABORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CABORT` writer - "]
pub struct CABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> CABORT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CFORCE` reader - "]
pub struct CFORCE_R(crate::FieldReader<bool, bool>);
impl CFORCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFORCE` writer - "]
pub struct CFORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFORCE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CHSIRQ` reader - "]
pub struct CHSIRQ_R(crate::FieldReader<u8, u8>);
impl CHSIRQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHSIRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSIRQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSIRQ` writer - "]
pub struct CHSIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSIRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CHAIRQ` reader - "]
pub struct CHAIRQ_R(crate::FieldReader<u8, u8>);
impl CHAIRQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHAIRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHAIRQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHAIRQ` writer - "]
pub struct CHAIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAIRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn airqen(&self) -> AIRQEN_R {
        AIRQEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sirqen(&self) -> SIRQEN_R {
        SIRQEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn paten(&self) -> PATEN_R {
        PATEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cabort(&self) -> CABORT_R {
        CABORT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cforce(&self) -> CFORCE_R {
        CFORCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn chsirq(&self) -> CHSIRQ_R {
        CHSIRQ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn chairq(&self) -> CHAIRQ_R {
        CHAIRQ_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn airqen(&mut self) -> AIRQEN_W {
        AIRQEN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sirqen(&mut self) -> SIRQEN_W {
        SIRQEN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn paten(&mut self) -> PATEN_W {
        PATEN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cabort(&mut self) -> CABORT_W {
        CABORT_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cforce(&mut self) -> CFORCE_W {
        CFORCE_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn chsirq(&mut self) -> CHSIRQ_W {
        CHSIRQ_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn chairq(&mut self) -> CHAIRQ_W {
        CHAIRQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCH0ECON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [econ](index.html) module"]
pub struct ECON_SPEC;
impl crate::RegisterSpec for ECON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [econ::R](R) reader structure"]
impl crate::Readable for ECON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [econ::W](W) writer structure"]
impl crate::Writable for ECON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECON to value 0x00ff_ff00"]
impl crate::Resettable for ECON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_ff00
    }
}
