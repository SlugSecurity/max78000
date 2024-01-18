#[doc = "Register `INTEN_SET` reader"]
pub type R = crate::R<INTEN_SET_SPEC>;
#[doc = "Register `INTEN_SET` writer"]
pub type W = crate::W<INTEN_SET_SPEC>;
#[doc = "Field `GPIO_INTEN_SET` reader - Mask of all of the pins on the port."]
pub type GPIO_INTEN_SET_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_INTEN_SET` writer - Mask of all of the pins on the port."]
pub type GPIO_INTEN_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_inten_set(&self) -> GPIO_INTEN_SET_R {
        GPIO_INTEN_SET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_inten_set(&mut self) -> GPIO_INTEN_SET_W<INTEN_SET_SPEC> {
        GPIO_INTEN_SET_W::new(self, 0)
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
#[doc = "GPIO Interrupt Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_INT_EN to 1, without affecting other bits in that register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SET_SPEC;
impl crate::RegisterSpec for INTEN_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten_set::R`](R) reader structure"]
impl crate::Readable for INTEN_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten_set::W`](W) writer structure"]
impl crate::Writable for INTEN_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN_SET to value 0"]
impl crate::Resettable for INTEN_SET_SPEC {
    const RESET_VALUE: u32 = 0;
}
