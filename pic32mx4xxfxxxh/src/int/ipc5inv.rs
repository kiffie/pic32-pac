#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IPC5INV {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct T5ISR {
    bits: u8,
}
impl T5ISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct T5IPR {
    bits: u8,
}
impl T5IPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IC5ISR {
    bits: u8,
}
impl IC5ISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IC5IPR {
    bits: u8,
}
impl IC5IPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OC5ISR {
    bits: u8,
}
impl OC5ISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OC5IPR {
    bits: u8,
}
impl OC5IPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AD1ISR {
    bits: u8,
}
impl AD1ISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AD1IPR {
    bits: u8,
}
impl AD1IPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _T5ISW<'a> {
    w: &'a mut W,
}
impl<'a> _T5ISW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _T5IPW<'a> {
    w: &'a mut W,
}
impl<'a> _T5IPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IC5ISW<'a> {
    w: &'a mut W,
}
impl<'a> _IC5ISW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IC5IPW<'a> {
    w: &'a mut W,
}
impl<'a> _IC5IPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OC5ISW<'a> {
    w: &'a mut W,
}
impl<'a> _OC5ISW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OC5IPW<'a> {
    w: &'a mut W,
}
impl<'a> _OC5IPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AD1ISW<'a> {
    w: &'a mut W,
}
impl<'a> _AD1ISW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AD1IPW<'a> {
    w: &'a mut W,
}
impl<'a> _AD1IPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1"]
    #[inline]
    pub fn t5is(&self) -> T5ISR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        T5ISR { bits }
    }
    #[doc = "Bits 2:4"]
    #[inline]
    pub fn t5ip(&self) -> T5IPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        T5IPR { bits }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn ic5is(&self) -> IC5ISR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IC5ISR { bits }
    }
    #[doc = "Bits 10:12"]
    #[inline]
    pub fn ic5ip(&self) -> IC5IPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IC5IPR { bits }
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn oc5is(&self) -> OC5ISR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OC5ISR { bits }
    }
    #[doc = "Bits 18:20"]
    #[inline]
    pub fn oc5ip(&self) -> OC5IPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OC5IPR { bits }
    }
    #[doc = "Bits 24:25"]
    #[inline]
    pub fn ad1is(&self) -> AD1ISR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AD1ISR { bits }
    }
    #[doc = "Bits 26:28"]
    #[inline]
    pub fn ad1ip(&self) -> AD1IPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AD1IPR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1"]
    #[inline]
    pub fn t5is(&mut self) -> _T5ISW {
        _T5ISW { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline]
    pub fn t5ip(&mut self) -> _T5IPW {
        _T5IPW { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn ic5is(&mut self) -> _IC5ISW {
        _IC5ISW { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline]
    pub fn ic5ip(&mut self) -> _IC5IPW {
        _IC5IPW { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn oc5is(&mut self) -> _OC5ISW {
        _OC5ISW { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline]
    pub fn oc5ip(&mut self) -> _OC5IPW {
        _OC5IPW { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline]
    pub fn ad1is(&mut self) -> _AD1ISW {
        _AD1ISW { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline]
    pub fn ad1ip(&mut self) -> _AD1IPW {
        _AD1IPW { w: self }
    }
}
