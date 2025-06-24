#[doc = "Register `IOGCSR` reader"]
pub type R = crate::R<IogcsrSpec>;
#[doc = "Register `IOGCSR` writer"]
pub type W = crate::W<IogcsrSpec>;
#[doc = "Field `G1E` reader - Analog I/O group x enable"]
pub type G1eR = crate::BitReader;
#[doc = "Field `G1E` writer - Analog I/O group x enable"]
pub type G1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G2E` reader - Analog I/O group x enable"]
pub type G2eR = crate::BitReader;
#[doc = "Field `G2E` writer - Analog I/O group x enable"]
pub type G2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G3E` reader - Analog I/O group x enable"]
pub type G3eR = crate::BitReader;
#[doc = "Field `G3E` writer - Analog I/O group x enable"]
pub type G3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G4E` reader - Analog I/O group x enable"]
pub type G4eR = crate::BitReader;
#[doc = "Field `G4E` writer - Analog I/O group x enable"]
pub type G4eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G5E` reader - Analog I/O group x enable"]
pub type G5eR = crate::BitReader;
#[doc = "Field `G5E` writer - Analog I/O group x enable"]
pub type G5eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G6E` reader - Analog I/O group x enable"]
pub type G6eR = crate::BitReader;
#[doc = "Field `G6E` writer - Analog I/O group x enable"]
pub type G6eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G7E` reader - Analog I/O group x enable"]
pub type G7eR = crate::BitReader;
#[doc = "Field `G7E` writer - Analog I/O group x enable"]
pub type G7eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G8E` reader - Analog I/O group x enable"]
pub type G8eR = crate::BitReader;
#[doc = "Field `G8E` writer - Analog I/O group x enable"]
pub type G8eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G1S` reader - Analog I/O group x status"]
pub type G1sR = crate::BitReader;
#[doc = "Field `G2S` reader - Analog I/O group x status"]
pub type G2sR = crate::BitReader;
#[doc = "Field `G3S` reader - Analog I/O group x status"]
pub type G3sR = crate::BitReader;
#[doc = "Field `G4S` reader - Analog I/O group x status"]
pub type G4sR = crate::BitReader;
#[doc = "Field `G5S` reader - Analog I/O group x status"]
pub type G5sR = crate::BitReader;
#[doc = "Field `G6S` reader - Analog I/O group x status"]
pub type G6sR = crate::BitReader;
#[doc = "Field `G7S` reader - Analog I/O group x status"]
pub type G7sR = crate::BitReader;
#[doc = "Field `G7S` writer - Analog I/O group x status"]
pub type G7sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `G8S` reader - Analog I/O group x status"]
pub type G8sR = crate::BitReader;
#[doc = "Field `G8S` writer - Analog I/O group x status"]
pub type G8sW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g1e(&self) -> G1eR {
        G1eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g2e(&self) -> G2eR {
        G2eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g3e(&self) -> G3eR {
        G3eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g4e(&self) -> G4eR {
        G4eR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g5e(&self) -> G5eR {
        G5eR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g6e(&self) -> G6eR {
        G6eR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g7e(&self) -> G7eR {
        G7eR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g8e(&self) -> G8eR {
        G8eR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g1s(&self) -> G1sR {
        G1sR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g2s(&self) -> G2sR {
        G2sR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g3s(&self) -> G3sR {
        G3sR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g4s(&self) -> G4sR {
        G4sR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g5s(&self) -> G5sR {
        G5sR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g6s(&self) -> G6sR {
        G6sR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g7s(&self) -> G7sR {
        G7sR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g8s(&self) -> G8sR {
        G8sR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g1e(&mut self) -> G1eW<'_, IogcsrSpec> {
        G1eW::new(self, 0)
    }
    #[doc = "Bit 1 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g2e(&mut self) -> G2eW<'_, IogcsrSpec> {
        G2eW::new(self, 1)
    }
    #[doc = "Bit 2 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g3e(&mut self) -> G3eW<'_, IogcsrSpec> {
        G3eW::new(self, 2)
    }
    #[doc = "Bit 3 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g4e(&mut self) -> G4eW<'_, IogcsrSpec> {
        G4eW::new(self, 3)
    }
    #[doc = "Bit 4 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g5e(&mut self) -> G5eW<'_, IogcsrSpec> {
        G5eW::new(self, 4)
    }
    #[doc = "Bit 5 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g6e(&mut self) -> G6eW<'_, IogcsrSpec> {
        G6eW::new(self, 5)
    }
    #[doc = "Bit 6 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g7e(&mut self) -> G7eW<'_, IogcsrSpec> {
        G7eW::new(self, 6)
    }
    #[doc = "Bit 7 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g8e(&mut self) -> G8eW<'_, IogcsrSpec> {
        G8eW::new(self, 7)
    }
    #[doc = "Bit 22 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g7s(&mut self) -> G7sW<'_, IogcsrSpec> {
        G7sW::new(self, 22)
    }
    #[doc = "Bit 23 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g8s(&mut self) -> G8sW<'_, IogcsrSpec> {
        G8sW::new(self, 23)
    }
}
#[doc = "I/O group control status register\n\nYou can [`read`](crate::Reg::read) this register and get [`iogcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iogcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IogcsrSpec;
impl crate::RegisterSpec for IogcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iogcsr::R`](R) reader structure"]
impl crate::Readable for IogcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`iogcsr::W`](W) writer structure"]
impl crate::Writable for IogcsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOGCSR to value 0"]
impl crate::Resettable for IogcsrSpec {}
