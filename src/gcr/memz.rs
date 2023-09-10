#[doc = "Register `MEMZ` reader"]
pub type R = crate::R<MEMZ_SPEC>;
#[doc = "Register `MEMZ` writer"]
pub type W = crate::W<MEMZ_SPEC>;
#[doc = "Field `RAM0` reader - System RAM Block 0 Zeroization."]
pub type RAM0_R = crate::BitReader<RAM0_A>;
#[doc = "System RAM Block 0 Zeroization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM0_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<RAM0_A> for bool {
    #[inline(always)]
    fn from(variant: RAM0_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM0_A {
        match self.bits {
            false => RAM0_A::NOP,
            true => RAM0_A::START,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == RAM0_A::NOP
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == RAM0_A::START
    }
}
#[doc = "Field `RAM0` writer - System RAM Block 0 Zeroization."]
pub type RAM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RAM0_A>;
impl<'a, REG, const O: u8> RAM0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(RAM0_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(RAM0_A::START)
    }
}
#[doc = "Field `RAM1` reader - System RAM Block 1 Zeroization."]
pub type RAM1_R = crate::BitReader<RAM1_A>;
#[doc = "System RAM Block 1 Zeroization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM1_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<RAM1_A> for bool {
    #[inline(always)]
    fn from(variant: RAM1_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM1_A {
        match self.bits {
            false => RAM1_A::NOP,
            true => RAM1_A::START,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == RAM1_A::NOP
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == RAM1_A::START
    }
}
#[doc = "Field `RAM1` writer - System RAM Block 1 Zeroization."]
pub type RAM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RAM1_A>;
impl<'a, REG, const O: u8> RAM1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(RAM1_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(RAM1_A::START)
    }
}
#[doc = "Field `RAM2` reader - System RAM Block 2 Zeroization."]
pub type RAM2_R = crate::BitReader<RAM2_A>;
#[doc = "System RAM Block 2 Zeroization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM2_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<RAM2_A> for bool {
    #[inline(always)]
    fn from(variant: RAM2_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM2_A {
        match self.bits {
            false => RAM2_A::NOP,
            true => RAM2_A::START,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == RAM2_A::NOP
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == RAM2_A::START
    }
}
#[doc = "Field `RAM2` writer - System RAM Block 2 Zeroization."]
pub type RAM2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RAM2_A>;
impl<'a, REG, const O: u8> RAM2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(RAM2_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(RAM2_A::START)
    }
}
#[doc = "Field `RAM3` reader - System RAM Block 3 Zeroization."]
pub type RAM3_R = crate::BitReader<RAM3_A>;
#[doc = "System RAM Block 3 Zeroization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM3_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<RAM3_A> for bool {
    #[inline(always)]
    fn from(variant: RAM3_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM3_A {
        match self.bits {
            false => RAM3_A::NOP,
            true => RAM3_A::START,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == RAM3_A::NOP
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == RAM3_A::START
    }
}
#[doc = "Field `RAM3` writer - System RAM Block 3 Zeroization."]
pub type RAM3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RAM3_A>;
impl<'a, REG, const O: u8> RAM3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(RAM3_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(RAM3_A::START)
    }
}
#[doc = "Field `SYSRAM0ECC` reader - System RAM 0 ECC Zeroization."]
pub type SYSRAM0ECC_R = crate::BitReader<SYSRAM0ECC_A>;
#[doc = "System RAM 0 ECC Zeroization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSRAM0ECC_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<SYSRAM0ECC_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRAM0ECC_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSRAM0ECC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRAM0ECC_A {
        match self.bits {
            false => SYSRAM0ECC_A::NOP,
            true => SYSRAM0ECC_A::START,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == SYSRAM0ECC_A::NOP
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SYSRAM0ECC_A::START
    }
}
#[doc = "Field `SYSRAM0ECC` writer - System RAM 0 ECC Zeroization."]
pub type SYSRAM0ECC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SYSRAM0ECC_A>;
impl<'a, REG, const O: u8> SYSRAM0ECC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRAM0ECC_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRAM0ECC_A::START)
    }
}
#[doc = "Field `ICC0` reader - Instruction Cachei 0 Zeroization."]
pub type ICC0_R = crate::BitReader<ICC0_A>;
#[doc = "Instruction Cachei 0 Zeroization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICC0_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<ICC0_A> for bool {
    #[inline(always)]
    fn from(variant: ICC0_A) -> Self {
        variant as u8 != 0
    }
}
impl ICC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICC0_A {
        match self.bits {
            false => ICC0_A::NOP,
            true => ICC0_A::START,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == ICC0_A::NOP
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == ICC0_A::START
    }
}
#[doc = "Field `ICC0` writer - Instruction Cachei 0 Zeroization."]
pub type ICC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ICC0_A>;
impl<'a, REG, const O: u8> ICC0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(ICC0_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(ICC0_A::START)
    }
}
#[doc = "Field `ICC1` reader - Instruction Cachei 1 Zeroization."]
pub type ICC1_R = crate::BitReader<ICC1_A>;
#[doc = "Instruction Cachei 1 Zeroization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICC1_A {
    #[doc = "0: No operation/complete."]
    NOP = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<ICC1_A> for bool {
    #[inline(always)]
    fn from(variant: ICC1_A) -> Self {
        variant as u8 != 0
    }
}
impl ICC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICC1_A {
        match self.bits {
            false => ICC1_A::NOP,
            true => ICC1_A::START,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == ICC1_A::NOP
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == ICC1_A::START
    }
}
#[doc = "Field `ICC1` writer - Instruction Cachei 1 Zeroization."]
pub type ICC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ICC1_A>;
impl<'a, REG, const O: u8> ICC1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(ICC1_A::NOP)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(ICC1_A::START)
    }
}
impl R {
    #[doc = "Bit 0 - System RAM Block 0 Zeroization."]
    #[inline(always)]
    pub fn ram0(&self) -> RAM0_R {
        RAM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System RAM Block 1 Zeroization."]
    #[inline(always)]
    pub fn ram1(&self) -> RAM1_R {
        RAM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System RAM Block 2 Zeroization."]
    #[inline(always)]
    pub fn ram2(&self) -> RAM2_R {
        RAM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - System RAM Block 3 Zeroization."]
    #[inline(always)]
    pub fn ram3(&self) -> RAM3_R {
        RAM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - System RAM 0 ECC Zeroization."]
    #[inline(always)]
    pub fn sysram0ecc(&self) -> SYSRAM0ECC_R {
        SYSRAM0ECC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Instruction Cachei 0 Zeroization."]
    #[inline(always)]
    pub fn icc0(&self) -> ICC0_R {
        ICC0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Instruction Cachei 1 Zeroization."]
    #[inline(always)]
    pub fn icc1(&self) -> ICC1_R {
        ICC1_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System RAM Block 0 Zeroization."]
    #[inline(always)]
    #[must_use]
    pub fn ram0(&mut self) -> RAM0_W<MEMZ_SPEC, 0> {
        RAM0_W::new(self)
    }
    #[doc = "Bit 1 - System RAM Block 1 Zeroization."]
    #[inline(always)]
    #[must_use]
    pub fn ram1(&mut self) -> RAM1_W<MEMZ_SPEC, 1> {
        RAM1_W::new(self)
    }
    #[doc = "Bit 2 - System RAM Block 2 Zeroization."]
    #[inline(always)]
    #[must_use]
    pub fn ram2(&mut self) -> RAM2_W<MEMZ_SPEC, 2> {
        RAM2_W::new(self)
    }
    #[doc = "Bit 3 - System RAM Block 3 Zeroization."]
    #[inline(always)]
    #[must_use]
    pub fn ram3(&mut self) -> RAM3_W<MEMZ_SPEC, 3> {
        RAM3_W::new(self)
    }
    #[doc = "Bit 4 - System RAM 0 ECC Zeroization."]
    #[inline(always)]
    #[must_use]
    pub fn sysram0ecc(&mut self) -> SYSRAM0ECC_W<MEMZ_SPEC, 4> {
        SYSRAM0ECC_W::new(self)
    }
    #[doc = "Bit 5 - Instruction Cachei 0 Zeroization."]
    #[inline(always)]
    #[must_use]
    pub fn icc0(&mut self) -> ICC0_W<MEMZ_SPEC, 5> {
        ICC0_W::new(self)
    }
    #[doc = "Bit 6 - Instruction Cachei 1 Zeroization."]
    #[inline(always)]
    #[must_use]
    pub fn icc1(&mut self) -> ICC1_W<MEMZ_SPEC, 6> {
        ICC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Memory Zeroize Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEMZ_SPEC;
impl crate::RegisterSpec for MEMZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memz::R`](R) reader structure"]
impl crate::Readable for MEMZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`memz::W`](W) writer structure"]
impl crate::Writable for MEMZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEMZ to value 0"]
impl crate::Resettable for MEMZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
