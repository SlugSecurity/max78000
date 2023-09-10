#[doc = "Register `INT_FL` reader"]
pub type R = crate::R<INT_FL_SPEC>;
#[doc = "Register `INT_FL` writer"]
pub type W = crate::W<INT_FL_SPEC>;
#[doc = "Field `IMG_DONE` reader - Image Done."]
pub type IMG_DONE_R = crate::BitReader;
#[doc = "Field `IMG_DONE` writer - Image Done."]
pub type IMG_DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFO_FULL` reader - FIFO Full."]
pub type FIFO_FULL_R = crate::BitReader;
#[doc = "Field `FIFO_FULL` writer - FIFO Full."]
pub type FIFO_FULL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFO_THRESH` reader - FIFO Threshold Level Met."]
pub type FIFO_THRESH_R = crate::BitReader;
#[doc = "Field `FIFO_THRESH` writer - FIFO Threshold Level Met."]
pub type FIFO_THRESH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFO_NOT_EMPTY` reader - FIFO Not Empty."]
pub type FIFO_NOT_EMPTY_R = crate::BitReader;
#[doc = "Field `FIFO_NOT_EMPTY` writer - FIFO Not Empty."]
pub type FIFO_NOT_EMPTY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Image Done."]
    #[inline(always)]
    pub fn img_done(&self) -> IMG_DONE_R {
        IMG_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Full."]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO Threshold Level Met."]
    #[inline(always)]
    pub fn fifo_thresh(&self) -> FIFO_THRESH_R {
        FIFO_THRESH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO Not Empty."]
    #[inline(always)]
    pub fn fifo_not_empty(&self) -> FIFO_NOT_EMPTY_R {
        FIFO_NOT_EMPTY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Image Done."]
    #[inline(always)]
    #[must_use]
    pub fn img_done(&mut self) -> IMG_DONE_W<INT_FL_SPEC, 0> {
        IMG_DONE_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Full."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_full(&mut self) -> FIFO_FULL_W<INT_FL_SPEC, 1> {
        FIFO_FULL_W::new(self)
    }
    #[doc = "Bit 2 - FIFO Threshold Level Met."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_thresh(&mut self) -> FIFO_THRESH_W<INT_FL_SPEC, 2> {
        FIFO_THRESH_W::new(self)
    }
    #[doc = "Bit 3 - FIFO Not Empty."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_not_empty(&mut self) -> FIFO_NOT_EMPTY_W<INT_FL_SPEC, 3> {
        FIFO_NOT_EMPTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interupt Flag Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_fl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_fl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_FL_SPEC;
impl crate::RegisterSpec for INT_FL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_fl::R`](R) reader structure"]
impl crate::Readable for INT_FL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_fl::W`](W) writer structure"]
impl crate::Writable for INT_FL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_FL to value 0"]
impl crate::Resettable for INT_FL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
