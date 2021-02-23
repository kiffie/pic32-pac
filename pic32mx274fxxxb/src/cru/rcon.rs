#[doc = "Register `RCON` reader"]
pub struct R(crate::R<RCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RCON_SPEC>> for R {
    fn from(reader: crate::R<RCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCON` writer"]
pub struct W(crate::W<RCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCON_SPEC>;
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
impl core::convert::From<crate::W<RCON_SPEC>> for W {
    fn from(writer: crate::W<RCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POR` reader - "]
pub struct POR_R(crate::FieldReader<bool, bool>);
impl POR_R {
    pub(crate) fn new(bits: bool) -> Self {
        POR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR` writer - "]
pub struct POR_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_W<'a> {
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
#[doc = "Field `BOR` reader - "]
pub struct BOR_R(crate::FieldReader<bool, bool>);
impl BOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOR` writer - "]
pub struct BOR_W<'a> {
    w: &'a mut W,
}
impl<'a> BOR_W<'a> {
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
#[doc = "Field `IDLE` reader - "]
pub struct IDLE_R(crate::FieldReader<bool, bool>);
impl IDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE` writer - "]
pub struct IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_W<'a> {
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
#[doc = "Field `SLEEP` reader - "]
pub struct SLEEP_R(crate::FieldReader<bool, bool>);
impl SLEEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEP` writer - "]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
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
#[doc = "Field `WDTO` reader - "]
pub struct WDTO_R(crate::FieldReader<bool, bool>);
impl WDTO_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTO` writer - "]
pub struct WDTO_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTO_W<'a> {
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
#[doc = "Field `SWR` reader - "]
pub struct SWR_R(crate::FieldReader<bool, bool>);
impl SWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWR` writer - "]
pub struct SWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWR_W<'a> {
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
#[doc = "Field `EXTR` reader - "]
pub struct EXTR_R(crate::FieldReader<bool, bool>);
impl EXTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTR` writer - "]
pub struct EXTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTR_W<'a> {
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
#[doc = "Field `CMR` reader - "]
pub struct CMR_R(crate::FieldReader<bool, bool>);
impl CMR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMR` writer - "]
pub struct CMR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMR_W<'a> {
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
#[doc = "Field `DPSLP` reader - "]
pub struct DPSLP_R(crate::FieldReader<bool, bool>);
impl DPSLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPSLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPSLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPSLP` writer - "]
pub struct DPSLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_W<'a> {
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
#[doc = "Field `VBAT` reader - "]
pub struct VBAT_R(crate::FieldReader<bool, bool>);
impl VBAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBAT` writer - "]
pub struct VBAT_W<'a> {
    w: &'a mut W,
}
impl<'a> VBAT_W<'a> {
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
#[doc = "Field `VBPOR` reader - "]
pub struct VBPOR_R(crate::FieldReader<bool, bool>);
impl VBPOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBPOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBPOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBPOR` writer - "]
pub struct VBPOR_W<'a> {
    w: &'a mut W,
}
impl<'a> VBPOR_W<'a> {
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
#[doc = "Field `BCFGFAIL` reader - "]
pub struct BCFGFAIL_R(crate::FieldReader<bool, bool>);
impl BCFGFAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BCFGFAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCFGFAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCFGFAIL` writer - "]
pub struct BCFGFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> BCFGFAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `BCFGERR` reader - "]
pub struct BCFGERR_R(crate::FieldReader<bool, bool>);
impl BCFGERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BCFGERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCFGERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCFGERR` writer - "]
pub struct BCFGERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BCFGERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bor(&self) -> BOR_R {
        BOR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wdto(&self) -> WDTO_R {
        WDTO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn extr(&self) -> EXTR_R {
        EXTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cmr(&self) -> CMR_R {
        CMR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dpslp(&self) -> DPSLP_R {
        DPSLP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn vbat(&self) -> VBAT_R {
        VBAT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn vbpor(&self) -> VBPOR_R {
        VBPOR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bcfgfail(&self) -> BCFGFAIL_R {
        BCFGFAIL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bcfgerr(&self) -> BCFGERR_R {
        BCFGERR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn por(&mut self) -> POR_W {
        POR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bor(&mut self) -> BOR_W {
        BOR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn idle(&mut self) -> IDLE_W {
        IDLE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wdto(&mut self) -> WDTO_W {
        WDTO_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W {
        SWR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn extr(&mut self) -> EXTR_W {
        EXTR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cmr(&mut self) -> CMR_W {
        CMR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dpslp(&mut self) -> DPSLP_W {
        DPSLP_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn vbat(&mut self) -> VBAT_W {
        VBAT_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn vbpor(&mut self) -> VBPOR_W {
        VBPOR_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bcfgfail(&mut self) -> BCFGFAIL_W {
        BCFGFAIL_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bcfgerr(&mut self) -> BCFGERR_W {
        BCFGERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcon](index.html) module"]
pub struct RCON_SPEC;
impl crate::RegisterSpec for RCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcon::R](R) reader structure"]
impl crate::Readable for RCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcon::W](W) writer structure"]
impl crate::Writable for RCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCON to value 0"]
impl crate::Resettable for RCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
