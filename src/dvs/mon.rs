#[doc = "Register `MON` reader"]
pub type R = crate::R<MON_SPEC>;
#[doc = "Register `MON` writer"]
pub type W = crate::W<MON_SPEC>;
#[doc = "Field `DLY` reader - Number of prescaled clocks between delay line samples"]
pub type DLY_R = crate::FieldReader<u32>;
#[doc = "Field `DLY` writer - Number of prescaled clocks between delay line samples"]
pub type DLY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
#[doc = "Field `PRE` reader - Number of clocks before DVS_MON_DLY is decremented"]
pub type PRE_R = crate::FieldReader;
#[doc = "Field `PRE` writer - Number of clocks before DVS_MON_DLY is decremented"]
pub type PRE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:23 - Number of prescaled clocks between delay line samples"]
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - Number of clocks before DVS_MON_DLY is decremented"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Number of prescaled clocks between delay line samples"]
    #[inline(always)]
    #[must_use]
    pub fn dly(&mut self) -> DLY_W<MON_SPEC, 0> {
        DLY_W::new(self)
    }
    #[doc = "Bits 24:31 - Number of clocks before DVS_MON_DLY is decremented"]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PRE_W<MON_SPEC, 24> {
        PRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Monitor Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MON_SPEC;
impl crate::RegisterSpec for MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mon::R`](R) reader structure"]
impl crate::Readable for MON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mon::W`](W) writer structure"]
impl crate::Writable for MON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MON to value 0"]
impl crate::Resettable for MON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
