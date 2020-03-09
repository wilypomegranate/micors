#[doc = "Reader of register DR38"]
pub type R = crate::R<u32, super::DR38>;
#[doc = "Writer for register DR38"]
pub type W = crate::W<u32, super::DR38>;
#[doc = "Register DR38 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR38 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D38`"]
pub type D38_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D38`"]
pub struct D38_W<'a> {
    w: &'a mut W,
}
impl<'a> D38_W<'a> {
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
    pub fn d38(&self) -> D38_R {
        D38_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d38(&mut self) -> D38_W {
        D38_W { w: self }
    }
}
