#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IPC9 {
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
pub struct U2ISR {
    bits: u8,
}
impl U2ISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct U2IPR {
    bits: u8,
}
impl U2IPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct I2C2ISR {
    bits: u8,
}
impl I2C2ISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct I2C2IPR {
    bits: u8,
}
impl I2C2IPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct U3ISR {
    bits: u8,
}
impl U3ISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct U3IPR {
    bits: u8,
}
impl U3IPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct U4ISR {
    bits: u8,
}
impl U4ISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct U4IPR {
    bits: u8,
}
impl U4IPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _U2ISW<'a> {
    w: &'a mut W,
}
impl<'a> _U2ISW<'a> {
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
pub struct _U2IPW<'a> {
    w: &'a mut W,
}
impl<'a> _U2IPW<'a> {
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
pub struct _I2C2ISW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C2ISW<'a> {
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
pub struct _I2C2IPW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C2IPW<'a> {
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
pub struct _U3ISW<'a> {
    w: &'a mut W,
}
impl<'a> _U3ISW<'a> {
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
pub struct _U3IPW<'a> {
    w: &'a mut W,
}
impl<'a> _U3IPW<'a> {
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
pub struct _U4ISW<'a> {
    w: &'a mut W,
}
impl<'a> _U4ISW<'a> {
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
pub struct _U4IPW<'a> {
    w: &'a mut W,
}
impl<'a> _U4IPW<'a> {
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
    pub fn u2is(&self) -> U2ISR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        U2ISR { bits }
    }
    #[doc = "Bits 2:4"]
    #[inline]
    pub fn u2ip(&self) -> U2IPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        U2IPR { bits }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn i2c2is(&self) -> I2C2ISR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        I2C2ISR { bits }
    }
    #[doc = "Bits 10:12"]
    #[inline]
    pub fn i2c2ip(&self) -> I2C2IPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        I2C2IPR { bits }
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn u3is(&self) -> U3ISR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        U3ISR { bits }
    }
    #[doc = "Bits 18:20"]
    #[inline]
    pub fn u3ip(&self) -> U3IPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        U3IPR { bits }
    }
    #[doc = "Bits 24:25"]
    #[inline]
    pub fn u4is(&self) -> U4ISR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        U4ISR { bits }
    }
    #[doc = "Bits 26:28"]
    #[inline]
    pub fn u4ip(&self) -> U4IPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        U4IPR { bits }
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
    pub fn u2is(&mut self) -> _U2ISW {
        _U2ISW { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline]
    pub fn u2ip(&mut self) -> _U2IPW {
        _U2IPW { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline]
    pub fn i2c2is(&mut self) -> _I2C2ISW {
        _I2C2ISW { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline]
    pub fn i2c2ip(&mut self) -> _I2C2IPW {
        _I2C2IPW { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline]
    pub fn u3is(&mut self) -> _U3ISW {
        _U3ISW { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline]
    pub fn u3ip(&mut self) -> _U3IPW {
        _U3IPW { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline]
    pub fn u4is(&mut self) -> _U4ISW {
        _U4ISW { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline]
    pub fn u4ip(&mut self) -> _U4IPW {
        _U4IPW { w: self }
    }
}
