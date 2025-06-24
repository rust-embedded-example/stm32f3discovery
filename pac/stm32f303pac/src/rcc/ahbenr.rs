#[doc = "Register `AHBENR` reader"]
pub type R = crate::R<AhbenrSpec>;
#[doc = "Register `AHBENR` writer"]
pub type W = crate::W<AhbenrSpec>;
#[doc = "Field `DMAEN` reader - DMA1 clock enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA1 clock enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub type Dma2enR = crate::BitReader;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub type Dma2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMEN` reader - SRAM interface clock enable"]
pub type SramenR = crate::BitReader;
#[doc = "Field `SRAMEN` writer - SRAM interface clock enable"]
pub type SramenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLITFEN` reader - FLITF clock enable"]
pub type FlitfenR = crate::BitReader;
#[doc = "Field `FLITFEN` writer - FLITF clock enable"]
pub type FlitfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMCEN` reader - FMC clock enable"]
pub type FmcenR = crate::BitReader;
#[doc = "Field `FMCEN` writer - FMC clock enable"]
pub type FmcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPHEN` reader - IO port H clock enable"]
pub type IophenR = crate::BitReader;
#[doc = "Field `IOPHEN` writer - IO port H clock enable"]
pub type IophenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPAEN` reader - I/O port A clock enable"]
pub type IopaenR = crate::BitReader;
#[doc = "Field `IOPAEN` writer - I/O port A clock enable"]
pub type IopaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPBEN` reader - I/O port B clock enable"]
pub type IopbenR = crate::BitReader;
#[doc = "Field `IOPBEN` writer - I/O port B clock enable"]
pub type IopbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPCEN` reader - I/O port C clock enable"]
pub type IopcenR = crate::BitReader;
#[doc = "Field `IOPCEN` writer - I/O port C clock enable"]
pub type IopcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPDEN` reader - I/O port D clock enable"]
pub type IopdenR = crate::BitReader;
#[doc = "Field `IOPDEN` writer - I/O port D clock enable"]
pub type IopdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPEEN` reader - I/O port E clock enable"]
pub type IopeenR = crate::BitReader;
#[doc = "Field `IOPEEN` writer - I/O port E clock enable"]
pub type IopeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPFEN` reader - I/O port F clock enable"]
pub type IopfenR = crate::BitReader;
#[doc = "Field `IOPFEN` writer - I/O port F clock enable"]
pub type IopfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPGEN` reader - I/O port G clock enable"]
pub type IopgenR = crate::BitReader;
#[doc = "Field `IOPGEN` writer - I/O port G clock enable"]
pub type IopgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCEN` reader - Touch sensing controller clock enable"]
pub type TscenR = crate::BitReader;
#[doc = "Field `TSCEN` writer - Touch sensing controller clock enable"]
pub type TscenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12EN` reader - ADC1 and ADC2 clock enable"]
pub type Adc12enR = crate::BitReader;
#[doc = "Field `ADC12EN` writer - ADC1 and ADC2 clock enable"]
pub type Adc12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC34EN` reader - ADC3 and ADC4 clock enable"]
pub type Adc34enR = crate::BitReader;
#[doc = "Field `ADC34EN` writer - ADC3 and ADC4 clock enable"]
pub type Adc34enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> Dma2enR {
        Dma2enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&self) -> SramenR {
        SramenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    pub fn flitfen(&self) -> FlitfenR {
        FlitfenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FMC clock enable"]
    #[inline(always)]
    pub fn fmcen(&self) -> FmcenR {
        FmcenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - IO port H clock enable"]
    #[inline(always)]
    pub fn iophen(&self) -> IophenR {
        IophenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline(always)]
    pub fn iopaen(&self) -> IopaenR {
        IopaenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline(always)]
    pub fn iopben(&self) -> IopbenR {
        IopbenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I/O port C clock enable"]
    #[inline(always)]
    pub fn iopcen(&self) -> IopcenR {
        IopcenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I/O port D clock enable"]
    #[inline(always)]
    pub fn iopden(&self) -> IopdenR {
        IopdenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I/O port E clock enable"]
    #[inline(always)]
    pub fn iopeen(&self) -> IopeenR {
        IopeenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I/O port F clock enable"]
    #[inline(always)]
    pub fn iopfen(&self) -> IopfenR {
        IopfenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I/O port G clock enable"]
    #[inline(always)]
    pub fn iopgen(&self) -> IopgenR {
        IopgenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Touch sensing controller clock enable"]
    #[inline(always)]
    pub fn tscen(&self) -> TscenR {
        TscenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC1 and ADC2 clock enable"]
    #[inline(always)]
    pub fn adc12en(&self) -> Adc12enR {
        Adc12enR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ADC3 and ADC4 clock enable"]
    #[inline(always)]
    pub fn adc34en(&self) -> Adc34enR {
        Adc34enR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, AhbenrSpec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> Dma2enW<'_, AhbenrSpec> {
        Dma2enW::new(self, 1)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&mut self) -> SramenW<'_, AhbenrSpec> {
        SramenW::new(self, 2)
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    pub fn flitfen(&mut self) -> FlitfenW<'_, AhbenrSpec> {
        FlitfenW::new(self, 4)
    }
    #[doc = "Bit 5 - FMC clock enable"]
    #[inline(always)]
    pub fn fmcen(&mut self) -> FmcenW<'_, AhbenrSpec> {
        FmcenW::new(self, 5)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<'_, AhbenrSpec> {
        CrcenW::new(self, 6)
    }
    #[doc = "Bit 16 - IO port H clock enable"]
    #[inline(always)]
    pub fn iophen(&mut self) -> IophenW<'_, AhbenrSpec> {
        IophenW::new(self, 16)
    }
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline(always)]
    pub fn iopaen(&mut self) -> IopaenW<'_, AhbenrSpec> {
        IopaenW::new(self, 17)
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline(always)]
    pub fn iopben(&mut self) -> IopbenW<'_, AhbenrSpec> {
        IopbenW::new(self, 18)
    }
    #[doc = "Bit 19 - I/O port C clock enable"]
    #[inline(always)]
    pub fn iopcen(&mut self) -> IopcenW<'_, AhbenrSpec> {
        IopcenW::new(self, 19)
    }
    #[doc = "Bit 20 - I/O port D clock enable"]
    #[inline(always)]
    pub fn iopden(&mut self) -> IopdenW<'_, AhbenrSpec> {
        IopdenW::new(self, 20)
    }
    #[doc = "Bit 21 - I/O port E clock enable"]
    #[inline(always)]
    pub fn iopeen(&mut self) -> IopeenW<'_, AhbenrSpec> {
        IopeenW::new(self, 21)
    }
    #[doc = "Bit 22 - I/O port F clock enable"]
    #[inline(always)]
    pub fn iopfen(&mut self) -> IopfenW<'_, AhbenrSpec> {
        IopfenW::new(self, 22)
    }
    #[doc = "Bit 23 - I/O port G clock enable"]
    #[inline(always)]
    pub fn iopgen(&mut self) -> IopgenW<'_, AhbenrSpec> {
        IopgenW::new(self, 23)
    }
    #[doc = "Bit 24 - Touch sensing controller clock enable"]
    #[inline(always)]
    pub fn tscen(&mut self) -> TscenW<'_, AhbenrSpec> {
        TscenW::new(self, 24)
    }
    #[doc = "Bit 28 - ADC1 and ADC2 clock enable"]
    #[inline(always)]
    pub fn adc12en(&mut self) -> Adc12enW<'_, AhbenrSpec> {
        Adc12enW::new(self, 28)
    }
    #[doc = "Bit 29 - ADC3 and ADC4 clock enable"]
    #[inline(always)]
    pub fn adc34en(&mut self) -> Adc34enW<'_, AhbenrSpec> {
        Adc34enW::new(self, 29)
    }
}
#[doc = "AHB Peripheral Clock enable register (RCC_AHBENR)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbenrSpec;
impl crate::RegisterSpec for AhbenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbenr::R`](R) reader structure"]
impl crate::Readable for AhbenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbenr::W`](W) writer structure"]
impl crate::Writable for AhbenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBENR to value 0x14"]
impl crate::Resettable for AhbenrSpec {
    const RESET_VALUE: u32 = 0x14;
}
