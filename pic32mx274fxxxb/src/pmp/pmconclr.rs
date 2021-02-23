#[doc = "Register `PMCONCLR` reader"]
pub struct R(crate::R<PMCONCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMCONCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMCONCLR_SPEC>> for R {
    fn from(reader: crate::R<PMCONCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMCONCLR` writer"]
pub struct W(crate::W<PMCONCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMCONCLR_SPEC>;
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
impl core::convert::From<crate::W<PMCONCLR_SPEC>> for W {
    fn from(writer: crate::W<PMCONCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDSP` reader - "]
pub struct RDSP_R(crate::FieldReader<bool, bool>);
impl RDSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDSP` writer - "]
pub struct RDSP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDSP_W<'a> {
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
#[doc = "Field `WRSP` reader - "]
pub struct WRSP_R(crate::FieldReader<bool, bool>);
impl WRSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRSP` writer - "]
pub struct WRSP_W<'a> {
    w: &'a mut W,
}
impl<'a> WRSP_W<'a> {
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
#[doc = "Field `CS1P` reader - "]
pub struct CS1P_R(crate::FieldReader<bool, bool>);
impl CS1P_R {
    pub(crate) fn new(bits: bool) -> Self {
        CS1P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS1P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS1P` writer - "]
pub struct CS1P_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1P_W<'a> {
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
#[doc = "Field `ALP` reader - "]
pub struct ALP_R(crate::FieldReader<bool, bool>);
impl ALP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALP` writer - "]
pub struct ALP_W<'a> {
    w: &'a mut W,
}
impl<'a> ALP_W<'a> {
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
#[doc = "Field `CSF` reader - "]
pub struct CSF_R(crate::FieldReader<u8, u8>);
impl CSF_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSF` writer - "]
pub struct CSF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `PTRDEN` reader - "]
pub struct PTRDEN_R(crate::FieldReader<bool, bool>);
impl PTRDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTRDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTRDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTRDEN` writer - "]
pub struct PTRDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTRDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `PTWREN` reader - "]
pub struct PTWREN_R(crate::FieldReader<bool, bool>);
impl PTWREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTWREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTWREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTWREN` writer - "]
pub struct PTWREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTWREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `PMPTTL` reader - "]
pub struct PMPTTL_R(crate::FieldReader<bool, bool>);
impl PMPTTL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMPTTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMPTTL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMPTTL` writer - "]
pub struct PMPTTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PMPTTL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `ADRMUX` reader - "]
pub struct ADRMUX_R(crate::FieldReader<u8, u8>);
impl ADRMUX_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADRMUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADRMUX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADRMUX` writer - "]
pub struct ADRMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRMUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `SIDL` reader - "]
pub struct SIDL_R(crate::FieldReader<bool, bool>);
impl SIDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIDL` writer - "]
pub struct SIDL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `ON` reader - "]
pub struct ON_R(crate::FieldReader<bool, bool>);
impl ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ON` writer - "]
pub struct ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `DUALBUF` reader - "]
pub struct DUALBUF_R(crate::FieldReader<bool, bool>);
impl DUALBUF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUALBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUALBUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUALBUF` writer - "]
pub struct DUALBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DUALBUF_W<'a> {
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
#[doc = "Field `RDSTART` reader - "]
pub struct RDSTART_R(crate::FieldReader<bool, bool>);
impl RDSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDSTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDSTART` writer - "]
pub struct RDSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RDSTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rdsp(&self) -> RDSP_R {
        RDSP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wrsp(&self) -> WRSP_R {
        WRSP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cs1p(&self) -> CS1P_R {
        CS1P_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn alp(&self) -> ALP_R {
        ALP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn csf(&self) -> CSF_R {
        CSF_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ptrden(&self) -> PTRDEN_R {
        PTRDEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ptwren(&self) -> PTWREN_R {
        PTWREN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pmpttl(&self) -> PMPTTL_R {
        PMPTTL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn adrmux(&self) -> ADRMUX_R {
        ADRMUX_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&self) -> SIDL_R {
        SIDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dualbuf(&self) -> DUALBUF_R {
        DUALBUF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rdstart(&self) -> RDSTART_R {
        RDSTART_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rdsp(&mut self) -> RDSP_W {
        RDSP_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wrsp(&mut self) -> WRSP_W {
        WRSP_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cs1p(&mut self) -> CS1P_W {
        CS1P_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn alp(&mut self) -> ALP_W {
        ALP_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn csf(&mut self) -> CSF_W {
        CSF_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ptrden(&mut self) -> PTRDEN_W {
        PTRDEN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ptwren(&mut self) -> PTWREN_W {
        PTWREN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pmpttl(&mut self) -> PMPTTL_W {
        PMPTTL_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn adrmux(&mut self) -> ADRMUX_W {
        ADRMUX_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&mut self) -> SIDL_W {
        SIDL_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dualbuf(&mut self) -> DUALBUF_W {
        DUALBUF_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rdstart(&mut self) -> RDSTART_W {
        RDSTART_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmconclr](index.html) module"]
pub struct PMCONCLR_SPEC;
impl crate::RegisterSpec for PMCONCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmconclr::R](R) reader structure"]
impl crate::Readable for PMCONCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmconclr::W](W) writer structure"]
impl crate::Writable for PMCONCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMCONCLR to value 0"]
impl crate::Resettable for PMCONCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
