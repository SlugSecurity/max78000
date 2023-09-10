#[doc = "Register `CMP_CTRL` reader"]
pub type R = crate::R<CMP_CTRL_SPEC>;
#[doc = "Register `CMP_CTRL` writer"]
pub type W = crate::W<CMP_CTRL_SPEC>;
#[doc = "Field `EN` reader - Comparator Enable."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Comparator Enable."]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POL` reader - Polarity Select"]
pub type POL_R = crate::BitReader;
#[doc = "Field `POL` writer - Polarity Select"]
pub type POL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INT_EN` reader - IRQ Enable."]
pub type INT_EN_R = crate::BitReader;
#[doc = "Field `INT_EN` writer - IRQ Enable."]
pub type INT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUT` reader - Comparator Output State."]
pub type OUT_R = crate::BitReader;
#[doc = "Field `OUT` writer - Comparator Output State."]
pub type OUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INT_FL` reader - IRQ Flag"]
pub type INT_FL_R = crate::BitReader;
#[doc = "Field `INT_FL` writer - IRQ Flag"]
pub type INT_FL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Comparator Enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Polarity Select"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRQ Enable."]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - Comparator Output State."]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IRQ Flag"]
    #[inline(always)]
    pub fn int_fl(&self) -> INT_FL_R {
        INT_FL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator Enable."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CMP_CTRL_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 5 - Polarity Select"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<CMP_CTRL_SPEC, 5> {
        POL_W::new(self)
    }
    #[doc = "Bit 6 - IRQ Enable."]
    #[inline(always)]
    #[must_use]
    pub fn int_en(&mut self) -> INT_EN_W<CMP_CTRL_SPEC, 6> {
        INT_EN_W::new(self)
    }
    #[doc = "Bit 14 - Comparator Output State."]
    #[inline(always)]
    #[must_use]
    pub fn out(&mut self) -> OUT_W<CMP_CTRL_SPEC, 14> {
        OUT_W::new(self)
    }
    #[doc = "Bit 15 - IRQ Flag"]
    #[inline(always)]
    #[must_use]
    pub fn int_fl(&mut self) -> INT_FL_W<CMP_CTRL_SPEC, 15> {
        INT_FL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Comparator Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP_CTRL_SPEC;
impl crate::RegisterSpec for CMP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp_ctrl::R`](R) reader structure"]
impl crate::Readable for CMP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp_ctrl::W`](W) writer structure"]
impl crate::Writable for CMP_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMP_CTRL to value 0"]
impl crate::Resettable for CMP_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
