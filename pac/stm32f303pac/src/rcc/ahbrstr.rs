#[doc = "Register `AHBRSTR` reader"]
pub type R = crate::R<AhbrstrSpec>;
#[doc = "Register `AHBRSTR` writer"]
pub type W = crate::W<AhbrstrSpec>;
#[doc = "Field `FMCRST` reader - FMC reset"]
pub type FmcrstR = crate::BitReader;
#[doc = "Field `FMCRST` writer - FMC reset"]
pub type FmcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPHRST` reader - I/O port H reset"]
pub type IophrstR = crate::BitReader;
#[doc = "Field `IOPHRST` writer - I/O port H reset"]
pub type IophrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPARST` reader - I/O port A reset"]
pub type IoparstR = crate::BitReader;
#[doc = "Field `IOPARST` writer - I/O port A reset"]
pub type IoparstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPBRST` reader - I/O port B reset"]
pub type IopbrstR = crate::BitReader;
#[doc = "Field `IOPBRST` writer - I/O port B reset"]
pub type IopbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPCRST` reader - I/O port C reset"]
pub type IopcrstR = crate::BitReader;
#[doc = "Field `IOPCRST` writer - I/O port C reset"]
pub type IopcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPDRST` reader - I/O port D reset"]
pub type IopdrstR = crate::BitReader;
#[doc = "Field `IOPDRST` writer - I/O port D reset"]
pub type IopdrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPERST` reader - I/O port E reset"]
pub type IoperstR = crate::BitReader;
#[doc = "Field `IOPERST` writer - I/O port E reset"]
pub type IoperstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPFRST` reader - I/O port F reset"]
pub type IopfrstR = crate::BitReader;
#[doc = "Field `IOPFRST` writer - I/O port F reset"]
pub type IopfrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPGRST` reader - Touch sensing controller reset"]
pub type IopgrstR = crate::BitReader;
#[doc = "Field `IOPGRST` writer - Touch sensing controller reset"]
pub type IopgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCRST` reader - Touch sensing controller reset"]
pub type TscrstR = crate::BitReader;
#[doc = "Field `TSCRST` writer - Touch sensing controller reset"]
pub type TscrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12RST` reader - ADC1 and ADC2 reset"]
pub type Adc12rstR = crate::BitReader;
#[doc = "Field `ADC12RST` writer - ADC1 and ADC2 reset"]
pub type Adc12rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC34RST` reader - ADC3 and ADC4 reset"]
pub type Adc34rstR = crate::BitReader;
#[doc = "Field `ADC34RST` writer - ADC3 and ADC4 reset"]
pub type Adc34rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - FMC reset"]
    #[inline(always)]
    pub fn fmcrst(&self) -> FmcrstR {
        FmcrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - I/O port H reset"]
    #[inline(always)]
    pub fn iophrst(&self) -> IophrstR {
        IophrstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I/O port A reset"]
    #[inline(always)]
    pub fn ioparst(&self) -> IoparstR {
        IoparstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I/O port B reset"]
    #[inline(always)]
    pub fn iopbrst(&self) -> IopbrstR {
        IopbrstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I/O port C reset"]
    #[inline(always)]
    pub fn iopcrst(&self) -> IopcrstR {
        IopcrstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I/O port D reset"]
    #[inline(always)]
    pub fn iopdrst(&self) -> IopdrstR {
        IopdrstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I/O port E reset"]
    #[inline(always)]
    pub fn ioperst(&self) -> IoperstR {
        IoperstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I/O port F reset"]
    #[inline(always)]
    pub fn iopfrst(&self) -> IopfrstR {
        IopfrstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Touch sensing controller reset"]
    #[inline(always)]
    pub fn iopgrst(&self) -> IopgrstR {
        IopgrstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Touch sensing controller reset"]
    #[inline(always)]
    pub fn tscrst(&self) -> TscrstR {
        TscrstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC1 and ADC2 reset"]
    #[inline(always)]
    pub fn adc12rst(&self) -> Adc12rstR {
        Adc12rstR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ADC3 and ADC4 reset"]
    #[inline(always)]
    pub fn adc34rst(&self) -> Adc34rstR {
        Adc34rstR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - FMC reset"]
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FmcrstW<'_, AhbrstrSpec> {
        FmcrstW::new(self, 5)
    }
    #[doc = "Bit 16 - I/O port H reset"]
    #[inline(always)]
    pub fn iophrst(&mut self) -> IophrstW<'_, AhbrstrSpec> {
        IophrstW::new(self, 16)
    }
    #[doc = "Bit 17 - I/O port A reset"]
    #[inline(always)]
    pub fn ioparst(&mut self) -> IoparstW<'_, AhbrstrSpec> {
        IoparstW::new(self, 17)
    }
    #[doc = "Bit 18 - I/O port B reset"]
    #[inline(always)]
    pub fn iopbrst(&mut self) -> IopbrstW<'_, AhbrstrSpec> {
        IopbrstW::new(self, 18)
    }
    #[doc = "Bit 19 - I/O port C reset"]
    #[inline(always)]
    pub fn iopcrst(&mut self) -> IopcrstW<'_, AhbrstrSpec> {
        IopcrstW::new(self, 19)
    }
    #[doc = "Bit 20 - I/O port D reset"]
    #[inline(always)]
    pub fn iopdrst(&mut self) -> IopdrstW<'_, AhbrstrSpec> {
        IopdrstW::new(self, 20)
    }
    #[doc = "Bit 21 - I/O port E reset"]
    #[inline(always)]
    pub fn ioperst(&mut self) -> IoperstW<'_, AhbrstrSpec> {
        IoperstW::new(self, 21)
    }
    #[doc = "Bit 22 - I/O port F reset"]
    #[inline(always)]
    pub fn iopfrst(&mut self) -> IopfrstW<'_, AhbrstrSpec> {
        IopfrstW::new(self, 22)
    }
    #[doc = "Bit 23 - Touch sensing controller reset"]
    #[inline(always)]
    pub fn iopgrst(&mut self) -> IopgrstW<'_, AhbrstrSpec> {
        IopgrstW::new(self, 23)
    }
    #[doc = "Bit 24 - Touch sensing controller reset"]
    #[inline(always)]
    pub fn tscrst(&mut self) -> TscrstW<'_, AhbrstrSpec> {
        TscrstW::new(self, 24)
    }
    #[doc = "Bit 28 - ADC1 and ADC2 reset"]
    #[inline(always)]
    pub fn adc12rst(&mut self) -> Adc12rstW<'_, AhbrstrSpec> {
        Adc12rstW::new(self, 28)
    }
    #[doc = "Bit 29 - ADC3 and ADC4 reset"]
    #[inline(always)]
    pub fn adc34rst(&mut self) -> Adc34rstW<'_, AhbrstrSpec> {
        Adc34rstW::new(self, 29)
    }
}
#[doc = "AHB peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbrstrSpec;
impl crate::RegisterSpec for AhbrstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrstr::R`](R) reader structure"]
impl crate::Readable for AhbrstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure"]
impl crate::Writable for AhbrstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AhbrstrSpec {}
