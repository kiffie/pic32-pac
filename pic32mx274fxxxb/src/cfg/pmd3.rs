#[doc = "Register `PMD3` reader"]
pub struct R(crate::R<PMD3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMD3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMD3_SPEC>> for R {
    fn from(reader: crate::R<PMD3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMD3` writer"]
pub struct W(crate::W<PMD3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMD3_SPEC>;
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
impl core::convert::From<crate::W<PMD3_SPEC>> for W {
    fn from(writer: crate::W<PMD3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC1MD` reader - "]
pub struct IC1MD_R(crate::FieldReader<bool, bool>);
impl IC1MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC1MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC1MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC1MD` writer - "]
pub struct IC1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1MD_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Field `IC2MD` reader - "]
pub struct IC2MD_R(crate::FieldReader<bool, bool>);
impl IC2MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC2MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC2MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC2MD` writer - "]
pub struct IC2MD_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2MD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `IC3MD` reader - "]
pub struct IC3MD_R(crate::FieldReader<bool, bool>);
impl IC3MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC3MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC3MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC3MD` writer - "]
pub struct IC3MD_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3MD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `IC4MD` reader - "]
pub struct IC4MD_R(crate::FieldReader<bool, bool>);
impl IC4MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC4MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC4MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC4MD` writer - "]
pub struct IC4MD_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4MD_W<'a> {
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
#[doc = "Field `IC5MD` reader - "]
pub struct IC5MD_R(crate::FieldReader<bool, bool>);
impl IC5MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC5MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC5MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC5MD` writer - "]
pub struct IC5MD_W<'a> {
    w: &'a mut W,
}
impl<'a> IC5MD_W<'a> {
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
#[doc = "Field `OC1MD` reader - "]
pub struct OC1MD_R(crate::FieldReader<bool, bool>);
impl OC1MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC1MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC1MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC1MD` writer - "]
pub struct OC1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1MD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `OC2MD` reader - "]
pub struct OC2MD_R(crate::FieldReader<bool, bool>);
impl OC2MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC2MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC2MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC2MD` writer - "]
pub struct OC2MD_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2MD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `OC3MD` reader - "]
pub struct OC3MD_R(crate::FieldReader<bool, bool>);
impl OC3MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC3MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC3MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC3MD` writer - "]
pub struct OC3MD_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3MD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `OC4MD` reader - "]
pub struct OC4MD_R(crate::FieldReader<bool, bool>);
impl OC4MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC4MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC4MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC4MD` writer - "]
pub struct OC4MD_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4MD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `OC5MD` reader - "]
pub struct OC5MD_R(crate::FieldReader<bool, bool>);
impl OC5MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC5MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC5MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC5MD` writer - "]
pub struct OC5MD_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5MD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ic1md(&self) -> IC1MD_R {
        IC1MD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ic2md(&self) -> IC2MD_R {
        IC2MD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ic3md(&self) -> IC3MD_R {
        IC3MD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ic4md(&self) -> IC4MD_R {
        IC4MD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ic5md(&self) -> IC5MD_R {
        IC5MD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn oc1md(&self) -> OC1MD_R {
        OC1MD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn oc2md(&self) -> OC2MD_R {
        OC2MD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn oc3md(&self) -> OC3MD_R {
        OC3MD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn oc4md(&self) -> OC4MD_R {
        OC4MD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn oc5md(&self) -> OC5MD_R {
        OC5MD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ic1md(&mut self) -> IC1MD_W {
        IC1MD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ic2md(&mut self) -> IC2MD_W {
        IC2MD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ic3md(&mut self) -> IC3MD_W {
        IC3MD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ic4md(&mut self) -> IC4MD_W {
        IC4MD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ic5md(&mut self) -> IC5MD_W {
        IC5MD_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn oc1md(&mut self) -> OC1MD_W {
        OC1MD_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn oc2md(&mut self) -> OC2MD_W {
        OC2MD_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn oc3md(&mut self) -> OC3MD_W {
        OC3MD_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn oc4md(&mut self) -> OC4MD_W {
        OC4MD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn oc5md(&mut self) -> OC5MD_W {
        OC5MD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMD3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd3](index.html) module"]
pub struct PMD3_SPEC;
impl crate::RegisterSpec for PMD3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmd3::R](R) reader structure"]
impl crate::Readable for PMD3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmd3::W](W) writer structure"]
impl crate::Writable for PMD3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMD3 to value 0"]
impl crate::Resettable for PMD3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
