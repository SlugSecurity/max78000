#[doc = "Register `ECCIE` reader"]
pub type R = crate::R<ECCIE_SPEC>;
#[doc = "Register `ECCIE` writer"]
pub type W = crate::W<ECCIE_SPEC>;
#[doc = "Field `RAM` reader - ECC System RAM0 Error Interrup Enable"]
pub type RAM_R = crate::BitReader;
#[doc = "Field `RAM` writer - ECC System RAM0 Error Interrup Enable"]
pub type RAM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - ECC System RAM0 Error Interrup Enable"]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC System RAM0 Error Interrup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ram(&mut self) -> RAM_W<ECCIE_SPEC, 0> {
        RAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ECC IRQ Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCIE_SPEC;
impl crate::RegisterSpec for ECCIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccie::R`](R) reader structure"]
impl crate::Readable for ECCIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eccie::W`](W) writer structure"]
impl crate::Writable for ECCIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECCIE to value 0"]
impl crate::Resettable for ECCIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
