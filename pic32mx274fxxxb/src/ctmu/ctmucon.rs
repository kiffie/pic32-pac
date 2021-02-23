#[doc = "Register `CTMUCON` reader"]
pub struct R(crate::R<CTMUCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTMUCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CTMUCON_SPEC>> for R {
    fn from(reader: crate::R<CTMUCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTMUCON` writer"]
pub struct W(crate::W<CTMUCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTMUCON_SPEC>;
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
impl core::convert::From<crate::W<CTMUCON_SPEC>> for W {
    fn from(writer: crate::W<CTMUCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRNG` reader - "]
pub struct IRNG_R(crate::FieldReader<u8, u8>);
impl IRNG_R {
    pub(crate) fn new(bits: u8) -> Self {
        IRNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRNG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRNG` writer - "]
pub struct IRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> IRNG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Field `ITRIM` reader - "]
pub struct ITRIM_R(crate::FieldReader<u8, u8>);
impl ITRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ITRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITRIM` writer - "]
pub struct ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Field `CTTRIG` reader - "]
pub struct CTTRIG_R(crate::FieldReader<bool, bool>);
impl CTTRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTTRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTTRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTTRIG` writer - "]
pub struct CTTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTRIG_W<'a> {
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
#[doc = "Field `IDISSEN` reader - "]
pub struct IDISSEN_R(crate::FieldReader<bool, bool>);
impl IDISSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDISSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDISSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDISSEN` writer - "]
pub struct IDISSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IDISSEN_W<'a> {
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
#[doc = "Field `EDGSEQEN` reader - "]
pub struct EDGSEQEN_R(crate::FieldReader<bool, bool>);
impl EDGSEQEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDGSEQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGSEQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGSEQEN` writer - "]
pub struct EDGSEQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGSEQEN_W<'a> {
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
#[doc = "Field `EDGEN` reader - "]
pub struct EDGEN_R(crate::FieldReader<bool, bool>);
impl EDGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGEN` writer - "]
pub struct EDGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `TGEN` reader - "]
pub struct TGEN_R(crate::FieldReader<bool, bool>);
impl TGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TGEN` writer - "]
pub struct TGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CTMUSIDL` reader - "]
pub struct CTMUSIDL_R(crate::FieldReader<bool, bool>);
impl CTMUSIDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTMUSIDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTMUSIDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTMUSIDL` writer - "]
pub struct CTMUSIDL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMUSIDL_W<'a> {
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
#[doc = "Field `FRZ` reader - "]
pub struct FRZ_R(crate::FieldReader<bool, bool>);
impl FRZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRZ` writer - "]
pub struct FRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
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
#[doc = "Field `EDG2SEL` reader - "]
pub struct EDG2SEL_R(crate::FieldReader<u8, u8>);
impl EDG2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EDG2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDG2SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDG2SEL` writer - "]
pub struct EDG2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDG2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Field `EDG2POL` reader - "]
pub struct EDG2POL_R(crate::FieldReader<bool, bool>);
impl EDG2POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDG2POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDG2POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDG2POL` writer - "]
pub struct EDG2POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDG2POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `EDG2MOD` reader - "]
pub struct EDG2MOD_R(crate::FieldReader<bool, bool>);
impl EDG2MOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDG2MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDG2MOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDG2MOD` writer - "]
pub struct EDG2MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> EDG2MOD_W<'a> {
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
#[doc = "Field `EDG1STAT` reader - "]
pub struct EDG1STAT_R(crate::FieldReader<bool, bool>);
impl EDG1STAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDG1STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDG1STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDG1STAT` writer - "]
pub struct EDG1STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> EDG1STAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `EDG2STAT` reader - "]
pub struct EDG2STAT_R(crate::FieldReader<bool, bool>);
impl EDG2STAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDG2STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDG2STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDG2STAT` writer - "]
pub struct EDG2STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> EDG2STAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `EDG1SEL` reader - "]
pub struct EDG1SEL_R(crate::FieldReader<u8, u8>);
impl EDG1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EDG1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDG1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDG1SEL` writer - "]
pub struct EDG1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDG1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 26)) | (((value as u32) & 0x0f) << 26);
        self.w
    }
}
#[doc = "Field `EDG1POL` reader - "]
pub struct EDG1POL_R(crate::FieldReader<bool, bool>);
impl EDG1POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDG1POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDG1POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDG1POL` writer - "]
pub struct EDG1POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDG1POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `EDG1MOD` reader - "]
pub struct EDG1MOD_R(crate::FieldReader<bool, bool>);
impl EDG1MOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDG1MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDG1MOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDG1MOD` writer - "]
pub struct EDG1MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> EDG1MOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn irng(&self) -> IRNG_R {
        IRNG_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn itrim(&self) -> ITRIM_R {
        ITRIM_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cttrig(&self) -> CTTRIG_R {
        CTTRIG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn idissen(&self) -> IDISSEN_R {
        IDISSEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn edgseqen(&self) -> EDGSEQEN_R {
        EDGSEQEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn edgen(&self) -> EDGEN_R {
        EDGEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tgen(&self) -> TGEN_R {
        TGEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ctmusidl(&self) -> CTMUSIDL_R {
        CTMUSIDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn edg2sel(&self) -> EDG2SEL_R {
        EDG2SEL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn edg2pol(&self) -> EDG2POL_R {
        EDG2POL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn edg2mod(&self) -> EDG2MOD_R {
        EDG2MOD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn edg1stat(&self) -> EDG1STAT_R {
        EDG1STAT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn edg2stat(&self) -> EDG2STAT_R {
        EDG2STAT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn edg1sel(&self) -> EDG1SEL_R {
        EDG1SEL_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn edg1pol(&self) -> EDG1POL_R {
        EDG1POL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn edg1mod(&self) -> EDG1MOD_R {
        EDG1MOD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn irng(&mut self) -> IRNG_W {
        IRNG_W { w: self }
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn itrim(&mut self) -> ITRIM_W {
        ITRIM_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cttrig(&mut self) -> CTTRIG_W {
        CTTRIG_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn idissen(&mut self) -> IDISSEN_W {
        IDISSEN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn edgseqen(&mut self) -> EDGSEQEN_W {
        EDGSEQEN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn edgen(&mut self) -> EDGEN_W {
        EDGEN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tgen(&mut self) -> TGEN_W {
        TGEN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ctmusidl(&mut self) -> CTMUSIDL_W {
        CTMUSIDL_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&mut self) -> FRZ_W {
        FRZ_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W { w: self }
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn edg2sel(&mut self) -> EDG2SEL_W {
        EDG2SEL_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn edg2pol(&mut self) -> EDG2POL_W {
        EDG2POL_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn edg2mod(&mut self) -> EDG2MOD_W {
        EDG2MOD_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn edg1stat(&mut self) -> EDG1STAT_W {
        EDG1STAT_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn edg2stat(&mut self) -> EDG2STAT_W {
        EDG2STAT_W { w: self }
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn edg1sel(&mut self) -> EDG1SEL_W {
        EDG1SEL_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn edg1pol(&mut self) -> EDG1POL_W {
        EDG1POL_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn edg1mod(&mut self) -> EDG1MOD_W {
        EDG1MOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTMUCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctmucon](index.html) module"]
pub struct CTMUCON_SPEC;
impl crate::RegisterSpec for CTMUCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctmucon::R](R) reader structure"]
impl crate::Readable for CTMUCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctmucon::W](W) writer structure"]
impl crate::Writable for CTMUCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTMUCON to value 0"]
impl crate::Resettable for CTMUCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
