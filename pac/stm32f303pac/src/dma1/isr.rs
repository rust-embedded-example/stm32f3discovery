#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `GIF1` reader - Channel 1 Global interrupt flag"]
pub type Gif1R = crate::BitReader;
#[doc = "Field `TCIF1` reader - Channel 1 Transfer Complete flag"]
pub type Tcif1R = crate::BitReader;
#[doc = "Field `HTIF1` reader - Channel 1 Half Transfer Complete flag"]
pub type Htif1R = crate::BitReader;
#[doc = "Field `TEIF1` reader - Channel 1 Transfer Error flag"]
pub type Teif1R = crate::BitReader;
#[doc = "Field `GIF2` reader - Channel 2 Global interrupt flag"]
pub type Gif2R = crate::BitReader;
#[doc = "Field `TCIF2` reader - Channel 2 Transfer Complete flag"]
pub type Tcif2R = crate::BitReader;
#[doc = "Field `HTIF2` reader - Channel 2 Half Transfer Complete flag"]
pub type Htif2R = crate::BitReader;
#[doc = "Field `TEIF2` reader - Channel 2 Transfer Error flag"]
pub type Teif2R = crate::BitReader;
#[doc = "Field `GIF3` reader - Channel 3 Global interrupt flag"]
pub type Gif3R = crate::BitReader;
#[doc = "Field `TCIF3` reader - Channel 3 Transfer Complete flag"]
pub type Tcif3R = crate::BitReader;
#[doc = "Field `HTIF3` reader - Channel 3 Half Transfer Complete flag"]
pub type Htif3R = crate::BitReader;
#[doc = "Field `TEIF3` reader - Channel 3 Transfer Error flag"]
pub type Teif3R = crate::BitReader;
#[doc = "Field `GIF4` reader - Channel 4 Global interrupt flag"]
pub type Gif4R = crate::BitReader;
#[doc = "Field `TCIF4` reader - Channel 4 Transfer Complete flag"]
pub type Tcif4R = crate::BitReader;
#[doc = "Field `HTIF4` reader - Channel 4 Half Transfer Complete flag"]
pub type Htif4R = crate::BitReader;
#[doc = "Field `TEIF4` reader - Channel 4 Transfer Error flag"]
pub type Teif4R = crate::BitReader;
#[doc = "Field `GIF5` reader - Channel 5 Global interrupt flag"]
pub type Gif5R = crate::BitReader;
#[doc = "Field `TCIF5` reader - Channel 5 Transfer Complete flag"]
pub type Tcif5R = crate::BitReader;
#[doc = "Field `HTIF5` reader - Channel 5 Half Transfer Complete flag"]
pub type Htif5R = crate::BitReader;
#[doc = "Field `TEIF5` reader - Channel 5 Transfer Error flag"]
pub type Teif5R = crate::BitReader;
#[doc = "Field `GIF6` reader - Channel 6 Global interrupt flag"]
pub type Gif6R = crate::BitReader;
#[doc = "Field `TCIF6` reader - Channel 6 Transfer Complete flag"]
pub type Tcif6R = crate::BitReader;
#[doc = "Field `HTIF6` reader - Channel 6 Half Transfer Complete flag"]
pub type Htif6R = crate::BitReader;
#[doc = "Field `TEIF6` reader - Channel 6 Transfer Error flag"]
pub type Teif6R = crate::BitReader;
#[doc = "Field `GIF7` reader - Channel 7 Global interrupt flag"]
pub type Gif7R = crate::BitReader;
#[doc = "Field `TCIF7` reader - Channel 7 Transfer Complete flag"]
pub type Tcif7R = crate::BitReader;
#[doc = "Field `HTIF7` reader - Channel 7 Half Transfer Complete flag"]
pub type Htif7R = crate::BitReader;
#[doc = "Field `TEIF7` reader - Channel 7 Transfer Error flag"]
pub type Teif7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 1 Global interrupt flag"]
    #[inline(always)]
    pub fn gif1(&self) -> Gif1R {
        Gif1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif1(&self) -> Tcif1R {
        Tcif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif1(&self) -> Htif1R {
        Htif1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 Transfer Error flag"]
    #[inline(always)]
    pub fn teif1(&self) -> Teif1R {
        Teif1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 2 Global interrupt flag"]
    #[inline(always)]
    pub fn gif2(&self) -> Gif2R {
        Gif2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif2(&self) -> Tcif2R {
        Tcif2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 2 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif2(&self) -> Htif2R {
        Htif2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 2 Transfer Error flag"]
    #[inline(always)]
    pub fn teif2(&self) -> Teif2R {
        Teif2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 3 Global interrupt flag"]
    #[inline(always)]
    pub fn gif3(&self) -> Gif3R {
        Gif3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 3 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif3(&self) -> Tcif3R {
        Tcif3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 3 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif3(&self) -> Htif3R {
        Htif3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Transfer Error flag"]
    #[inline(always)]
    pub fn teif3(&self) -> Teif3R {
        Teif3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Global interrupt flag"]
    #[inline(always)]
    pub fn gif4(&self) -> Gif4R {
        Gif4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 4 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif4(&self) -> Tcif4R {
        Tcif4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 4 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif4(&self) -> Htif4R {
        Htif4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 4 Transfer Error flag"]
    #[inline(always)]
    pub fn teif4(&self) -> Teif4R {
        Teif4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 5 Global interrupt flag"]
    #[inline(always)]
    pub fn gif5(&self) -> Gif5R {
        Gif5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 5 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif5(&self) -> Tcif5R {
        Tcif5R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 5 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif5(&self) -> Htif5R {
        Htif5R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 5 Transfer Error flag"]
    #[inline(always)]
    pub fn teif5(&self) -> Teif5R {
        Teif5R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel 6 Global interrupt flag"]
    #[inline(always)]
    pub fn gif6(&self) -> Gif6R {
        Gif6R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel 6 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif6(&self) -> Tcif6R {
        Tcif6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel 6 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif6(&self) -> Htif6R {
        Htif6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 6 Transfer Error flag"]
    #[inline(always)]
    pub fn teif6(&self) -> Teif6R {
        Teif6R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel 7 Global interrupt flag"]
    #[inline(always)]
    pub fn gif7(&self) -> Gif7R {
        Gif7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel 7 Transfer Complete flag"]
    #[inline(always)]
    pub fn tcif7(&self) -> Tcif7R {
        Tcif7R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel 7 Half Transfer Complete flag"]
    #[inline(always)]
    pub fn htif7(&self) -> Htif7R {
        Htif7R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel 7 Transfer Error flag"]
    #[inline(always)]
    pub fn teif7(&self) -> Teif7R {
        Teif7R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "DMA interrupt status register (DMA_ISR)\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
