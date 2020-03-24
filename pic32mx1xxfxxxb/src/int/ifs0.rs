#[doc = "Reader of register IFS0"]
pub type R = crate::R<u32, super::IFS0>;
#[doc = "Writer for register IFS0"]
pub type W = crate::W<u32, super::IFS0>;
#[doc = "Register IFS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IFS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTIF`"]
pub type CTIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTIF`"]
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
#[doc = "Reader of field `CS0IF`"]
pub type CS0IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS0IF`"]
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
#[doc = "Reader of field `CS1IF`"]
pub type CS1IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS1IF`"]
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
#[doc = "Reader of field `INT0IF`"]
pub type INT0IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT0IF`"]
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
#[doc = "Reader of field `T1IF`"]
pub type T1IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T1IF`"]
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
#[doc = "Reader of field `IC1EIF`"]
pub type IC1EIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC1EIF`"]
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
#[doc = "Reader of field `IC1IF`"]
pub type IC1IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC1IF`"]
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
#[doc = "Reader of field `OC1IF`"]
pub type OC1IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC1IF`"]
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
#[doc = "Reader of field `INT1IF`"]
pub type INT1IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT1IF`"]
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
#[doc = "Reader of field `T2IF`"]
pub type T2IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T2IF`"]
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
#[doc = "Reader of field `IC2EIF`"]
pub type IC2EIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC2EIF`"]
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
#[doc = "Reader of field `IC2IF`"]
pub type IC2IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC2IF`"]
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
#[doc = "Reader of field `OC2IF`"]
pub type OC2IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC2IF`"]
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
#[doc = "Reader of field `INT2IF`"]
pub type INT2IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT2IF`"]
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
#[doc = "Reader of field `T3IF`"]
pub type T3IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T3IF`"]
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
#[doc = "Reader of field `IC3EIF`"]
pub type IC3EIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC3EIF`"]
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
#[doc = "Reader of field `IC3IF`"]
pub type IC3IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC3IF`"]
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
#[doc = "Reader of field `OC3IF`"]
pub type OC3IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC3IF`"]
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
#[doc = "Reader of field `INT3IF`"]
pub type INT3IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT3IF`"]
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
#[doc = "Reader of field `T4IF`"]
pub type T4IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T4IF`"]
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
#[doc = "Reader of field `IC4EIF`"]
pub type IC4EIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC4EIF`"]
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
#[doc = "Reader of field `IC4IF`"]
pub type IC4IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC4IF`"]
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
#[doc = "Reader of field `OC4IF`"]
pub type OC4IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC4IF`"]
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
#[doc = "Reader of field `INT4IF`"]
pub type INT4IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT4IF`"]
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
#[doc = "Reader of field `T5IF`"]
pub type T5IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T5IF`"]
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
#[doc = "Reader of field `IC5EIF`"]
pub type IC5EIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC5EIF`"]
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
#[doc = "Reader of field `IC5IF`"]
pub type IC5IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IC5IF`"]
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
#[doc = "Reader of field `OC5IF`"]
pub type OC5IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OC5IF`"]
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
#[doc = "Reader of field `AD1IF`"]
pub type AD1IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AD1IF`"]
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
#[doc = "Reader of field `FSCMIF`"]
pub type FSCMIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSCMIF`"]
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
#[doc = "Reader of field `RTCCIF`"]
pub type RTCCIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCCIF`"]
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
#[doc = "Reader of field `FCEIF`"]
pub type FCEIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCEIF`"]
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
}
