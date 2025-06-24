#[doc = "Register `COMP4_CSR` reader"]
pub type R = crate::R<Comp4CsrSpec>;
#[doc = "Register `COMP4_CSR` writer"]
pub type W = crate::W<Comp4CsrSpec>;
#[doc = "Field `COMP4EN` reader - Comparator 4 enable"]
pub type Comp4enR = crate::BitReader;
#[doc = "Field `COMP4EN` writer - Comparator 4 enable"]
pub type Comp4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP4MODE` reader - Comparator 4 mode"]
pub type Comp4modeR = crate::FieldReader;
#[doc = "Field `COMP4MODE` writer - Comparator 4 mode"]
pub type Comp4modeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP4INSEL` reader - Comparator 4 inverting input selection"]
pub type Comp4inselR = crate::FieldReader;
#[doc = "Field `COMP4INSEL` writer - Comparator 4 inverting input selection"]
pub type Comp4inselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP4INPSEL` reader - Comparator 4 non inverted input selection"]
pub type Comp4inpselR = crate::BitReader;
#[doc = "Field `COMP4INPSEL` writer - Comparator 4 non inverted input selection"]
pub type Comp4inpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COM4WINMODE` reader - Comparator 4 window mode"]
pub type Com4winmodeR = crate::BitReader;
#[doc = "Field `COM4WINMODE` writer - Comparator 4 window mode"]
pub type Com4winmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP4_OUT_SEL` reader - Comparator 4 output selection"]
pub type Comp4OutSelR = crate::FieldReader;
#[doc = "Field `COMP4_OUT_SEL` writer - Comparator 4 output selection"]
pub type Comp4OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COMP4POL` reader - Comparator 4 output polarity"]
pub type Comp4polR = crate::BitReader;
#[doc = "Field `COMP4POL` writer - Comparator 4 output polarity"]
pub type Comp4polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP4HYST` reader - Comparator 4 hysteresis"]
pub type Comp4hystR = crate::FieldReader;
#[doc = "Field `COMP4HYST` writer - Comparator 4 hysteresis"]
pub type Comp4hystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP4_BLANKING` reader - Comparator 4 blanking source"]
pub type Comp4BlankingR = crate::FieldReader;
#[doc = "Field `COMP4_BLANKING` writer - Comparator 4 blanking source"]
pub type Comp4BlankingW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP4OUT` reader - Comparator 4 output"]
pub type Comp4outR = crate::BitReader;
#[doc = "Field `COMP4LOCK` reader - Comparator 4 lock"]
pub type Comp4lockR = crate::BitReader;
#[doc = "Field `COMP4LOCK` writer - Comparator 4 lock"]
pub type Comp4lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 4 enable"]
    #[inline(always)]
    pub fn comp4en(&self) -> Comp4enR {
        Comp4enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 4 mode"]
    #[inline(always)]
    pub fn comp4mode(&self) -> Comp4modeR {
        Comp4modeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 4 inverting input selection"]
    #[inline(always)]
    pub fn comp4insel(&self) -> Comp4inselR {
        Comp4inselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator 4 non inverted input selection"]
    #[inline(always)]
    pub fn comp4inpsel(&self) -> Comp4inpselR {
        Comp4inpselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator 4 window mode"]
    #[inline(always)]
    pub fn com4winmode(&self) -> Com4winmodeR {
        Com4winmodeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 4 output selection"]
    #[inline(always)]
    pub fn comp4_out_sel(&self) -> Comp4OutSelR {
        Comp4OutSelR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 4 output polarity"]
    #[inline(always)]
    pub fn comp4pol(&self) -> Comp4polR {
        Comp4polR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 4 hysteresis"]
    #[inline(always)]
    pub fn comp4hyst(&self) -> Comp4hystR {
        Comp4hystR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 4 blanking source"]
    #[inline(always)]
    pub fn comp4_blanking(&self) -> Comp4BlankingR {
        Comp4BlankingR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator 4 output"]
    #[inline(always)]
    pub fn comp4out(&self) -> Comp4outR {
        Comp4outR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 4 lock"]
    #[inline(always)]
    pub fn comp4lock(&self) -> Comp4lockR {
        Comp4lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 4 enable"]
    #[inline(always)]
    pub fn comp4en(&mut self) -> Comp4enW<'_, Comp4CsrSpec> {
        Comp4enW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Comparator 4 mode"]
    #[inline(always)]
    pub fn comp4mode(&mut self) -> Comp4modeW<'_, Comp4CsrSpec> {
        Comp4modeW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 4 inverting input selection"]
    #[inline(always)]
    pub fn comp4insel(&mut self) -> Comp4inselW<'_, Comp4CsrSpec> {
        Comp4inselW::new(self, 4)
    }
    #[doc = "Bit 7 - Comparator 4 non inverted input selection"]
    #[inline(always)]
    pub fn comp4inpsel(&mut self) -> Comp4inpselW<'_, Comp4CsrSpec> {
        Comp4inpselW::new(self, 7)
    }
    #[doc = "Bit 9 - Comparator 4 window mode"]
    #[inline(always)]
    pub fn com4winmode(&mut self) -> Com4winmodeW<'_, Comp4CsrSpec> {
        Com4winmodeW::new(self, 9)
    }
    #[doc = "Bits 10:13 - Comparator 4 output selection"]
    #[inline(always)]
    pub fn comp4_out_sel(&mut self) -> Comp4OutSelW<'_, Comp4CsrSpec> {
        Comp4OutSelW::new(self, 10)
    }
    #[doc = "Bit 15 - Comparator 4 output polarity"]
    #[inline(always)]
    pub fn comp4pol(&mut self) -> Comp4polW<'_, Comp4CsrSpec> {
        Comp4polW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 4 hysteresis"]
    #[inline(always)]
    pub fn comp4hyst(&mut self) -> Comp4hystW<'_, Comp4CsrSpec> {
        Comp4hystW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator 4 blanking source"]
    #[inline(always)]
    pub fn comp4_blanking(&mut self) -> Comp4BlankingW<'_, Comp4CsrSpec> {
        Comp4BlankingW::new(self, 18)
    }
    #[doc = "Bit 31 - Comparator 4 lock"]
    #[inline(always)]
    pub fn comp4lock(&mut self) -> Comp4lockW<'_, Comp4CsrSpec> {
        Comp4lockW::new(self, 31)
    }
}
#[doc = "control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp4_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp4_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp4CsrSpec;
impl crate::RegisterSpec for Comp4CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp4_csr::R`](R) reader structure"]
impl crate::Readable for Comp4CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`comp4_csr::W`](W) writer structure"]
impl crate::Writable for Comp4CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP4_CSR to value 0"]
impl crate::Resettable for Comp4CsrSpec {}
