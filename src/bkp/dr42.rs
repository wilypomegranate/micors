#[doc = "Reader of register DR42"]
pub type R = crate::R<u32, super::DR42>;
#[doc = "Writer for register DR42"]
pub type W = crate::W<u32, super::DR42>;
#[doc = "Register DR42 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR42 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D42`"]
pub type D42_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D42`"]
pub struct D42_W<'a> {
    w: &'a mut W,
}
impl<'a> D42_W<'a> {
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
    pub fn d42(&self) -> D42_R {
        D42_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d42(&mut self) -> D42_W {
        D42_W { w: self }
    }
}
