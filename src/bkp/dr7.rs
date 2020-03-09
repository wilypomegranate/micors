#[doc = "Reader of register DR7"]
pub type R = crate::R<u32, super::DR7>;
#[doc = "Writer for register DR7"]
pub type W = crate::W<u32, super::DR7>;
#[doc = "Register DR7 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D7`"]
pub type D7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D7`"]
pub struct D7_W<'a> {
    w: &'a mut W,
}
impl<'a> D7_W<'a> {
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
    pub fn d7(&self) -> D7_R {
        D7_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d7(&mut self) -> D7_W {
        D7_W { w: self }
    }
}
