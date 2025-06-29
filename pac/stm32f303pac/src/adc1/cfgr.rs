#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACFG` reader - DMACFG"]
pub type DmacfgR = crate::BitReader;
#[doc = "Field `DMACFG` writer - DMACFG"]
pub type DmacfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES` reader - RES"]
pub type ResR = crate::FieldReader;
#[doc = "Field `RES` writer - RES"]
pub type ResW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ALIGN` reader - ALIGN"]
pub type AlignR = crate::BitReader;
#[doc = "Field `ALIGN` writer - ALIGN"]
pub type AlignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTSEL` reader - EXTSEL"]
pub type ExtselR = crate::FieldReader;
#[doc = "Field `EXTSEL` writer - EXTSEL"]
pub type ExtselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTEN` reader - EXTEN"]
pub type ExtenR = crate::FieldReader;
#[doc = "Field `EXTEN` writer - EXTEN"]
pub type ExtenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OVRMOD` reader - OVRMOD"]
pub type OvrmodR = crate::BitReader;
#[doc = "Field `OVRMOD` writer - OVRMOD"]
pub type OvrmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT` reader - CONT"]
pub type ContR = crate::BitReader;
#[doc = "Field `CONT` writer - CONT"]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTDLY` reader - AUTDLY"]
pub type AutdlyR = crate::BitReader;
#[doc = "Field `AUTDLY` writer - AUTDLY"]
pub type AutdlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCEN` reader - DISCEN"]
pub type DiscenR = crate::BitReader;
#[doc = "Field `DISCEN` writer - DISCEN"]
pub type DiscenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCNUM` reader - DISCNUM"]
pub type DiscnumR = crate::FieldReader;
#[doc = "Field `DISCNUM` writer - DISCNUM"]
pub type DiscnumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JDISCEN` reader - JDISCEN"]
pub type JdiscenR = crate::BitReader;
#[doc = "Field `JDISCEN` writer - JDISCEN"]
pub type JdiscenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JQM` reader - JQM"]
pub type JqmR = crate::BitReader;
#[doc = "Field `JQM` writer - JQM"]
pub type JqmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1SGL` reader - AWD1SGL"]
pub type Awd1sglR = crate::BitReader;
#[doc = "Field `AWD1SGL` writer - AWD1SGL"]
pub type Awd1sglW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1EN` reader - AWD1EN"]
pub type Awd1enR = crate::BitReader;
#[doc = "Field `AWD1EN` writer - AWD1EN"]
pub type Awd1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JAWD1EN` reader - JAWD1EN"]
pub type Jawd1enR = crate::BitReader;
#[doc = "Field `JAWD1EN` writer - JAWD1EN"]
pub type Jawd1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JAUTO` reader - JAUTO"]
pub type JautoR = crate::BitReader;
#[doc = "Field `JAUTO` writer - JAUTO"]
pub type JautoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDCH1CH` reader - AWDCH1CH"]
pub type Awdch1chR = crate::FieldReader;
#[doc = "Field `AWDCH1CH` writer - AWDCH1CH"]
pub type Awdch1chW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    pub fn dmacfg(&self) -> DmacfgR {
        DmacfgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:4 - RES"]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - ALIGN"]
    #[inline(always)]
    pub fn align(&self) -> AlignR {
        AlignR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - EXTSEL"]
    #[inline(always)]
    pub fn extsel(&self) -> ExtselR {
        ExtselR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - EXTEN"]
    #[inline(always)]
    pub fn exten(&self) -> ExtenR {
        ExtenR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - OVRMOD"]
    #[inline(always)]
    pub fn ovrmod(&self) -> OvrmodR {
        OvrmodR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CONT"]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AUTDLY"]
    #[inline(always)]
    pub fn autdly(&self) -> AutdlyR {
        AutdlyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DISCEN"]
    #[inline(always)]
    pub fn discen(&self) -> DiscenR {
        DiscenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - DISCNUM"]
    #[inline(always)]
    pub fn discnum(&self) -> DiscnumR {
        DiscnumR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - JDISCEN"]
    #[inline(always)]
    pub fn jdiscen(&self) -> JdiscenR {
        JdiscenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - JQM"]
    #[inline(always)]
    pub fn jqm(&self) -> JqmR {
        JqmR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - AWD1SGL"]
    #[inline(always)]
    pub fn awd1sgl(&self) -> Awd1sglR {
        Awd1sglR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AWD1EN"]
    #[inline(always)]
    pub fn awd1en(&self) -> Awd1enR {
        Awd1enR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - JAWD1EN"]
    #[inline(always)]
    pub fn jawd1en(&self) -> Jawd1enR {
        Jawd1enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - JAUTO"]
    #[inline(always)]
    pub fn jauto(&self) -> JautoR {
        JautoR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - AWDCH1CH"]
    #[inline(always)]
    pub fn awdch1ch(&self) -> Awdch1chR {
        Awdch1chR::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, CfgrSpec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DmacfgW<'_, CfgrSpec> {
        DmacfgW::new(self, 1)
    }
    #[doc = "Bits 3:4 - RES"]
    #[inline(always)]
    pub fn res(&mut self) -> ResW<'_, CfgrSpec> {
        ResW::new(self, 3)
    }
    #[doc = "Bit 5 - ALIGN"]
    #[inline(always)]
    pub fn align(&mut self) -> AlignW<'_, CfgrSpec> {
        AlignW::new(self, 5)
    }
    #[doc = "Bits 6:9 - EXTSEL"]
    #[inline(always)]
    pub fn extsel(&mut self) -> ExtselW<'_, CfgrSpec> {
        ExtselW::new(self, 6)
    }
    #[doc = "Bits 10:11 - EXTEN"]
    #[inline(always)]
    pub fn exten(&mut self) -> ExtenW<'_, CfgrSpec> {
        ExtenW::new(self, 10)
    }
    #[doc = "Bit 12 - OVRMOD"]
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OvrmodW<'_, CfgrSpec> {
        OvrmodW::new(self, 12)
    }
    #[doc = "Bit 13 - CONT"]
    #[inline(always)]
    pub fn cont(&mut self) -> ContW<'_, CfgrSpec> {
        ContW::new(self, 13)
    }
    #[doc = "Bit 14 - AUTDLY"]
    #[inline(always)]
    pub fn autdly(&mut self) -> AutdlyW<'_, CfgrSpec> {
        AutdlyW::new(self, 14)
    }
    #[doc = "Bit 16 - DISCEN"]
    #[inline(always)]
    pub fn discen(&mut self) -> DiscenW<'_, CfgrSpec> {
        DiscenW::new(self, 16)
    }
    #[doc = "Bits 17:19 - DISCNUM"]
    #[inline(always)]
    pub fn discnum(&mut self) -> DiscnumW<'_, CfgrSpec> {
        DiscnumW::new(self, 17)
    }
    #[doc = "Bit 20 - JDISCEN"]
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JdiscenW<'_, CfgrSpec> {
        JdiscenW::new(self, 20)
    }
    #[doc = "Bit 21 - JQM"]
    #[inline(always)]
    pub fn jqm(&mut self) -> JqmW<'_, CfgrSpec> {
        JqmW::new(self, 21)
    }
    #[doc = "Bit 22 - AWD1SGL"]
    #[inline(always)]
    pub fn awd1sgl(&mut self) -> Awd1sglW<'_, CfgrSpec> {
        Awd1sglW::new(self, 22)
    }
    #[doc = "Bit 23 - AWD1EN"]
    #[inline(always)]
    pub fn awd1en(&mut self) -> Awd1enW<'_, CfgrSpec> {
        Awd1enW::new(self, 23)
    }
    #[doc = "Bit 24 - JAWD1EN"]
    #[inline(always)]
    pub fn jawd1en(&mut self) -> Jawd1enW<'_, CfgrSpec> {
        Jawd1enW::new(self, 24)
    }
    #[doc = "Bit 25 - JAUTO"]
    #[inline(always)]
    pub fn jauto(&mut self) -> JautoW<'_, CfgrSpec> {
        JautoW::new(self, 25)
    }
    #[doc = "Bits 26:30 - AWDCH1CH"]
    #[inline(always)]
    pub fn awdch1ch(&mut self) -> Awdch1chW<'_, CfgrSpec> {
        Awdch1chW::new(self, 26)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {}
