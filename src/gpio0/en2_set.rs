#[doc = "Register `EN2_SET` reader"]
pub type R = crate::R<EN2_SET_SPEC>;
#[doc = "Register `EN2_SET` writer"]
pub type W = crate::W<EN2_SET_SPEC>;
#[doc = "Field `ALL` reader - Mask of all of the pins on the port."]
pub type ALL_R = crate::FieldReader<u32>;
#[doc = "Field `ALL` writer - Mask of all of the pins on the port."]
pub type ALL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn all(&self) -> ALL_R {
        ALL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    #[must_use]
    pub fn all(&mut self) -> ALL_W<EN2_SET_SPEC> {
        ALL_W::new(self, 0)
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
#[doc = "GPIO Alternate Function 2 Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN2 to 1, without affecting other bits in that register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en2_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en2_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EN2_SET_SPEC;
impl crate::RegisterSpec for EN2_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en2_set::R`](R) reader structure"]
impl crate::Readable for EN2_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`en2_set::W`](W) writer structure"]
impl crate::Writable for EN2_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EN2_SET to value 0"]
impl crate::Resettable for EN2_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
