#[doc = "Register `IFS0SET` reader"]
pub struct R(crate::R<IFS0SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFS0SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IFS0SET_SPEC>> for R {
    fn from(reader: crate::R<IFS0SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFS0SET` writer"]
pub struct W(crate::W<IFS0SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFS0SET_SPEC>;
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
impl core::convert::From<crate::W<IFS0SET_SPEC>> for W {
    fn from(writer: crate::W<IFS0SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTIF` reader - "]
pub struct CTIF_R(crate::FieldReader<bool, bool>);
impl CTIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIF` writer - "]
pub struct CTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIF_W<'a> {
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
#[doc = "Field `CS0IF` reader - "]
pub struct CS0IF_R(crate::FieldReader<bool, bool>);
impl CS0IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CS0IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS0IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS0IF` writer - "]
pub struct CS0IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CS0IF_W<'a> {
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
#[doc = "Field `CS1IF` reader - "]
pub struct CS1IF_R(crate::FieldReader<bool, bool>);
impl CS1IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CS1IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS1IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS1IF` writer - "]
pub struct CS1IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1IF_W<'a> {
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
#[doc = "Field `INT0IF` reader - "]
pub struct INT0IF_R(crate::FieldReader<bool, bool>);
impl INT0IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT0IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT0IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT0IF` writer - "]
pub struct INT0IF_W<'a> {
    w: &'a mut W,
}
impl<'a> INT0IF_W<'a> {
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
#[doc = "Field `T1IF` reader - "]
pub struct T1IF_R(crate::FieldReader<bool, bool>);
impl T1IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        T1IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T1IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T1IF` writer - "]
pub struct T1IF_W<'a> {
    w: &'a mut W,
}
impl<'a> T1IF_W<'a> {
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
#[doc = "Field `IC1EIF` reader - "]
pub struct IC1EIF_R(crate::FieldReader<bool, bool>);
impl IC1EIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC1EIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC1EIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC1EIF` writer - "]
pub struct IC1EIF_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1EIF_W<'a> {
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
#[doc = "Field `IC1IF` reader - "]
pub struct IC1IF_R(crate::FieldReader<bool, bool>);
impl IC1IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC1IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC1IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC1IF` writer - "]
pub struct IC1IF_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1IF_W<'a> {
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
#[doc = "Field `OC1IF` reader - "]
pub struct OC1IF_R(crate::FieldReader<bool, bool>);
impl OC1IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC1IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC1IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC1IF` writer - "]
pub struct OC1IF_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1IF_W<'a> {
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
#[doc = "Field `INT1IF` reader - "]
pub struct INT1IF_R(crate::FieldReader<bool, bool>);
impl INT1IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT1IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT1IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT1IF` writer - "]
pub struct INT1IF_W<'a> {
    w: &'a mut W,
}
impl<'a> INT1IF_W<'a> {
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
#[doc = "Field `T2IF` reader - "]
pub struct T2IF_R(crate::FieldReader<bool, bool>);
impl T2IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        T2IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T2IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T2IF` writer - "]
pub struct T2IF_W<'a> {
    w: &'a mut W,
}
impl<'a> T2IF_W<'a> {
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
#[doc = "Field `IC2EIF` reader - "]
pub struct IC2EIF_R(crate::FieldReader<bool, bool>);
impl IC2EIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC2EIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC2EIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC2EIF` writer - "]
pub struct IC2EIF_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2EIF_W<'a> {
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
#[doc = "Field `IC2IF` reader - "]
pub struct IC2IF_R(crate::FieldReader<bool, bool>);
impl IC2IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC2IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC2IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC2IF` writer - "]
pub struct IC2IF_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2IF_W<'a> {
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
#[doc = "Field `OC2IF` reader - "]
pub struct OC2IF_R(crate::FieldReader<bool, bool>);
impl OC2IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC2IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC2IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC2IF` writer - "]
pub struct OC2IF_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2IF_W<'a> {
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
#[doc = "Field `INT2IF` reader - "]
pub struct INT2IF_R(crate::FieldReader<bool, bool>);
impl INT2IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT2IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT2IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT2IF` writer - "]
pub struct INT2IF_W<'a> {
    w: &'a mut W,
}
impl<'a> INT2IF_W<'a> {
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
#[doc = "Field `T3IF` reader - "]
pub struct T3IF_R(crate::FieldReader<bool, bool>);
impl T3IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        T3IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T3IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T3IF` writer - "]
pub struct T3IF_W<'a> {
    w: &'a mut W,
}
impl<'a> T3IF_W<'a> {
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
#[doc = "Field `IC3EIF` reader - "]
pub struct IC3EIF_R(crate::FieldReader<bool, bool>);
impl IC3EIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC3EIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC3EIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC3EIF` writer - "]
pub struct IC3EIF_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3EIF_W<'a> {
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
#[doc = "Field `IC3IF` reader - "]
pub struct IC3IF_R(crate::FieldReader<bool, bool>);
impl IC3IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC3IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC3IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC3IF` writer - "]
pub struct IC3IF_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3IF_W<'a> {
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
#[doc = "Field `OC3IF` reader - "]
pub struct OC3IF_R(crate::FieldReader<bool, bool>);
impl OC3IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC3IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC3IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC3IF` writer - "]
pub struct OC3IF_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3IF_W<'a> {
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
#[doc = "Field `INT3IF` reader - "]
pub struct INT3IF_R(crate::FieldReader<bool, bool>);
impl INT3IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT3IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT3IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT3IF` writer - "]
pub struct INT3IF_W<'a> {
    w: &'a mut W,
}
impl<'a> INT3IF_W<'a> {
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
#[doc = "Field `T4IF` reader - "]
pub struct T4IF_R(crate::FieldReader<bool, bool>);
impl T4IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        T4IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T4IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T4IF` writer - "]
pub struct T4IF_W<'a> {
    w: &'a mut W,
}
impl<'a> T4IF_W<'a> {
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
#[doc = "Field `IC4EIF` reader - "]
pub struct IC4EIF_R(crate::FieldReader<bool, bool>);
impl IC4EIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC4EIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC4EIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC4EIF` writer - "]
pub struct IC4EIF_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4EIF_W<'a> {
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
#[doc = "Field `IC4IF` reader - "]
pub struct IC4IF_R(crate::FieldReader<bool, bool>);
impl IC4IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC4IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC4IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC4IF` writer - "]
pub struct IC4IF_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4IF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `OC4IF` reader - "]
pub struct OC4IF_R(crate::FieldReader<bool, bool>);
impl OC4IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC4IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC4IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC4IF` writer - "]
pub struct OC4IF_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4IF_W<'a> {
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
#[doc = "Field `INT4IF` reader - "]
pub struct INT4IF_R(crate::FieldReader<bool, bool>);
impl INT4IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT4IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT4IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT4IF` writer - "]
pub struct INT4IF_W<'a> {
    w: &'a mut W,
}
impl<'a> INT4IF_W<'a> {
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
#[doc = "Field `T5IF` reader - "]
pub struct T5IF_R(crate::FieldReader<bool, bool>);
impl T5IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        T5IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T5IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T5IF` writer - "]
pub struct T5IF_W<'a> {
    w: &'a mut W,
}
impl<'a> T5IF_W<'a> {
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
#[doc = "Field `IC5EIF` reader - "]
pub struct IC5EIF_R(crate::FieldReader<bool, bool>);
impl IC5EIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC5EIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC5EIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC5EIF` writer - "]
pub struct IC5EIF_W<'a> {
    w: &'a mut W,
}
impl<'a> IC5EIF_W<'a> {
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
#[doc = "Field `IC5IF` reader - "]
pub struct IC5IF_R(crate::FieldReader<bool, bool>);
impl IC5IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC5IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC5IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC5IF` writer - "]
pub struct IC5IF_W<'a> {
    w: &'a mut W,
}
impl<'a> IC5IF_W<'a> {
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
#[doc = "Field `OC5IF` reader - "]
pub struct OC5IF_R(crate::FieldReader<bool, bool>);
impl OC5IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC5IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC5IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC5IF` writer - "]
pub struct OC5IF_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5IF_W<'a> {
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
#[doc = "Field `AD1IF` reader - "]
pub struct AD1IF_R(crate::FieldReader<bool, bool>);
impl AD1IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD1IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD1IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD1IF` writer - "]
pub struct AD1IF_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1IF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `FSCMIF` reader - "]
pub struct FSCMIF_R(crate::FieldReader<bool, bool>);
impl FSCMIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSCMIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSCMIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSCMIF` writer - "]
pub struct FSCMIF_W<'a> {
    w: &'a mut W,
}
impl<'a> FSCMIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `RTCCIF` reader - "]
pub struct RTCCIF_R(crate::FieldReader<bool, bool>);
impl RTCCIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCCIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCCIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCIF` writer - "]
pub struct RTCCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCIF_W<'a> {
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
#[doc = "Field `FCEIF` reader - "]
pub struct FCEIF_R(crate::FieldReader<bool, bool>);
impl FCEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCEIF` writer - "]
pub struct FCEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> FCEIF_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctif(&self) -> CTIF_R {
        CTIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cs0if(&self) -> CS0IF_R {
        CS0IF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cs1if(&self) -> CS1IF_R {
        CS1IF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn int0if(&self) -> INT0IF_R {
        INT0IF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn t1if(&self) -> T1IF_R {
        T1IF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ic1eif(&self) -> IC1EIF_R {
        IC1EIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ic1if(&self) -> IC1IF_R {
        IC1IF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn oc1if(&self) -> OC1IF_R {
        OC1IF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn int1if(&self) -> INT1IF_R {
        INT1IF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn t2if(&self) -> T2IF_R {
        T2IF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ic2eif(&self) -> IC2EIF_R {
        IC2EIF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ic2if(&self) -> IC2IF_R {
        IC2IF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn oc2if(&self) -> OC2IF_R {
        OC2IF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn int2if(&self) -> INT2IF_R {
        INT2IF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn t3if(&self) -> T3IF_R {
        T3IF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ic3eif(&self) -> IC3EIF_R {
        IC3EIF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ic3if(&self) -> IC3IF_R {
        IC3IF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn oc3if(&self) -> OC3IF_R {
        OC3IF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn int3if(&self) -> INT3IF_R {
        INT3IF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn t4if(&self) -> T4IF_R {
        T4IF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ic4eif(&self) -> IC4EIF_R {
        IC4EIF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ic4if(&self) -> IC4IF_R {
        IC4IF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn oc4if(&self) -> OC4IF_R {
        OC4IF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn int4if(&self) -> INT4IF_R {
        INT4IF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn t5if(&self) -> T5IF_R {
        T5IF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ic5eif(&self) -> IC5EIF_R {
        IC5EIF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ic5if(&self) -> IC5IF_R {
        IC5IF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn oc5if(&self) -> OC5IF_R {
        OC5IF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ad1if(&self) -> AD1IF_R {
        AD1IF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn fscmif(&self) -> FSCMIF_R {
        FSCMIF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtccif(&self) -> RTCCIF_R {
        RTCCIF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn fceif(&self) -> FCEIF_R {
        FCEIF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctif(&mut self) -> CTIF_W {
        CTIF_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cs0if(&mut self) -> CS0IF_W {
        CS0IF_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cs1if(&mut self) -> CS1IF_W {
        CS1IF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn int0if(&mut self) -> INT0IF_W {
        INT0IF_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn t1if(&mut self) -> T1IF_W {
        T1IF_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ic1eif(&mut self) -> IC1EIF_W {
        IC1EIF_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ic1if(&mut self) -> IC1IF_W {
        IC1IF_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn oc1if(&mut self) -> OC1IF_W {
        OC1IF_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn int1if(&mut self) -> INT1IF_W {
        INT1IF_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn t2if(&mut self) -> T2IF_W {
        T2IF_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ic2eif(&mut self) -> IC2EIF_W {
        IC2EIF_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ic2if(&mut self) -> IC2IF_W {
        IC2IF_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn oc2if(&mut self) -> OC2IF_W {
        OC2IF_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn int2if(&mut self) -> INT2IF_W {
        INT2IF_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn t3if(&mut self) -> T3IF_W {
        T3IF_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ic3eif(&mut self) -> IC3EIF_W {
        IC3EIF_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ic3if(&mut self) -> IC3IF_W {
        IC3IF_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn oc3if(&mut self) -> OC3IF_W {
        OC3IF_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn int3if(&mut self) -> INT3IF_W {
        INT3IF_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn t4if(&mut self) -> T4IF_W {
        T4IF_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ic4eif(&mut self) -> IC4EIF_W {
        IC4EIF_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ic4if(&mut self) -> IC4IF_W {
        IC4IF_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn oc4if(&mut self) -> OC4IF_W {
        OC4IF_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn int4if(&mut self) -> INT4IF_W {
        INT4IF_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn t5if(&mut self) -> T5IF_W {
        T5IF_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ic5eif(&mut self) -> IC5EIF_W {
        IC5EIF_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ic5if(&mut self) -> IC5IF_W {
        IC5IF_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn oc5if(&mut self) -> OC5IF_W {
        OC5IF_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ad1if(&mut self) -> AD1IF_W {
        AD1IF_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn fscmif(&mut self) -> FSCMIF_W {
        FSCMIF_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtccif(&mut self) -> RTCCIF_W {
        RTCCIF_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn fceif(&mut self) -> FCEIF_W {
        FCEIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IFS0SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs0set](index.html) module"]
pub struct IFS0SET_SPEC;
impl crate::RegisterSpec for IFS0SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifs0set::R](R) reader structure"]
impl crate::Readable for IFS0SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifs0set::W](W) writer structure"]
impl crate::Writable for IFS0SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFS0SET to value 0"]
impl crate::Resettable for IFS0SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
