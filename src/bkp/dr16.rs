#[doc = "Reader of register DR16"]
pub type R = crate::R<u32, super::DR16>;
#[doc = "Writer for register DR16"]
pub type W = crate::W<u32, super::DR16>;
#[doc = "Register DR16 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR16 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D16`"]
pub type D16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D16`"]
pub struct D16_W<'a> {
    w: &'a mut W,
}
impl<'a> D16_W<'a> {
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
    pub fn d16(&self) -> D16_R {
        D16_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d16(&mut self) -> D16_W {
        D16_W { w: self }
    }
}
