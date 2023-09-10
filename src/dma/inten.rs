#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `CH0` reader - Channel 0 Interrupt Enable."]
pub type CH0_R = crate::BitReader<CH0_A>;
#[doc = "Channel 0 Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<CH0_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_A {
        match self.bits {
            false => CH0_A::DIS,
            true => CH0_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CH0_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CH0_A::EN
    }
}
#[doc = "Field `CH0` writer - Channel 0 Interrupt Enable."]
pub type CH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH0_A>;
impl<'a, REG, const O: u8> CH0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(CH0_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(CH0_A::EN)
    }
}
#[doc = "Field `CH1` reader - Channel 1 Interrupt Enable."]
pub type CH1_R = crate::BitReader<CH1_A>;
#[doc = "Channel 1 Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<CH1_A> for bool {
    #[inline(always)]
    fn from(variant: CH1_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1_A {
        match self.bits {
            false => CH1_A::DIS,
            true => CH1_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CH1_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CH1_A::EN
    }
}
#[doc = "Field `CH1` writer - Channel 1 Interrupt Enable."]
pub type CH1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH1_A>;
impl<'a, REG, const O: u8> CH1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(CH1_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(CH1_A::EN)
    }
}
#[doc = "Field `CH2` reader - Channel 2 Interrupt Enable."]
pub type CH2_R = crate::BitReader<CH2_A>;
#[doc = "Channel 2 Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<CH2_A> for bool {
    #[inline(always)]
    fn from(variant: CH2_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2_A {
        match self.bits {
            false => CH2_A::DIS,
            true => CH2_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CH2_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CH2_A::EN
    }
}
#[doc = "Field `CH2` writer - Channel 2 Interrupt Enable."]
pub type CH2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH2_A>;
impl<'a, REG, const O: u8> CH2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(CH2_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(CH2_A::EN)
    }
}
#[doc = "Field `CH3` reader - Channel 3 Interrupt Enable."]
pub type CH3_R = crate::BitReader<CH3_A>;
#[doc = "Channel 3 Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<CH3_A> for bool {
    #[inline(always)]
    fn from(variant: CH3_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3_A {
        match self.bits {
            false => CH3_A::DIS,
            true => CH3_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CH3_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CH3_A::EN
    }
}
#[doc = "Field `CH3` writer - Channel 3 Interrupt Enable."]
pub type CH3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CH3_A>;
impl<'a, REG, const O: u8> CH3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(CH3_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(CH3_A::EN)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Interrupt Enable."]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Interrupt Enable."]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Interrupt Enable."]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Interrupt Enable."]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<INTEN_SPEC, 0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<INTEN_SPEC, 1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<INTEN_SPEC, 2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<INTEN_SPEC, 3> {
        CH3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
