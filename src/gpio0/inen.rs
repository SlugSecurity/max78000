#[doc = "Register `INEN` reader"]
pub type R = crate::R<INEN_SPEC>;
#[doc = "Register `INEN` writer"]
pub type W = crate::W<INEN_SPEC>;
#[doc = "Field `GPIO_INEN` reader - Mask of all of the pins on the port."]
pub type GPIO_INEN_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_INEN` writer - Mask of all of the pins on the port."]
pub type GPIO_INEN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_inen(&self) -> GPIO_INEN_R {
        GPIO_INEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_inen(&mut self) -> GPIO_INEN_W<INEN_SPEC> {
        GPIO_INEN_W::new(self, 0)
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
#[doc = "GPIO Input Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INEN_SPEC;
impl crate::RegisterSpec for INEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inen::R`](R) reader structure"]
impl crate::Readable for INEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inen::W`](W) writer structure"]
impl crate::Writable for INEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INEN to value 0"]
impl crate::Resettable for INEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
