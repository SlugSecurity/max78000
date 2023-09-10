#[doc = "Register `SRC` reader"]
pub type R = crate::R<SRC_SPEC>;
#[doc = "Register `SRC` writer"]
pub type W = crate::W<SRC_SPEC>;
#[doc = "Field `ADDR` reader - "]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - "]
pub type ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<SRC_SPEC, 0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Source Device Address. If SRCINC=1, the counter bits are incremented by 1,2, or 4, depending on the data width of each AHB cycle. For peripheral transfers, some or all of the actual address bits are fixed. If SRCINC=0, this register remains constant. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with the contents of DMA_SRC_RLD.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRC_SPEC;
impl crate::RegisterSpec for SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src::R`](R) reader structure"]
impl crate::Readable for SRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`src::W`](W) writer structure"]
impl crate::Writable for SRC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRC to value 0"]
impl crate::Resettable for SRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
