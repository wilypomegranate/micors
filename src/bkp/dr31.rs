#[doc = "Reader of register DR31"]
pub type R = crate::R<u32, super::DR31>;
#[doc = "Writer for register DR31"]
pub type W = crate::W<u32, super::DR31>;
#[doc = "Register DR31 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR31 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D31`"]
pub type D31_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D31`"]
pub struct D31_W<'a> {
    w: &'a mut W,
}
impl<'a> D31_W<'a> {
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
    pub fn d31(&self) -> D31_R {
        D31_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d31(&mut self) -> D31_W {
        D31_W { w: self }
    }
}
