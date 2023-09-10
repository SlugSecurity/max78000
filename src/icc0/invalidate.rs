#[doc = "Register `INVALIDATE` reader"]
pub type R = crate::R<INVALIDATE_SPEC>;
#[doc = "Register `INVALIDATE` writer"]
pub type W = crate::W<INVALIDATE_SPEC>;
#[doc = "Field `INVALID` reader - Invalidate."]
pub type INVALID_R = crate::FieldReader<u32>;
#[doc = "Field `INVALID` writer - Invalidate."]
pub type INVALID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Invalidate."]
    #[inline(always)]
    pub fn invalid(&self) -> INVALID_R {
        INVALID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Invalidate."]
    #[inline(always)]
    #[must_use]
    pub fn invalid(&mut self) -> INVALID_W<INVALIDATE_SPEC, 0> {
        INVALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Invalidate All Registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`invalidate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`invalidate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INVALIDATE_SPEC;
impl crate::RegisterSpec for INVALIDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`invalidate::R`](R) reader structure"]
impl crate::Readable for INVALIDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`invalidate::W`](W) writer structure"]
impl crate::Writable for INVALIDATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INVALIDATE to value 0"]
impl crate::Resettable for INVALIDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
