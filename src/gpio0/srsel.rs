#[doc = "Register `SRSEL` reader"]
pub type R = crate::R<SRSEL_SPEC>;
#[doc = "Register `SRSEL` writer"]
pub type W = crate::W<SRSEL_SPEC>;
#[doc = "Field `GPIO_SRSEL` reader - Mask of all of the pins on the port."]
pub type GPIO_SRSEL_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_SRSEL` writer - Mask of all of the pins on the port."]
pub type GPIO_SRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_srsel(&self) -> GPIO_SRSEL_R {
        GPIO_SRSEL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_srsel(&mut self) -> GPIO_SRSEL_W<SRSEL_SPEC> {
        GPIO_SRSEL_W::new(self, 0)
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
#[doc = "GPIO Slew Rate Enable Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRSEL_SPEC;
impl crate::RegisterSpec for SRSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsel::R`](R) reader structure"]
impl crate::Readable for SRSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srsel::W`](W) writer structure"]
impl crate::Writable for SRSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSEL to value 0"]
impl crate::Resettable for SRSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
