#[doc = "Reader of register DR34"]
pub type R = crate::R<u32, super::DR34>;
#[doc = "Writer for register DR34"]
pub type W = crate::W<u32, super::DR34>;
#[doc = "Register DR34 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR34 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D34`"]
pub type D34_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D34`"]
pub struct D34_W<'a> {
    w: &'a mut W,
}
impl<'a> D34_W<'a> {
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
    pub fn d34(&self) -> D34_R {
        D34_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d34(&mut self) -> D34_W {
        D34_W { w: self }
    }
}
