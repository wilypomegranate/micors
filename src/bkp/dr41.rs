#[doc = "Reader of register DR41"]
pub type R = crate::R<u32, super::DR41>;
#[doc = "Writer for register DR41"]
pub type W = crate::W<u32, super::DR41>;
#[doc = "Register DR41 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR41 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D41`"]
pub type D41_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D41`"]
pub struct D41_W<'a> {
    w: &'a mut W,
}
impl<'a> D41_W<'a> {
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
    pub fn d41(&self) -> D41_R {
        D41_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d41(&mut self) -> D41_W {
        D41_W { w: self }
    }
}
