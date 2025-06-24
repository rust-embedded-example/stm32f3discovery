#[doc = "Register `COMP6_CSR` reader"]
pub type R = crate::R<Comp6CsrSpec>;
#[doc = "Register `COMP6_CSR` writer"]
pub type W = crate::W<Comp6CsrSpec>;
#[doc = "Field `COMP6EN` reader - Comparator 6 enable"]
pub type Comp6enR = crate::BitReader;
#[doc = "Field `COMP6EN` writer - Comparator 6 enable"]
pub type Comp6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP6MODE` reader - Comparator 6 mode"]
pub type Comp6modeR = crate::FieldReader;
#[doc = "Field `COMP6MODE` writer - Comparator 6 mode"]
pub type Comp6modeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP6INSEL` reader - Comparator 6 inverting input selection"]
pub type Comp6inselR = crate::FieldReader;
#[doc = "Field `COMP6INSEL` writer - Comparator 6 inverting input selection"]
pub type Comp6inselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP6INPSEL` reader - Comparator 6 non inverted input selection"]
pub type Comp6inpselR = crate::BitReader;
#[doc = "Field `COMP6INPSEL` writer - Comparator 6 non inverted input selection"]
pub type Comp6inpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COM6WINMODE` reader - Comparator 6 window mode"]
pub type Com6winmodeR = crate::BitReader;
#[doc = "Field `COM6WINMODE` writer - Comparator 6 window mode"]
pub type Com6winmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP6_OUT_SEL` reader - Comparator 6 output selection"]
pub type Comp6OutSelR = crate::FieldReader;
#[doc = "Field `COMP6_OUT_SEL` writer - Comparator 6 output selection"]
pub type Comp6OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COMP6POL` reader - Comparator 6 output polarity"]
pub type Comp6polR = crate::BitReader;
#[doc = "Field `COMP6POL` writer - Comparator 6 output polarity"]
pub type Comp6polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP6HYST` reader - Comparator 6 hysteresis"]
pub type Comp6hystR = crate::FieldReader;
#[doc = "Field `COMP6HYST` writer - Comparator 6 hysteresis"]
pub type Comp6hystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP6_BLANKING` reader - Comparator 6 blanking source"]
pub type Comp6BlankingR = crate::FieldReader;
#[doc = "Field `COMP6_BLANKING` writer - Comparator 6 blanking source"]
pub type Comp6BlankingW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP6OUT` reader - Comparator 6 output"]
pub type Comp6outR = crate::BitReader;
#[doc = "Field `COMP6LOCK` reader - Comparator 6 lock"]
pub type Comp6lockR = crate::BitReader;
#[doc = "Field `COMP6LOCK` writer - Comparator 6 lock"]
pub type Comp6lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 6 enable"]
    #[inline(always)]
    pub fn comp6en(&self) -> Comp6enR {
        Comp6enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 6 mode"]
    #[inline(always)]
    pub fn comp6mode(&self) -> Comp6modeR {
        Comp6modeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 6 inverting input selection"]
    #[inline(always)]
    pub fn comp6insel(&self) -> Comp6inselR {
        Comp6inselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator 6 non inverted input selection"]
    #[inline(always)]
    pub fn comp6inpsel(&self) -> Comp6inpselR {
        Comp6inpselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator 6 window mode"]
    #[inline(always)]
    pub fn com6winmode(&self) -> Com6winmodeR {
        Com6winmodeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 6 output selection"]
    #[inline(always)]
    pub fn comp6_out_sel(&self) -> Comp6OutSelR {
        Comp6OutSelR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 6 output polarity"]
    #[inline(always)]
    pub fn comp6pol(&self) -> Comp6polR {
        Comp6polR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 6 hysteresis"]
    #[inline(always)]
    pub fn comp6hyst(&self) -> Comp6hystR {
        Comp6hystR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 6 blanking source"]
    #[inline(always)]
    pub fn comp6_blanking(&self) -> Comp6BlankingR {
        Comp6BlankingR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator 6 output"]
    #[inline(always)]
    pub fn comp6out(&self) -> Comp6outR {
        Comp6outR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 6 lock"]
    #[inline(always)]
    pub fn comp6lock(&self) -> Comp6lockR {
        Comp6lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 6 enable"]
    #[inline(always)]
    pub fn comp6en(&mut self) -> Comp6enW<'_, Comp6CsrSpec> {
        Comp6enW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Comparator 6 mode"]
    #[inline(always)]
    pub fn comp6mode(&mut self) -> Comp6modeW<'_, Comp6CsrSpec> {
        Comp6modeW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 6 inverting input selection"]
    #[inline(always)]
    pub fn comp6insel(&mut self) -> Comp6inselW<'_, Comp6CsrSpec> {
        Comp6inselW::new(self, 4)
    }
    #[doc = "Bit 7 - Comparator 6 non inverted input selection"]
    #[inline(always)]
    pub fn comp6inpsel(&mut self) -> Comp6inpselW<'_, Comp6CsrSpec> {
        Comp6inpselW::new(self, 7)
    }
    #[doc = "Bit 9 - Comparator 6 window mode"]
    #[inline(always)]
    pub fn com6winmode(&mut self) -> Com6winmodeW<'_, Comp6CsrSpec> {
        Com6winmodeW::new(self, 9)
    }
    #[doc = "Bits 10:13 - Comparator 6 output selection"]
    #[inline(always)]
    pub fn comp6_out_sel(&mut self) -> Comp6OutSelW<'_, Comp6CsrSpec> {
        Comp6OutSelW::new(self, 10)
    }
    #[doc = "Bit 15 - Comparator 6 output polarity"]
    #[inline(always)]
    pub fn comp6pol(&mut self) -> Comp6polW<'_, Comp6CsrSpec> {
        Comp6polW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 6 hysteresis"]
    #[inline(always)]
    pub fn comp6hyst(&mut self) -> Comp6hystW<'_, Comp6CsrSpec> {
        Comp6hystW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator 6 blanking source"]
    #[inline(always)]
    pub fn comp6_blanking(&mut self) -> Comp6BlankingW<'_, Comp6CsrSpec> {
        Comp6BlankingW::new(self, 18)
    }
    #[doc = "Bit 31 - Comparator 6 lock"]
    #[inline(always)]
    pub fn comp6lock(&mut self) -> Comp6lockW<'_, Comp6CsrSpec> {
        Comp6lockW::new(self, 31)
    }
}
#[doc = "control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp6_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp6_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp6CsrSpec;
impl crate::RegisterSpec for Comp6CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp6_csr::R`](R) reader structure"]
impl crate::Readable for Comp6CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`comp6_csr::W`](W) writer structure"]
impl crate::Writable for Comp6CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP6_CSR to value 0"]
impl crate::Resettable for Comp6CsrSpec {}
