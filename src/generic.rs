use core::marker;
#[doc = " Trait implemented by readable registers to enable the `read` method."]
#[doc = ""]
#[doc = " Registers marked with `Writable` can be also `modify`'ed."]
pub trait Readable {}
#[doc = " Trait implemented by writeable registers."]
#[doc = ""]
#[doc = " This enables the  `write`, `write_with_zero` and `reset` methods."]
#[doc = ""]
#[doc = " Registers marked with `Readable` can be also `modify`'ed."]
pub trait Writable {}
#[doc = " Reset value of the register."]
#[doc = ""]
#[doc = " This value is the initial value for the `write` method. It can also be directly written to the"]
#[doc = " register by using the `reset` method."]
pub trait ResetValue {
    #[doc = " Raw register type (`u8`, `u16`, `u32`, ...)."]
    type Type;
    #[doc = " Reset value of the register."]
    fn reset_value() -> Self::Type;
}
#[doc = " This structure provides volatile access to registers."]
pub struct Reg<U, REG> {
    register: vcell::VolatileCell<U>,
    _marker: marker::PhantomData<REG>,
}
unsafe impl<U: Send, REG> Send for Reg<U, REG> {}
impl<U, REG> Reg<U, REG>
where
    Self: Readable,
    U: Copy,
{
    #[doc = " Reads the contents of a `Readable` register."]
    #[doc = ""]
    #[doc = " You can read the raw contents of a register by using `bits`:"]
    #[doc = " ```ignore"]
    #[doc = " let bits = periph.reg.read().bits();"]
    #[doc = " ```"]
    #[doc = " or get the content of a particular field of a register:"]
    #[doc = " ```ignore"]
    #[doc = " let reader = periph.reg.read();"]
    #[doc = " let bits = reader.field1().bits();"]
    #[doc = " let flag = reader.field2().bit_is_set();"]
    #[doc = " ```"]
    #[inline(always)]
    pub fn read(&self) -> R<U, Self> {
        R {
            bits: self.register.get(),
            _reg: marker::PhantomData,
        }
    }
}
impl<U, REG> Reg<U, REG>
where
    Self: ResetValue<Type = U> + Writable,
    U: Copy,
{
    #[doc = " Writes the reset value to `Writable` register."]
    #[doc = ""]
    #[doc = " Resets the register to its initial state."]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
impl<U, REG> Reg<U, REG>
where
    Self: ResetValue<Type = U> + Writable,
    U: Copy,
{
    #[doc = " Writes bits to a `Writable` register."]
    #[doc = ""]
    #[doc = " You can write raw bits into a register:"]
    #[doc = " ```ignore"]
    #[doc = " periph.reg.write(|w| unsafe { w.bits(rawbits) });"]
    #[doc = " ```"]
    #[doc = " or write only the fields you need:"]
    #[doc = " ```ignore"]
    #[doc = " periph.reg.write(|w| w"]
    #[doc = "     .field1().bits(newfield1bits)"]
    #[doc = "     .field2().set_bit()"]
    #[doc = "     .field3().variant(VARIANT)"]
    #[doc = " );"]
    #[doc = " ```"]
    #[doc = " In the latter case, other fields will be set to their reset value."]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W<U, Self>) -> &mut W<U, Self>,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
                _reg: marker::PhantomData,
            })
            .bits,
        );
    }
}
impl<U, REG> Reg<U, REG>
where
    Self: Writable,
    U: Copy + Default,
{
    #[doc = " Writes 0 to a `Writable` register."]
    #[doc = ""]
    #[doc = " Similar to `write`, but unused bits will contain 0."]
    #[inline(always)]
    pub fn write_with_zero<F>(&self, f: F)
    where
        F: FnOnce(&mut W<U, Self>) -> &mut W<U, Self>,
    {
        self.register.set(
            f(&mut W {
                bits: U::default(),
                _reg: marker::PhantomData,
            })
            .bits,
        );
    }
}
impl<U, REG> Reg<U, REG>
where
    Self: Readable + Writable,
    U: Copy,
{
    #[doc = " Modifies the contents of the register by reading and then writing it."]
    #[doc = ""]
    #[doc = " E.g. to do a read-modify-write sequence to change parts of a register:"]
    #[doc = " ```ignore"]
    #[doc = " periph.reg.modify(|r, w| unsafe { w.bits("]
    #[doc = "    r.bits() | 3"]
    #[doc = " ) });"]
    #[doc = " ```"]
    #[doc = " or"]
    #[doc = " ```ignore"]
    #[doc = " periph.reg.modify(|_, w| w"]
    #[doc = "     .field1().bits(newfield1bits)"]
    #[doc = "     .field2().set_bit()"]
    #[doc = "     .field3().variant(VARIANT)"]
    #[doc = " );"]
    #[doc = " ```"]
    #[doc = " Other fields will have the value they had before the call to `modify`."]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R<U, Self>, &'w mut W<U, Self>) -> &'w mut W<U, Self>,
    {
        let bits = self.register.get();
        self.register.set(
            f(
                &R {
                    bits,
                    _reg: marker::PhantomData,
                },
                &mut W {
                    bits,
                    _reg: marker::PhantomData,
                },
            )
            .bits,
        );
    }
}
#[doc = " Register/field reader."]
#[doc = ""]
#[doc = " Result of the `read` methods of registers. Also used as a closure argument in the `modify`"]
#[doc = " method."]
pub struct R<U, T> {
    pub(crate) bits: U,
    _reg: marker::PhantomData<T>,
}
impl<U, T> R<U, T>
where
    U: Copy,
{
    #[doc = " Creates a new instance of the reader."]
    #[inline(always)]
    pub(crate) fn new(bits: U) -> Self {
        Self {
            bits,
            _reg: marker::PhantomData,
        }
    }
    #[doc = " Reads raw bits from register/field."]
    #[inline(always)]
    pub fn bits(&self) -> U {
        self.bits
    }
}
impl<U, T, FI> PartialEq<FI> for R<U, T>
where
    U: PartialEq,
    FI: Copy + Into<U>,
{
    #[inline(always)]
    fn eq(&self, other: &FI) -> bool {
        self.bits.eq(&(*other).into())
    }
}
impl<FI> R<bool, FI> {
    #[doc = " Value of the field as raw bits."]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = " Returns `true` if the bit is clear (0)."]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = " Returns `true` if the bit is set (1)."]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = " Register writer."]
#[doc = ""]
#[doc = " Used as an argument to the closures in the `write` and `modify` methods of the register."]
pub struct W<U, REG> {
    #[doc = "Writable bits"]
    pub(crate) bits: U,
    _reg: marker::PhantomData<REG>,
}
impl<U, REG> W<U, REG> {
    #[doc = " Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: U) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = " Used if enumerated values cover not the whole range."]
#[derive(Clone, Copy, PartialEq)]
pub enum Variant<U, T> {
    #[doc = " Expected variant."]
    Val(T),
    #[doc = " Raw bits."]
    Res(U),
}
