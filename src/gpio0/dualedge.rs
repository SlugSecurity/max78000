#[doc = "Register `DUALEDGE` reader"]
pub type R = crate::R<DUALEDGE_SPEC>;
#[doc = "Register `DUALEDGE` writer"]
pub type W = crate::W<DUALEDGE_SPEC>;
#[doc = "Field `GPIO_DUALEDGE` reader - Mask of all of the pins on the port."]
pub type GPIO_DUALEDGE_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_DUALEDGE` writer - Mask of all of the pins on the port."]
pub type GPIO_DUALEDGE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_dualedge(&self) -> GPIO_DUALEDGE_R {
        GPIO_DUALEDGE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_dualedge(&mut self) -> GPIO_DUALEDGE_W<DUALEDGE_SPEC> {
        GPIO_DUALEDGE_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO Interrupt Dual Edge Mode Register. Each bit in this register selects dual edge mode for the associated GPIO pin in this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dualedge::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dualedge::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DUALEDGE_SPEC;
impl crate::RegisterSpec for DUALEDGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dualedge::R`](R) reader structure"]
impl crate::Readable for DUALEDGE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dualedge::W`](W) writer structure"]
impl crate::Writable for DUALEDGE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DUALEDGE to value 0"]
impl crate::Resettable for DUALEDGE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
