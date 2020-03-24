#[doc = "Reader of register IEC0SET"]
pub type R = crate::R<u32, super::IEC0SET>;
#[doc = "Writer for register IEC0SET"]
pub type W = crate::W<u32, super::IEC0SET>;
#[doc = "Register IEC0SET `reset()`'s with value 0"]
impl crate::ResetValue for super::IEC0SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTIE`"]
pub type CTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTIE`"]
pub struct CTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIE_W<'a> {
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
#[doc = "Reader of field `CS0IE`"]
pub type CS0IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS0IE`"]
pub struct CS0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS0IE_W<'a> {
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
#[doc = "Reader of field `CS1IE`"]
pub type CS1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS1IE`"]
pub struct CS1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1IE_W<'a> {
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
#[doc = "Reader of field `INT0IE`"]
pub type INT0IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT0IE`"]
pub struct INT0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT0IE_W<'a> {
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
#[doc = "Reader of field `T1IE`"]
pub type T1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T1IE`"]
pub struct T1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> T1IE_W<'a> {
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
#[doc = "Reader of field `IC1EIE`"]
pub type IC1EIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC1EIE`"]
pub struct IC1EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1EIE_W<'a> {
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
#[doc = "Reader of field `IC1IE`"]
pub type IC1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC1IE`"]
pub struct IC1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1IE_W<'a> {
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
#[doc = "Reader of field `OC1IE`"]
pub type OC1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC1IE`"]
pub struct OC1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1IE_W<'a> {
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
#[doc = "Reader of field `INT1IE`"]
pub type INT1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT1IE`"]
pub struct INT1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT1IE_W<'a> {
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
#[doc = "Reader of field `T2IE`"]
pub type T2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T2IE`"]
pub struct T2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> T2IE_W<'a> {
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
#[doc = "Reader of field `IC2EIE`"]
pub type IC2EIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC2EIE`"]
pub struct IC2EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2EIE_W<'a> {
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
#[doc = "Reader of field `IC2IE`"]
pub type IC2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC2IE`"]
pub struct IC2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2IE_W<'a> {
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
#[doc = "Reader of field `OC2IE`"]
pub type OC2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC2IE`"]
pub struct OC2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2IE_W<'a> {
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
#[doc = "Reader of field `INT2IE`"]
pub type INT2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT2IE`"]
pub struct INT2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT2IE_W<'a> {
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
#[doc = "Reader of field `T3IE`"]
pub type T3IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T3IE`"]
pub struct T3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> T3IE_W<'a> {
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
#[doc = "Reader of field `IC3EIE`"]
pub type IC3EIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC3EIE`"]
pub struct IC3EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3EIE_W<'a> {
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
#[doc = "Reader of field `IC3IE`"]
pub type IC3IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC3IE`"]
pub struct IC3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3IE_W<'a> {
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
#[doc = "Reader of field `OC3IE`"]
pub type OC3IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC3IE`"]
pub struct OC3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3IE_W<'a> {
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
#[doc = "Reader of field `INT3IE`"]
pub type INT3IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT3IE`"]
pub struct INT3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT3IE_W<'a> {
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
#[doc = "Reader of field `T4IE`"]
pub type T4IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T4IE`"]
pub struct T4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> T4IE_W<'a> {
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
#[doc = "Reader of field `IC4EIE`"]
pub type IC4EIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC4EIE`"]
pub struct IC4EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4EIE_W<'a> {
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
#[doc = "Reader of field `IC4IE`"]
pub type IC4IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC4IE`"]
pub struct IC4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4IE_W<'a> {
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
#[doc = "Reader of field `OC4IE`"]
pub type OC4IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC4IE`"]
pub struct OC4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4IE_W<'a> {
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
#[doc = "Reader of field `INT4IE`"]
pub type INT4IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT4IE`"]
pub struct INT4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT4IE_W<'a> {
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
#[doc = "Reader of field `T5IE`"]
pub type T5IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T5IE`"]
pub struct T5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> T5IE_W<'a> {
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
#[doc = "Reader of field `IC5EIE`"]
pub type IC5EIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC5EIE`"]
pub struct IC5EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC5EIE_W<'a> {
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
#[doc = "Reader of field `IC5IE`"]
pub type IC5IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC5IE`"]
pub struct IC5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IC5IE_W<'a> {
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
#[doc = "Reader of field `OC5IE`"]
pub type OC5IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC5IE`"]
pub struct OC5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5IE_W<'a> {
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
#[doc = "Reader of field `AD1IE`"]
pub type AD1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AD1IE`"]
pub struct AD1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1IE_W<'a> {
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
#[doc = "Reader of field `FSCMIE`"]
pub type FSCMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSCMIE`"]
pub struct FSCMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSCMIE_W<'a> {
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
#[doc = "Reader of field `RTCCIE`"]
pub type RTCCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCCIE`"]
pub struct RTCCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCIE_W<'a> {
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
#[doc = "Reader of field `FCEIE`"]
pub type FCEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCEIE`"]
pub struct FCEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCEIE_W<'a> {
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
    pub fn ctie(&self) -> CTIE_R {
        CTIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cs0ie(&self) -> CS0IE_R {
        CS0IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cs1ie(&self) -> CS1IE_R {
        CS1IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn int0ie(&self) -> INT0IE_R {
        INT0IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn t1ie(&self) -> T1IE_R {
        T1IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ic1eie(&self) -> IC1EIE_R {
        IC1EIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ic1ie(&self) -> IC1IE_R {
        IC1IE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn oc1ie(&self) -> OC1IE_R {
        OC1IE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn int1ie(&self) -> INT1IE_R {
        INT1IE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn t2ie(&self) -> T2IE_R {
        T2IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ic2eie(&self) -> IC2EIE_R {
        IC2EIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ic2ie(&self) -> IC2IE_R {
        IC2IE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn oc2ie(&self) -> OC2IE_R {
        OC2IE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn int2ie(&self) -> INT2IE_R {
        INT2IE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn t3ie(&self) -> T3IE_R {
        T3IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ic3eie(&self) -> IC3EIE_R {
        IC3EIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ic3ie(&self) -> IC3IE_R {
        IC3IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn oc3ie(&self) -> OC3IE_R {
        OC3IE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn int3ie(&self) -> INT3IE_R {
        INT3IE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn t4ie(&self) -> T4IE_R {
        T4IE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ic4eie(&self) -> IC4EIE_R {
        IC4EIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ic4ie(&self) -> IC4IE_R {
        IC4IE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn oc4ie(&self) -> OC4IE_R {
        OC4IE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn int4ie(&self) -> INT4IE_R {
        INT4IE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn t5ie(&self) -> T5IE_R {
        T5IE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ic5eie(&self) -> IC5EIE_R {
        IC5EIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ic5ie(&self) -> IC5IE_R {
        IC5IE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn oc5ie(&self) -> OC5IE_R {
        OC5IE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ad1ie(&self) -> AD1IE_R {
        AD1IE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn fscmie(&self) -> FSCMIE_R {
        FSCMIE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtccie(&self) -> RTCCIE_R {
        RTCCIE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn fceie(&self) -> FCEIE_R {
        FCEIE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctie(&mut self) -> CTIE_W {
        CTIE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cs0ie(&mut self) -> CS0IE_W {
        CS0IE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cs1ie(&mut self) -> CS1IE_W {
        CS1IE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn int0ie(&mut self) -> INT0IE_W {
        INT0IE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn t1ie(&mut self) -> T1IE_W {
        T1IE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ic1eie(&mut self) -> IC1EIE_W {
        IC1EIE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ic1ie(&mut self) -> IC1IE_W {
        IC1IE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn oc1ie(&mut self) -> OC1IE_W {
        OC1IE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn int1ie(&mut self) -> INT1IE_W {
        INT1IE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn t2ie(&mut self) -> T2IE_W {
        T2IE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ic2eie(&mut self) -> IC2EIE_W {
        IC2EIE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ic2ie(&mut self) -> IC2IE_W {
        IC2IE_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn oc2ie(&mut self) -> OC2IE_W {
        OC2IE_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn int2ie(&mut self) -> INT2IE_W {
        INT2IE_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn t3ie(&mut self) -> T3IE_W {
        T3IE_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ic3eie(&mut self) -> IC3EIE_W {
        IC3EIE_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ic3ie(&mut self) -> IC3IE_W {
        IC3IE_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn oc3ie(&mut self) -> OC3IE_W {
        OC3IE_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn int3ie(&mut self) -> INT3IE_W {
        INT3IE_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn t4ie(&mut self) -> T4IE_W {
        T4IE_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ic4eie(&mut self) -> IC4EIE_W {
        IC4EIE_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ic4ie(&mut self) -> IC4IE_W {
        IC4IE_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn oc4ie(&mut self) -> OC4IE_W {
        OC4IE_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn int4ie(&mut self) -> INT4IE_W {
        INT4IE_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn t5ie(&mut self) -> T5IE_W {
        T5IE_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ic5eie(&mut self) -> IC5EIE_W {
        IC5EIE_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ic5ie(&mut self) -> IC5IE_W {
        IC5IE_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn oc5ie(&mut self) -> OC5IE_W {
        OC5IE_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ad1ie(&mut self) -> AD1IE_W {
        AD1IE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn fscmie(&mut self) -> FSCMIE_W {
        FSCMIE_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtccie(&mut self) -> RTCCIE_W {
        RTCCIE_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn fceie(&mut self) -> FCEIE_W {
        FCEIE_W { w: self }
    }
}
