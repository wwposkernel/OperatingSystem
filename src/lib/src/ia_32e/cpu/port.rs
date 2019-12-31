#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use crate::ia_32e::instructions::port::{inb, outb, inw, outw, inl, outl};
use core::marker::PhantomData;

pub trait PortRead {
    unsafe fn read(port: u16) -> Self;
}

pub trait PortWrite {
    unsafe fn write(port: u16, value: Self);
}

//pub trait PortReadWrite: PortRead + PortWrite {}

// ---------------------- u8 ---------------------

pub trait PortReadWrite: PortRead + PortWrite {}

impl PortRead for u8 {
    unsafe fn read(port: u16) -> Self {
        inb(port)
    }
}

impl PortWrite for u8 {
    unsafe fn write(port: u16, value: Self) {
        outb(value, port);
    }
}

impl PortReadWrite for u8 {}

// ---------------------- u16 ---------------------
impl PortWrite for u16 {
    unsafe fn write(port: u16, value: Self) {
        outw(value, port);
    }
}

impl PortRead for u16 {
    unsafe fn read(port: u16) -> Self {
        inw(port)
    }
}

impl PortReadWrite for u16 {}

// ---------------------- u32 ---------------------


impl PortRead for u32 {
    unsafe fn read(port: u16) -> Self {
        inl(port)
    }
}

impl PortWrite for u32 {
    unsafe fn write(port: u16, value: Self) {
        outl(value, port);
    }
}

impl PortReadWrite for u32 {}


#[derive(Debug)]
pub struct Port<T> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T: PortReadWrite> Port<T> {
    pub const unsafe fn new(port: u16) -> Port<T> {
        Port { port, phantom: PhantomData {} }
    }

    pub fn read(&mut self) -> T {
        unsafe {
            T::read(self.port)
        }
    }

    pub fn write(&mut self, value: T) {
        unsafe {
            T::write(self.port, value);
        }
    }
}

#[derive(Debug)]
pub struct UnsafePort<T> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T: PortReadWrite> UnsafePort<T> {
    pub const unsafe fn new(port: u16) -> UnsafePort<T> {
        UnsafePort { port, phantom: PhantomData }
    }

    pub unsafe fn write(&mut self, value: T) {
        T::write(self.port, value);
    }

    pub unsafe fn read(&mut self) -> T {
        T::read(self.port)
    }
}