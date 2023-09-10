#[doc = "Register `FIFO_SIZE` reader"]
pub type R = crate::R<FIFO_SIZE_SPEC>;
#[doc = "Register `FIFO_SIZE` writer"]
pub type W = crate::W<FIFO_SIZE_SPEC>;
#[doc = "Field `fifo_size` reader - FIFO size."]
pub type FIFO_SIZE_R = crate::FieldReader;
#[doc = "Field `fifo_size` writer - FIFO size."]
pub type FIFO_SIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - FIFO size."]
    #[inline(always)]
    pub fn fifo_size(&self) -> FIFO_SIZE_R {
        FIFO_SIZE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FIFO size."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_size(&mut self) -> FIFO_SIZE_W<FIFO_SIZE_SPEC, 0> {
        FIFO_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FIFO Depth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_SIZE_SPEC;
impl crate::RegisterSpec for FIFO_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_size::R`](R) reader structure"]
impl crate::Readable for FIFO_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo_size::W`](W) writer structure"]
impl crate::Writable for FIFO_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO_SIZE to value 0"]
impl crate::Resettable for FIFO_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
