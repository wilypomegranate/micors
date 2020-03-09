#[doc = "Reader of register DR26"]
pub type R = crate::R<u32, super::DR26>;
#[doc = "Writer for register DR26"]
pub type W = crate::W<u32, super::DR26>;
#[doc = "Register DR26 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR26 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D26`"]
pub type D26_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D26`"]
pub struct D26_W<'a> {
    w: &'a mut W,
}
impl<'a> D26_W<'a> {
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
    pub fn d26(&self) -> D26_R {
        D26_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d26(&mut self) -> D26_W {
        D26_W { w: self }
    }
}
