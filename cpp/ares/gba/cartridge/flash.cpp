//Dev.ID   Size    Blocks  Manufacturer
//======  =====  ========  ============
//0xd4bf   64KB   16x4096  SST
//0x1cc2   64KB   16x4096  Macronix
//0x1b32   64KB   16x4096  Panasonic
//0x3d1f   64KB  512x 128  Atmel
//0x1362  128KB   32x4096  Sanyo
//0x09c2  128KB   32x4096  Macronix

auto Cartridge::FLASH::read(n16 address) -> n8 {
  if(idmode) {
    if(address == 0x0000) return id >> 0;
    if(address == 0x0001) return id >> 8;
    return 0u;
  }

  return data[bank << 16 | address];
}

auto Cartridge::FLASH::write(n16 address, n8 byte) -> void {
  if(bankselect) {
    bankselect = false;
    //bank select is only applicable on 128KB chips
    if(address == 0x0000) bank = byte & (size > 64 * 1024);
    return;
  }

  if(writeselect) {
    //Atmel writes 128 bytes per command; all others write 1 byte per command
    if(id != 0x3d1f || (address & 0x007f) == 0x007f) writeselect = false;
    data[bank << 16 | address] = byte;
    return;
  }

  if(byte == 0xaa && address == 0x5555) { unlockhi = true; return; }
  if(byte == 0x55 && address == 0x2aaa) { unlocklo = true; return; }

  if(unlockhi && unlocklo) {
    unlockhi = false;
    unlocklo = false;

    if(byte == 0x10 && address == 0x5555) {
      if(erasemode) {
        erasemode = false;
        for(u32 n : range(size)) data[n] = 0xff;
      }
    }

    if(byte == 0x30 && (address & 0x0fff) == 0x0000) {
      //command only valid for non-Atmel chips
      if(erasemode && id != 0x3d1f) {
        erasemode = false;
        u32 offset = bank << 16 | (address & ~4095);
        for(u32 n : range(4096)) data[offset++] = 0xff;
      }
    }

    if(byte == 0x80 && address == 0x5555) {
      erasemode = true;
    }

    if(byte == 0x90 && address == 0x5555) {
      idmode = true;
    }

    if(byte == 0xa0 && address == 0x5555) {
      writeselect = true;
    }

    if(byte == 0xb0 && address == 0x5555) {
      bankselect = true;
    }

    if(byte == 0xf0 && address == 0x5555) {
      idmode = false;
    }
  }
}

auto Cartridge::FLASH::power() -> void {
  unlockhi = false;
  unlocklo = false;
  idmode = false;
  bankselect = false;
  writeselect = false;
  bank = 0;
}
