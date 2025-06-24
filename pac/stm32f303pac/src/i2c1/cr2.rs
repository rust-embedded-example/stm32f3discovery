#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `SADD0` reader - Slave address bit 0 (master mode)"]
pub type Sadd0R = crate::BitReader;
#[doc = "Field `SADD0` writer - Slave address bit 0 (master mode)"]
pub type Sadd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADD1` reader - Slave address bit 7:1 (master mode)"]
pub type Sadd1R = crate::FieldReader;
#[doc = "Field `SADD1` writer - Slave address bit 7:1 (master mode)"]
pub type Sadd1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SADD8` reader - Slave address bit 9:8 (master mode)"]
pub type Sadd8R = crate::FieldReader;
#[doc = "Field `SADD8` writer - Slave address bit 9:8 (master mode)"]
pub type Sadd8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RD_WRN` reader - Transfer direction (master mode)"]
pub type RdWrnR = crate::BitReader;
#[doc = "Field `RD_WRN` writer - Transfer direction (master mode)"]
pub type RdWrnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD10` reader - 10-bit addressing mode (master mode)"]
pub type Add10R = crate::BitReader;
#[doc = "Field `ADD10` writer - 10-bit addressing mode (master mode)"]
pub type Add10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEAD10R` reader - 10-bit address header only read direction (master receiver mode)"]
pub type Head10rR = crate::BitReader;
#[doc = "Field `HEAD10R` writer - 10-bit address header only read direction (master receiver mode)"]
pub type Head10rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Start generation"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start generation"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - Stop generation (master mode)"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Stop generation (master mode)"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - NACK generation (slave mode)"]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - NACK generation (slave mode)"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBYTES` reader - Number of bytes"]
pub type NbytesR = crate::FieldReader;
#[doc = "Field `NBYTES` writer - Number of bytes"]
pub type NbytesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RELOAD` reader - NBYTES reload mode"]
pub type ReloadR = crate::BitReader;
#[doc = "Field `RELOAD` writer - NBYTES reload mode"]
pub type ReloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOEND` reader - Automatic end mode (master mode)"]
pub type AutoendR = crate::BitReader;
#[doc = "Field `AUTOEND` writer - Automatic end mode (master mode)"]
pub type AutoendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECBYTE` reader - Packet error checking byte"]
pub type PecbyteR = crate::BitReader;
#[doc = "Field `PECBYTE` writer - Packet error checking byte"]
pub type PecbyteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Slave address bit 0 (master mode)"]
    #[inline(always)]
    pub fn sadd0(&self) -> Sadd0R {
        Sadd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Slave address bit 7:1 (master mode)"]
    #[inline(always)]
    pub fn sadd1(&self) -> Sadd1R {
        Sadd1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Slave address bit 9:8 (master mode)"]
    #[inline(always)]
    pub fn sadd8(&self) -> Sadd8R {
        Sadd8R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Transfer direction (master mode)"]
    #[inline(always)]
    pub fn rd_wrn(&self) -> RdWrnR {
        RdWrnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode)"]
    #[inline(always)]
    pub fn add10(&self) -> Add10R {
        Add10R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)"]
    #[inline(always)]
    pub fn head10r(&self) -> Head10rR {
        Head10rR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Start generation"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop generation (master mode)"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NACK generation (slave mode)"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Number of bytes"]
    #[inline(always)]
    pub fn nbytes(&self) -> NbytesR {
        NbytesR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - NBYTES reload mode"]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode)"]
    #[inline(always)]
    pub fn autoend(&self) -> AutoendR {
        AutoendR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Packet error checking byte"]
    #[inline(always)]
    pub fn pecbyte(&self) -> PecbyteR {
        PecbyteR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave address bit 0 (master mode)"]
    #[inline(always)]
    pub fn sadd0(&mut self) -> Sadd0W<Cr2Spec> {
        Sadd0W::new(self, 0)
    }
    #[doc = "Bits 1:7 - Slave address bit 7:1 (master mode)"]
    #[inline(always)]
    pub fn sadd1(&mut self) -> Sadd1W<Cr2Spec> {
        Sadd1W::new(self, 1)
    }
    #[doc = "Bits 8:9 - Slave address bit 9:8 (master mode)"]
    #[inline(always)]
    pub fn sadd8(&mut self) -> Sadd8W<Cr2Spec> {
        Sadd8W::new(self, 8)
    }
    #[doc = "Bit 10 - Transfer direction (master mode)"]
    #[inline(always)]
    pub fn rd_wrn(&mut self) -> RdWrnW<Cr2Spec> {
        RdWrnW::new(self, 10)
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode)"]
    #[inline(always)]
    pub fn add10(&mut self) -> Add10W<Cr2Spec> {
        Add10W::new(self, 11)
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)"]
    #[inline(always)]
    pub fn head10r(&mut self) -> Head10rW<Cr2Spec> {
        Head10rW::new(self, 12)
    }
    #[doc = "Bit 13 - Start generation"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<Cr2Spec> {
        StartW::new(self, 13)
    }
    #[doc = "Bit 14 - Stop generation (master mode)"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<Cr2Spec> {
        StopW::new(self, 14)
    }
    #[doc = "Bit 15 - NACK generation (slave mode)"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<Cr2Spec> {
        NackW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Number of bytes"]
    #[inline(always)]
    pub fn nbytes(&mut self) -> NbytesW<Cr2Spec> {
        NbytesW::new(self, 16)
    }
    #[doc = "Bit 24 - NBYTES reload mode"]
    #[inline(always)]
    pub fn reload(&mut self) -> ReloadW<Cr2Spec> {
        ReloadW::new(self, 24)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode)"]
    #[inline(always)]
    pub fn autoend(&mut self) -> AutoendW<Cr2Spec> {
        AutoendW::new(self, 25)
    }
    #[doc = "Bit 26 - Packet error checking byte"]
    #[inline(always)]
    pub fn pecbyte(&mut self) -> PecbyteW<Cr2Spec> {
        PecbyteW::new(self, 26)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
