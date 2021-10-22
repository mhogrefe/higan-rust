auto SPU::readRAM(u32 address) -> u16 {
  if(irq.enable && irq.address == u16(address >> 3)) {
    irq.flag = 1;
    interrupt.raise(Interrupt::SPU);
  }
  return ram.readHalf(address);
}

auto SPU::writeRAM(u32 address, u16 data) -> void {
  if(irq.enable && irq.address == u16(address >> 3)) {
    irq.flag = 1;
    interrupt.raise(Interrupt::SPU);
  }
  return ram.writeHalf(address, data);
}

auto SPU::readDMA() -> u32 {
  if(fifo.empty()) fifoReadBlock();
  n32    data = fifo.read(0) <<  0;
  return data | fifo.read(0) << 16;
}

auto SPU::writeDMA(u32 data) -> void {
  fifo.write(data >>  0);
  fifo.write(data >> 16);
  if(fifo.size() >= 8) fifoWriteBlock();
}

auto SPU::readByte(u32 address) -> u32 {
  n16    data = readHalf(address & ~1);
  return data >> 8 * (address & 1) & 0xff;
}

auto SPU::readHalf(u32 address) -> u32 {
  n16 data = 0;

  u8 v = address >> 4 & 31;

  //volume left
  if((address & 0xffff'fe0f) == 0x1f80'1c00 && v < 24) {
    data.bit( 0,14) = voice[v].volume[0].level;
    data.bit(15)    = voice[v].volume[0].sweep;
  }

  //volume right
  if((address & 0xffff'fe0f) == 0x1f80'1c02 && v < 24) {
    data.bit( 0,14) = voice[v].volume[1].level;
    data.bit(15)    = voice[v].volume[1].sweep;
  }

  //ADPCM sample rate
  if((address & 0xffff'fe0f) == 0x1f80'1c04 && v < 24) {
    data.bit(0,15) = voice[v].adpcm.sampleRate;
  }

  //ADPCM start address
  if((address & 0xffff'fe0f) == 0x1f80'1c06 && v < 24) {
    data.bit(0,15) = voice[v].adpcm.startAddress >> 3;
  }

  //ADSR
  if((address & 0xffff'fe0f) == 0x1f80'1c08 && v < 24) {
    data.bit( 0, 3) = voice[v].sustain.level;
    data.bit( 4, 7) = voice[v].decay.rate;
    data.bit( 8,14) = voice[v].attack.rate;
    data.bit(15)    = voice[v].attack.exponential;
  }

  //ADSR
  if((address & 0xffff'fe0f) == 0x1f80'1c0a && v < 24) {
    data.bit( 0, 4) = voice[v].release.rate;
    data.bit( 5)    = voice[v].release.exponential;
    data.bit( 6,12) = voice[v].sustain.rate;
    data.bit(13)    = voice[v].sustain.unknown;
    data.bit(14)    = voice[v].sustain.decrease;
    data.bit(15)    = voice[v].sustain.exponential;
  }

  //current ADSR volume
  if((address & 0xffff'fe0f) == 0x1f80'1c0c && v < 24) {
    data.bit(0,15) = voice[v].adsr.volume;
  }

  //ADPCM repeat address
  if((address & 0xffff'fe0f) == 0x1f80'1c0e && v < 24) {
    data.bit(0,15) = voice[v].adpcm.repeatAddress >> 3;
  }

  //main volume left
  if(address == 0x1f80'1d80) {
    data.bit( 0,14) = volume[0].level;
    data.bit(15)    = volume[0].sweep;
  }

  //main volume right
  if(address == 0x1f80'1d82) {
    data.bit( 0,14) = volume[1].level;
    data.bit(15)    = volume[1].sweep;
  }

  //vLOUT
  if(address == 0x1f80'1d84) {
    data.bit(0,15) = reverb.vLOUT;
  }

  //vROUT
  if(address == 0x1f80'1d86) {
    data.bit(0,15) = reverb.vROUT;
  }

  //KON
  if(address == 0x1f80'1d88) {
    for(u32 v : range(16)) data.bit(v) = voice[v].kon;
  }
  if(address == 0x1f80'1d8a) {
    for(u32 v : range( 8)) data.bit(v) = voice[v + 16].kon;
  }

  //KOFF
  if(address == 0x1f80'1d8c) {
    for(u32 v : range(16)) data.bit(v) = voice[v].koff;
  }
  if(address == 0x1f80'1d8e) {
    for(u32 v : range( 8)) data.bit(v) = voice[v + 16].koff;
  }

  //PMON
  if(address == 0x1f80'1d90) {
    for(u32 v : range(16)) data.bit(v) = voice[v].pmon;
  }
  if(address == 0x1f80'1d92) {
    for(u32 v : range( 8)) data.bit(v) = voice[v + 16].pmon;
  }

  //NON
  if(address == 0x1f80'1d94) {
    for(u32 v : range(16)) data.bit(v) = voice[v].non;
  }
  if(address == 0x1f80'1d96) {
    for(u32 v : range( 8)) data.bit(v) = voice[v + 16].non;
  }

  //EON
  if(address == 0x1f80'1d98) {
    for(u32 v : range(16)) data.bit(v) = voice[v].eon;
  }
  if(address == 0x1f80'1d9a) {
    for(u32 v : range( 8)) data.bit(v) = voice[v + 16].eon;
  }

  //ENDX
  if(address == 0x1f80'1d9c) {
    for(u32 v : range(16)) data.bit(v) = voice[v].endx;
  }
  if(address == 0x1f80'1d9e) {
    for(u32 v : range( 8)) data.bit(v) = voice[v + 16].endx;
  }

  //mBASE
  if(address == 0x1f80'1da2) {
    data.bit(0,15) = reverb.mBASE;
  }

  //RAM IRQ address
  if(address == 0x1f80'1da4) {
    data.bit(0,15) = irq.address;
  }

  //RAM transfer address
  if(address == 0x1f80'1da6) {
    data.bit(0,15) = transfer.address;
  }

  //RAM transfer data
  if(address == 0x1f80'1da8) {
    //write-only(?)
  }

  //SPUCNT
  if(address == 0x1f80'1daa) {
    data.bit( 0)    = cdaudio.enable;
    data.bit( 1)    = external.enable;
    data.bit( 2)    = cdaudio.reverb;
    data.bit( 3)    = external.reverb;
    data.bit( 4, 5) = transfer.mode;
    data.bit( 6)    = irq.enable;
    data.bit( 7)    = reverb.enable;
    data.bit( 8, 9) = noise.step;
    data.bit(10,13) = noise.shift;
    data.bit(14)    = master.unmute;
    data.bit(15)    = master.enable;
  }

  //SPURAMCNT
  if(address == 0x1f80'1dac) {
    data.bit(0)    = transfer.unknown_0;
    data.bit(1,3)  = transfer.type;
    data.bit(4,15) = transfer.unknown_4_15;
  }

  //SPUSTAT
  if(address == 0x1f80'1dae) {
    data.bit( 0)    = cdaudio.enable;
    data.bit( 1)    = external.enable;
    data.bit( 2)    = cdaudio.reverb;
    data.bit( 3)    = external.reverb;
    data.bit( 4, 5) = transfer.mode;
    data.bit( 6)    = irq.flag;
    data.bit( 7)    = transfer.mode.bit(1);
    data.bit( 8)    = 0;  //DMA read request
    data.bit( 9)    = 0;  //DMA write request
    data.bit(10)    = 0;  //transfer busy
    data.bit(11)    = capture.address.bit(9);  //in second-half of capture buffer
  }

  //CD audio volume left
  if(address == 0x1f80'1db0) {
    data.bit(0,15) = cdaudio.volume[0];
  }

  //CD audio volume right
  if(address == 0x1f80'1db2) {
    data.bit(0,15) = cdaudio.volume[1];
  }

  //external volume left
  if(address == 0x1f80'1db4) {
    data.bit(0,15) = external.volume[0];
  }

  //external volume right
  if(address == 0x1f80'1db6) {
    data.bit(0,15) = external.volume[1];
  }

  //current volume left
  if(address == 0x1f80'1db8) {
    data.bit(0,15) = current.volume[0];
  }

  //current volume right
  if(address == 0x1f80'1dba) {
    data.bit(0,15) = current.volume[1];
  }

  //dAPF1
  if(address == 0x1f80'1dc0) {
    data.bit(0,15) = reverb.FB_SRC_A;
  }

  //dAPF2
  if(address == 0x1f80'1dc2) {
    data.bit(0,15) = reverb.FB_SRC_B;
  }

  //vIIR
  if(address == 0x1f80'1dc4) {
    data.bit(0,15) = reverb.IIR_ALPHA;
  }

  //vCOMB1
  if(address == 0x1f80'1dc6) {
    data.bit(0,15) = reverb.ACC_COEF_A;
  }

  //vCOMB2
  if(address == 0x1f80'1dc8) {
    data.bit(0,15) = reverb.ACC_COEF_B;
  }

  //vCOMB3
  if(address == 0x1f80'1dca) {
    data.bit(0,15) = reverb.ACC_COEF_C;
  }

  //vCOMB4
  if(address == 0x1f80'1dcc) {
    data.bit(0,15) = reverb.ACC_COEF_D;
  }

  //vWALL
  if(address == 0x1f80'1dce) {
    data.bit(0,15) = reverb.IIR_COEF;
  }

  //vAPF1
  if(address == 0x1f80'1dd0) {
    data.bit(0,15) = reverb.FB_ALPHA;
  }

  //vAPF2
  if(address == 0x1f80'1dd2) {
    data.bit(0,15) = reverb.FB_X;
  }

  //mLSAME
  if(address == 0x1f80'1dd4) {
    data.bit(0,15) = reverb.IIR_DEST_A0;
  }

  //mRSAME
  if(address == 0x1f80'1dd6) {
    data.bit(0,15) = reverb.IIR_DEST_A1;
  }

  //mLCOMB1
  if(address == 0x1f80'1dd8) {
    data.bit(0,15) = reverb.ACC_SRC_A0;
  }

  //mRCOMB1
  if(address == 0x1f80'1dda) {
    data.bit(0,15) = reverb.ACC_SRC_A1;
  }

  //mLCOMB2
  if(address == 0x1f80'1ddc) {
    data.bit(0,15) = reverb.ACC_SRC_B0;
  }

  //mRCOMB2
  if(address == 0x1f80'1dde) {
    data.bit(0,15) = reverb.ACC_SRC_B1;
  }

  //dLSAME
  if(address == 0x1f80'1de0) {
    data.bit(0,15) = reverb.IIR_SRC_A0;
  }

  //dRSAME
  if(address == 0x1f80'1de2) {
    data.bit(0,15) = reverb.IIR_SRC_A1;
  }

  //mLDIFF
  if(address == 0x1f80'1de4) {
    data.bit(0,15) = reverb.IIR_DEST_B0;
  }

  //mRDIFF
  if(address == 0x1f80'1de6) {
    data.bit(0,15) = reverb.IIR_DEST_B1;
  }

  //mLCOMB3
  if(address == 0x1f80'1de8) {
    data.bit(0,15) = reverb.ACC_SRC_C0;
  }

  //mRCOMB3
  if(address == 0x1f80'1dea) {
    data.bit(0,15) = reverb.ACC_SRC_C1;
  }

  //mLCOMB4
  if(address == 0x1f80'1dec) {
    data.bit(0,15) = reverb.ACC_SRC_D0;
  }

  //mRCOMB4
  if(address == 0x1f80'1dee) {
    data.bit(0,15) = reverb.ACC_SRC_D1;
  }

  //dLDIFF
  if(address == 0x1f80'1df0) {
    data.bit(0,15) = reverb.IIR_SRC_B1;  //misordered
  }

  //dRDIFF
  if(address == 0x1f80'1df2) {
    data.bit(0,15) = reverb.IIR_SRC_B0;  //misordered
  }

  //mLAPF1
  if(address == 0x1f80'1df4) {
    data.bit(0,15) = reverb.MIX_DEST_A0;
  }

  //mRAPF1
  if(address == 0x1f80'1df6) {
    data.bit(0,15) = reverb.MIX_DEST_A1;
  }

  //mLAPF2
  if(address == 0x1f80'1df8) {
    data.bit(0,15) = reverb.MIX_DEST_B0;
  }

  //mRAPF2
  if(address == 0x1f80'1dfa) {
    data.bit(0,15) = reverb.MIX_DEST_B1;
  }

  //vLIN
  if(address == 0x1f80'1dfc) {
    data.bit(0,15) = reverb.IN_COEF_L;
  }

  //vRIN
  if(address == 0x1f80'1dfe) {
    data.bit(0,15) = reverb.IN_COEF_R;
  }

  v = address >> 2 & 31;

  //current volume left
  if((address & 0xffff'ff83) == 0x1f80'1e00) {
    data.bit(0,15) = voice[v].current.volume[0];
  }

  //current volume right
  if((address & 0xffff'ff83) == 0x1f80'1e02) {
    data.bit(0,15) = voice[v].current.volume[1];
  }

  return data;
}

auto SPU::readWord(u32 address) -> u32 {
  n32    data = readHalf(address & ~3 | 0) <<  0;
  return data | readHalf(address & ~3 | 2) << 16;
}

auto SPU::writeByte(u32 address, u32 value) -> void {
  if(address & 1) return;  //odd address writes are ignored
  writeHalf(address & ~1, value);
}

auto SPU::writeHalf(u32 address, u32 value) -> void {
  n16 data = value;

  u8 v = address >> 4 & 31;

  //volume left
  if((address & 0xffff'fe0f) == 0x1f80'1c00 && v < 24) {
    //normal
    voice[v].volume[0].level       = data.bit( 0,14);
    //sweep
    voice[v].volume[0].rate        = data.bit( 0, 6);
    voice[v].volume[0].negative    = data.bit(12);
    voice[v].volume[0].decreasing  = data.bit(13);
    voice[v].volume[0].exponential = data.bit(14);
    //mode
    voice[v].volume[0].sweep       = data.bit(15);
    voice[v].volume[0].reset();
  }

  //volume right
  if((address & 0xffff'fe0f) == 0x1f80'1c02 && v < 24) {
    //normal
    voice[v].volume[1].level       = data.bit( 0,14);
    //sweep
    voice[v].volume[1].rate        = data.bit( 0, 6);
    voice[v].volume[1].negative    = data.bit(12);
    voice[v].volume[1].decreasing  = data.bit(13);
    voice[v].volume[1].exponential = data.bit(14);
    //mode
    voice[v].volume[1].sweep       = data.bit(15);
    voice[v].volume[1].reset();
  }

  //ADPCM sample rate
  if((address & 0xffff'fe0f) == 0x1f80'1c04 && v < 24) {
    voice[v].adpcm.sampleRate = data.bit(0,15);
  }

  //ADPCM start address
  if((address & 0xffff'fe0f) == 0x1f80'1c06 && v < 24) {
    voice[v].adpcm.startAddress = data.bit(0,15) << 3;
  }

  //ADSR
  if((address & 0xffff'fe0f) == 0x1f80'1c08 && v < 24) {
    voice[v].sustain.level      = data.bit( 0, 3);
    voice[v].decay.rate         = data.bit( 4, 7);
    voice[v].attack.rate        = data.bit( 8,14);
    voice[v].attack.exponential = data.bit(15);
  }

  //ADSR
  if((address & 0xffff'fe0f) == 0x1f80'1c0a && v < 24) {
    voice[v].release.rate        = data.bit( 0, 4);
    voice[v].release.exponential = data.bit( 5);
    voice[v].sustain.rate        = data.bit( 6,12);
    voice[v].sustain.unknown     = data.bit(13);
    voice[v].sustain.decrease    = data.bit(14);
    voice[v].sustain.exponential = data.bit(15);
  }

  //current ADSR volume
  if((address & 0xffff'fe0f) == 0x1f80'1c0c && v < 24) {
    voice[v].adsr.volume = data.bit(0,15);
  }

  //ADPCM repeat address
  if((address & 0xffff'fe0f) == 0x1f80'1c0e && v < 24) {
    voice[v].adpcm.repeatAddress = data.bit(0,15) << 3;
    voice[v].adpcm.ignoreLoopAddress = 1;
  }

  //main volume left
  if(address == 0x1f80'1d80) {
    //normal
    volume[0].level       = data.bit( 0,14);
    //sweep
    volume[0].rate        = data.bit( 0, 6);
    volume[0].negative    = data.bit(12);
    volume[0].decreasing  = data.bit(13);
    volume[0].exponential = data.bit(14);
    //mode
    volume[0].sweep       = data.bit(15);
    volume[0].reset();
  }

  //main volume right
  if(address == 0x1f80'1d82) {
    //normal
    volume[1].level       = data.bit( 0,14);
    //sweep
    volume[1].rate        = data.bit( 0, 6);
    volume[1].negative    = data.bit(12);
    volume[1].decreasing  = data.bit(13);
    volume[1].exponential = data.bit(14);
    //mode
    volume[1].sweep       = data.bit(15);
    volume[1].reset();
  }

  //vLOUT
  if(address == 0x1f80'1d84) {
    reverb.vLOUT = data.bit(0,15);
  }

  //vROUT
  if(address == 0x1f80'1d86) {
    reverb.vROUT = data.bit(0,15);
  }

  //KON
  if(address == 0x1f80'1d88) {
    for(u32 v : range(16)) voice[v].kon = data.bit(v);
  }
  if(address == 0x1f80'1d8a) {
    for(u32 v : range( 8)) voice[v + 16].kon = data.bit(v);
  }

  //KOFF
  if(address == 0x1f80'1d8c) {
    for(u32 v : range(16)) voice[v].koff = data.bit(v);
  }
  if(address == 0x1f80'1d8e) {
    for(u32 v : range( 8)) voice[v + 16].koff = data.bit(v);
  }

  //PMON
  if(address == 0x1f80'1d90) {
    for(u32 v : range(16)) voice[v].pmon = data.bit(v);
  }
  if(address == 0x1f80'1d92) {
    for(u32 v : range( 8)) voice[v + 16].pmon = data.bit(v);
  }

  //NON
  if(address == 0x1f80'1d94) {
    for(u32 v : range(16)) voice[v].non = data.bit(v);
  }
  if(address == 0x1f80'1d96) {
    for(u32 v : range( 8)) voice[v + 16].non = data.bit(v);
  }

  //EON
  if(address == 0x1f80'1d98) {
    for(u32 v : range(16)) voice[v].eon = data.bit(v);
  }
  if(address == 0x1f80'1d9a) {
    for(u32 v : range( 8)) voice[v + 16].eon = data.bit(v);
  }

  //ENDX
  if(address == 0x1f80'1d9c) {
    for(u32 v : range(16)) voice[v].endx = data.bit(v);
  }
  if(address == 0x1f80'1d9e) {
    for(u32 v : range( 8)) voice[v + 16].endx = data.bit(v);
  }

  //mBASE
  if(address == 0x1f80'1da2) {
    reverb.mBASE = data.bit(0,15);
    reverb.baseAddress = reverb.mBASE << 2;
    reverb.currentAddress = reverb.mBASE << 2;
  }

  //RAM IRQ address
  if(address == 0x1f80'1da4) {
    irq.address = data.bit(0,15);
  }

  //RAM transfer address
  if(address == 0x1f80'1da6) {
    transfer.address = data.bit(0,15);
    transfer.current = transfer.address << 3;
  }

  //RAM transfer data
  if(address == 0x1f80'1da8) {
    if(!fifo.full()) fifo.write(data);
  }

  //SPUCNT
  if(address == 0x1f80'1daa) {
    cdaudio.enable  = data.bit( 0);
    external.enable = data.bit( 1);
    cdaudio.reverb  = data.bit( 2);
    external.reverb = data.bit( 3);
    transfer.mode   = data.bit( 4, 5);
    irq.enable      = data.bit( 6);
    reverb.enable   = data.bit( 7);
    noise.step      = data.bit( 8, 9);
    noise.shift     = data.bit(10,13);
    master.unmute   = data.bit(14);
    master.enable   = data.bit(15);

    if(transfer.mode == 0) {
      fifo.flush();
    }

    if(transfer.mode == 1) {
      while(fifo.size() >= 8) fifoWriteBlock();
      if(!fifo.empty()) debug(unhandled, "SPU::writeHalf: FIFO not empty");
    }

    if(irq.enable == 0) {
      irq.flag = 0;
      interrupt.lower(Interrupt::SPU);
    }
  }

  //SPURAMCNT
  if(address == 0x1f80'1dac) {
    transfer.unknown_0    = data.bit(0);
    transfer.type         = data.bit(1, 3);
    transfer.unknown_4_15 = data.bit(4,15);
  }

  //SPUSTAT
  if(address == 0x1f80'1dae) {
    //read-only
  }

  //CD audio volume left
  if(address == 0x1f80'1db0) {
    cdaudio.volume[0] = data.bit(0,15);
  }

  //CD audio volume right
  if(address == 0x1f80'1db2) {
    cdaudio.volume[1] = data.bit(0,15);
  }

  //external volume left
  if(address == 0x1f80'1db4) {
    external.volume[0] = data.bit(0,15);
  }

  //external volume right
  if(address == 0x1f80'1db6) {
    external.volume[1] = data.bit(0,15);
  }

  //current volume left
  if(address == 0x1f80'1db8) {
    current.volume[0] = data.bit(0,15);
  }

  //current volume right
  if(address == 0x1f80'1dba) {
    current.volume[1] = data.bit(0,15);
  }

  //dAPF1
  if(address == 0x1f80'1dc0) {
    reverb.FB_SRC_A = data.bit(0,15);
  }

  //dAPF2
  if(address == 0x1f80'1dc2) {
    reverb.FB_SRC_B = data.bit(0,15);
  }

  //vIIR
  if(address == 0x1f80'1dc4) {
    reverb.IIR_ALPHA = data.bit(0,15);
  }

  //vCOMB1
  if(address == 0x1f80'1dc6) {
    reverb.ACC_COEF_A = data.bit(0,15);
  }

  //vCOMB2
  if(address == 0x1f80'1dc8) {
    reverb.ACC_COEF_B = data.bit(0,15);
  }

  //vCOMB3
  if(address == 0x1f80'1dca) {
    reverb.ACC_COEF_C = data.bit(0,15);
  }

  //vCOMB4
  if(address == 0x1f80'1dcc) {
    reverb.ACC_COEF_D = data.bit(0,15);
  }

  //vWALL
  if(address == 0x1f80'1dce) {
    reverb.IIR_COEF = data.bit(0,15);
  }

  //vAPF1
  if(address == 0x1f80'1dd0) {
    reverb.FB_ALPHA = data.bit(0,15);
  }

  //vAPF2
  if(address == 0x1f80'1dd2) {
    reverb.FB_X = data.bit(0,15);
  }

  //mLSAME
  if(address == 0x1f80'1dd4) {
    reverb.IIR_DEST_A0 = data.bit(0,15);
  }

  //mRSAME
  if(address == 0x1f80'1dd6) {
    reverb.IIR_DEST_A1 = data.bit(0,15);
  }

  //mLCOMB1
  if(address == 0x1f80'1dd8) {
    reverb.ACC_SRC_A0 = data.bit(0,15);
  }

  //mRCOMB1
  if(address == 0x1f80'1dda) {
    reverb.ACC_SRC_A1 = data.bit(0,15);
  }

  //mLCOMB2
  if(address == 0x1f80'1ddc) {
    reverb.ACC_SRC_B0 = data.bit(0,15);
  }

  //mRCOMB2
  if(address == 0x1f80'1dde) {
    reverb.ACC_SRC_B1 = data.bit(0,15);
  }

  //dLSAME
  if(address == 0x1f80'1de0) {
    reverb.IIR_SRC_A0 = data.bit(0,15);
  }

  //dRSAME
  if(address == 0x1f80'1de2) {
    reverb.IIR_SRC_A1 = data.bit(0,15);
  }

  //mLDIFF
  if(address == 0x1f80'1de4) {
    reverb.IIR_DEST_B0 = data.bit(0,15);
  }

  //mRDIFF
  if(address == 0x1f80'1de6) {
    reverb.IIR_DEST_B1 = data.bit(0,15);
  }

  //mLCOMB3
  if(address == 0x1f80'1de8) {
    reverb.ACC_SRC_C0 = data.bit(0,15);
  }

  //mRCOMB3
  if(address == 0x1f80'1dea) {
    reverb.ACC_SRC_C1 = data.bit(0,15);
  }

  //mLCOMB4
  if(address == 0x1f80'1dec) {
    reverb.ACC_SRC_D0 = data.bit(0,15);
  }

  //mRCOMB4
  if(address == 0x1f80'1dee) {
    reverb.ACC_SRC_D1 = data.bit(0,15);
  }

  //dLDIFF
  if(address == 0x1f80'1df0) {
    reverb.IIR_SRC_B1 = data.bit(0,15);  //misordered
  }

  //dRDIFF
  if(address == 0x1f80'1df2) {
    reverb.IIR_SRC_B0 = data.bit(0,15);  //misordered
  }

  //mLAPF1
  if(address == 0x1f80'1df4) {
    reverb.MIX_DEST_A0 = data.bit(0,15);
  }

  //mRAPF1
  if(address == 0x1f80'1df6) {
    reverb.MIX_DEST_A1 = data.bit(0,15);
  }

  //mLAPF2
  if(address == 0x1f80'1df8) {
    reverb.MIX_DEST_B0 = data.bit(0,15);
  }

  //mRAPF2
  if(address == 0x1f80'1dfa) {
    reverb.MIX_DEST_B1 = data.bit(0,15);
  }

  //vLIN
  if(address == 0x1f80'1dfc) {
    reverb.IN_COEF_L = data.bit(0,15);
  }

  //vRIN
  if(address == 0x1f80'1dfe) {
    reverb.IN_COEF_R = data.bit(0,15);
  }

  v = address >> 2 & 31;

  //current volume left
  if((address & 0xffff'ff83) == 0x1f80'1e00 && v < 24) {
    voice[v].current.volume[0] = data.bit(0,15);
  }

  //current volume right
  if((address & 0xffff'ff83) == 0x1f80'1e02 && v < 24) {
    voice[v].current.volume[1] = data.bit(0,15);
  }
}

auto SPU::writeWord(u32 address, u32 data) -> void {
  writeHalf(address & ~3 | 0, data >>  0);
  writeHalf(address & ~3 | 2, data >> 16);
}
