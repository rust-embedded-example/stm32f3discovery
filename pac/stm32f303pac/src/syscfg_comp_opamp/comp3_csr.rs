#[doc = "Register `COMP3_CSR` reader"]
pub type R = crate::R<Comp3CsrSpec>;
#[doc = "Register `COMP3_CSR` writer"]
pub type W = crate::W<Comp3CsrSpec>;
#[doc = "Field `COMP3EN` reader - Comparator 3 enable"]
pub type Comp3enR = crate::BitReader;
#[doc = "Field `COMP3EN` writer - Comparator 3 enable"]
pub type Comp3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP3MODE` reader - Comparator 3 mode"]
pub type Comp3modeR = crate::FieldReader;
#[doc = "Field `COMP3MODE` writer - Comparator 3 mode"]
pub type Comp3modeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP3INSEL` reader - Comparator 3 inverting input selection"]
pub type Comp3inselR = crate::FieldReader;
#[doc = "Field `COMP3INSEL` writer - Comparator 3 inverting input selection"]
pub type Comp3inselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP3INPSEL` reader - Comparator 3 non inverted input selection"]
pub type Comp3inpselR = crate::BitReader;
#[doc = "Field `COMP3INPSEL` writer - Comparator 3 non inverted input selection"]
pub type Comp3inpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP3_OUT_SEL` reader - Comparator 3 output selection"]
pub type Comp3OutSelR = crate::FieldReader;
#[doc = "Field `COMP3_OUT_SEL` writer - Comparator 3 output selection"]
pub type Comp3OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COMP3POL` reader - Comparator 3 output polarity"]
pub type Comp3polR = crate::BitReader;
#[doc = "Field `COMP3POL` writer - Comparator 3 output polarity"]
pub type Comp3polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP3HYST` reader - Comparator 3 hysteresis"]
pub type Comp3hystR = crate::FieldReader;
#[doc = "Field `COMP3HYST` writer - Comparator 3 hysteresis"]
pub type Comp3hystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP3_BLANKING` reader - Comparator 3 blanking source"]
pub type Comp3BlankingR = crate::FieldReader;
#[doc = "Field `COMP3_BLANKING` writer - Comparator 3 blanking source"]
pub type Comp3BlankingW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMP3OUT` reader - Comparator 3 output"]
pub type Comp3outR = crate::BitReader;
#[doc = "Field `COMP3LOCK` reader - Comparator 3 lock"]
pub type Comp3lockR = crate::BitReader;
#[doc = "Field `COMP3LOCK` writer - Comparator 3 lock"]
pub type Comp3lockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 3 enable"]
    #[inline(always)]
    pub fn comp3en(&self) -> Comp3enR {
        Comp3enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 3 mode"]
    #[inline(always)]
    pub fn comp3mode(&self) -> Comp3modeR {
        Comp3modeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 3 inverting input selection"]
    #[inline(always)]
    pub fn comp3insel(&self) -> Comp3inselR {
        Comp3inselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator 3 non inverted input selection"]
    #[inline(always)]
    pub fn comp3inpsel(&self) -> Comp3inpselR {
        Comp3inpselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 3 output selection"]
    #[inline(always)]
    pub fn comp3_out_sel(&self) -> Comp3OutSelR {
        Comp3OutSelR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 3 output polarity"]
    #[inline(always)]
    pub fn comp3pol(&self) -> Comp3polR {
        Comp3polR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 3 hysteresis"]
    #[inline(always)]
    pub fn comp3hyst(&self) -> Comp3hystR {
        Comp3hystR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 3 blanking source"]
    #[inline(always)]
    pub fn comp3_blanking(&self) -> Comp3BlankingR {
        Comp3BlankingR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator 3 output"]
    #[inline(always)]
    pub fn comp3out(&self) -> Comp3outR {
        Comp3outR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 3 lock"]
    #[inline(always)]
    pub fn comp3lock(&self) -> Comp3lockR {
        Comp3lockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 3 enable"]
    #[inline(always)]
    pub fn comp3en(&mut self) -> Comp3enW<'_, Comp3CsrSpec> {
        Comp3enW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Comparator 3 mode"]
    #[inline(always)]
    pub fn comp3mode(&mut self) -> Comp3modeW<'_, Comp3CsrSpec> {
        Comp3modeW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 3 inverting input selection"]
    #[inline(always)]
    pub fn comp3insel(&mut self) -> Comp3inselW<'_, Comp3CsrSpec> {
        Comp3inselW::new(self, 4)
    }
    #[doc = "Bit 7 - Comparator 3 non inverted input selection"]
    #[inline(always)]
    pub fn comp3inpsel(&mut self) -> Comp3inpselW<'_, Comp3CsrSpec> {
        Comp3inpselW::new(self, 7)
    }
    #[doc = "Bits 10:13 - Comparator 3 output selection"]
    #[inline(always)]
    pub fn comp3_out_sel(&mut self) -> Comp3OutSelW<'_, Comp3CsrSpec> {
        Comp3OutSelW::new(self, 10)
    }
    #[doc = "Bit 15 - Comparator 3 output polarity"]
    #[inline(always)]
    pub fn comp3pol(&mut self) -> Comp3polW<'_, Comp3CsrSpec> {
        Comp3polW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 3 hysteresis"]
    #[inline(always)]
    pub fn comp3hyst(&mut self) -> Comp3hystW<'_, Comp3CsrSpec> {
        Comp3hystW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator 3 blanking source"]
    #[inline(always)]
    pub fn comp3_blanking(&mut self) -> Comp3BlankingW<'_, Comp3CsrSpec> {
        Comp3BlankingW::new(self, 18)
    }
    #[doc = "Bit 31 - Comparator 3 lock"]
    #[inline(always)]
    pub fn comp3lock(&mut self) -> Comp3lockW<'_, Comp3CsrSpec> {
        Comp3lockW::new(self, 31)
    }
}
#[doc = "control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp3_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp3_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp3CsrSpec;
impl crate::RegisterSpec for Comp3CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp3_csr::R`](R) reader structure"]
impl crate::Readable for Comp3CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`comp3_csr::W`](W) writer structure"]
impl crate::Writable for Comp3CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP3_CSR to value 0"]
impl crate::Resettable for Comp3CsrSpec {}
