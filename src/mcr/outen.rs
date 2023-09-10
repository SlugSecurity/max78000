#[doc = "Register `OUTEN` reader"]
pub type R = crate::R<OUTEN_SPEC>;
#[doc = "Register `OUTEN` writer"]
pub type W = crate::W<OUTEN_SPEC>;
#[doc = "Field `SQWOUT_EN` reader - Square Wave Output Enable."]
pub type SQWOUT_EN_R = crate::BitReader;
#[doc = "Field `SQWOUT_EN` writer - Square Wave Output Enable."]
pub type SQWOUT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDOWN_OUT_EN` reader - Power Down Output Enable."]
pub type PDOWN_OUT_EN_R = crate::BitReader;
#[doc = "Field `PDOWN_OUT_EN` writer - Power Down Output Enable."]
pub type PDOWN_OUT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Square Wave Output Enable."]
    #[inline(always)]
    pub fn sqwout_en(&self) -> SQWOUT_EN_R {
        SQWOUT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Down Output Enable."]
    #[inline(always)]
    pub fn pdown_out_en(&self) -> PDOWN_OUT_EN_R {
        PDOWN_OUT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Square Wave Output Enable."]
    #[inline(always)]
    #[must_use]
    pub fn sqwout_en(&mut self) -> SQWOUT_EN_W<OUTEN_SPEC, 0> {
        SQWOUT_EN_W::new(self)
    }
    #[doc = "Bit 1 - Power Down Output Enable."]
    #[inline(always)]
    #[must_use]
    pub fn pdown_out_en(&mut self) -> PDOWN_OUT_EN_W<OUTEN_SPEC, 1> {
        PDOWN_OUT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Output Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTEN_SPEC;
impl crate::RegisterSpec for OUTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outen::R`](R) reader structure"]
impl crate::Readable for OUTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`outen::W`](W) writer structure"]
impl crate::Writable for OUTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTEN to value 0"]
impl crate::Resettable for OUTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
