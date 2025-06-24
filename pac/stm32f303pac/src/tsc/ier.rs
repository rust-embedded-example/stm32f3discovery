#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `EOAIE` reader - End of acquisition interrupt enable"]
pub type EoaieR = crate::BitReader;
#[doc = "Field `EOAIE` writer - End of acquisition interrupt enable"]
pub type EoaieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCEIE` reader - Max count error interrupt enable"]
pub type MceieR = crate::BitReader;
#[doc = "Field `MCEIE` writer - Max count error interrupt enable"]
pub type MceieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - End of acquisition interrupt enable"]
    #[inline(always)]
    pub fn eoaie(&self) -> EoaieR {
        EoaieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max count error interrupt enable"]
    #[inline(always)]
    pub fn mceie(&self) -> MceieR {
        MceieR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of acquisition interrupt enable"]
    #[inline(always)]
    pub fn eoaie(&mut self) -> EoaieW<'_, IerSpec> {
        EoaieW::new(self, 0)
    }
    #[doc = "Bit 1 - Max count error interrupt enable"]
    #[inline(always)]
    pub fn mceie(&mut self) -> MceieW<'_, IerSpec> {
        MceieW::new(self, 1)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
