auto Interrupt::readByte(u32 address) -> u32 {
  return readWord(address & ~3) >> 8 * (address & 3);
}

auto Interrupt::readHalf(u32 address) -> u32 {
  return readWord(address & ~3) >> 8 * (address & 3);
}

auto Interrupt::readWord(u32 address) -> u32 {
  n32 data;

  //I_STAT
  if(address == 0x1f80'1070) {
    data.bit( 0) = vblank.stat;
    data.bit( 1) = gpu.stat;
    data.bit( 2) = cdrom.stat;
    data.bit( 3) = dma.stat;
    data.bit( 4) = timer0.stat;
    data.bit( 5) = timer1.stat;
    data.bit( 6) = timer2.stat;
    data.bit( 7) = peripheral.stat;
    data.bit( 8) = sio.stat;
    data.bit( 9) = spu.stat;
    data.bit(10) = pio.stat;
  }

  //I_MASK
  if(address == 0x1f80'1074) {
    data.bit( 0) = vblank.mask;
    data.bit( 1) = gpu.mask;
    data.bit( 2) = cdrom.mask;
    data.bit( 3) = dma.mask;
    data.bit( 4) = timer0.mask;
    data.bit( 5) = timer1.mask;
    data.bit( 6) = timer2.mask;
    data.bit( 7) = peripheral.mask;
    data.bit( 8) = sio.mask;
    data.bit( 9) = spu.mask;
    data.bit(10) = pio.mask;
  }

  return data;
}

auto Interrupt::writeByte(u32 address, u32 data) -> void {
  return writeWord(address & ~3, data << 8 * (address & 3));
}

auto Interrupt::writeHalf(u32 address, u32 data) -> void {
  return writeWord(address & ~3, data << 8 * (address & 3));
}

auto Interrupt::writeWord(u32 address, u32 value) -> void {
  n32 data = value;

  //I_STAT
  if(address == 0x1f80'1070) {
    if(!data.bit( 0)) vblank.acknowledge();
    if(!data.bit( 1)) gpu.acknowledge();
    if(!data.bit( 2)) cdrom.acknowledge();
    if(!data.bit( 3)) dma.acknowledge();
    if(!data.bit( 4)) timer0.acknowledge();
    if(!data.bit( 5)) timer1.acknowledge();
    if(!data.bit( 6)) timer2.acknowledge();
    if(!data.bit( 7)) peripheral.acknowledge();
    if(!data.bit( 8)) sio.acknowledge();
    if(!data.bit( 9)) spu.acknowledge();
    if(!data.bit(10)) pio.acknowledge();
    poll();
  }

  //I_MASK
  if(address == 0x1f80'1074) {
    vblank.mask     = data.bit( 0);
    gpu.mask        = data.bit( 1);
    cdrom.mask      = data.bit( 2);
    dma.mask        = data.bit( 3);
    timer0.mask     = data.bit( 4);
    timer1.mask     = data.bit( 5);
    timer2.mask     = data.bit( 6);
    peripheral.mask = data.bit( 7);
    sio.mask        = data.bit( 8);
    spu.mask        = data.bit( 9);
    pio.mask        = data.bit(10);
    poll();
  }
}
