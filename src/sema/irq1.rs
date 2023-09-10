#[doc = "Register `irq1` reader"]
pub type R = crate::R<IRQ1_SPEC>;
#[doc = "Register `irq1` writer"]
pub type W = crate::W<IRQ1_SPEC>;
#[doc = "Field `en` reader - "]
pub type EN_R = crate::BitReader;
#[doc = "Field `en` writer - "]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `rv32_irq` reader - "]
pub type RV32_IRQ_R = crate::BitReader;
#[doc = "Field `rv32_irq` writer - "]
pub type RV32_IRQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rv32_irq(&self) -> RV32_IRQ_R {
        RV32_IRQ_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<IRQ1_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn rv32_irq(&mut self) -> RV32_IRQ_W<IRQ1_SPEC, 16> {
        RV32_IRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Semaphore IRQ1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ1_SPEC;
impl crate::RegisterSpec for IRQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq1::R`](R) reader structure"]
impl crate::Readable for IRQ1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq1::W`](W) writer structure"]
impl crate::Writable for IRQ1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets irq1 to value 0"]
impl crate::Resettable for IRQ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
