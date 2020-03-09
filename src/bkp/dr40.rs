#[doc = "Reader of register DR40"]
pub type R = crate::R<u32, super::DR40>;
#[doc = "Writer for register DR40"]
pub type W = crate::W<u32, super::DR40>;
#[doc = "Register DR40 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR40 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D40`"]
pub type D40_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D40`"]
pub struct D40_W<'a> {
    w: &'a mut W,
}
impl<'a> D40_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d40(&self) -> D40_R {
        D40_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d40(&mut self) -> D40_W {
        D40_W { w: self }
    }
}
