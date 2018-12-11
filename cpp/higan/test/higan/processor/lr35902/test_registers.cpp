namespace registers {

void TestAF() {
  {
    GameBoy::CPU processor;
    processor.AF = 123;
    EXPECT_EQ("Registers AF", processor.AF, (uint16)123);
  }
  {
    GameBoy::CPU processor;
    processor.AF = 0x12ab;
    EXPECT_EQ("Registers AF", processor.A, (uint8)0x12);
    EXPECT_EQ("Registers AF", processor.F, (uint8)0xab);

    processor.A = 0x34;
    EXPECT_EQ("Registers AF", processor.A, (uint8)0x34);
    EXPECT_EQ("Registers AF", processor.F, (uint8)0xab);
    EXPECT_EQ("Registers AF", processor.AF, (uint16)0x34ab);

    processor.F = 0xcd;
    EXPECT_EQ("Registers AF", processor.A, (uint8)0x34);
    EXPECT_EQ("Registers AF", processor.F, (uint8)0xcd);
    EXPECT_EQ("Registers AF", processor.AF, (uint16)0x34cd);
  }
}

void TestBC() {
  {
    GameBoy::CPU processor;
    processor.BC = 123;
    EXPECT_EQ("Registers BC", processor.BC, (uint16)123);
  }
  {
    GameBoy::CPU processor;
    processor.BC = 0x12ab;
    EXPECT_EQ("Registers BC", processor.B, (uint8)0x12);
    EXPECT_EQ("Registers BC", processor.C, (uint8)0xab);

    processor.B = 0x34;
    EXPECT_EQ("Registers BC", processor.B, (uint8)0x34);
    EXPECT_EQ("Registers BC", processor.C, (uint8)0xab);
    EXPECT_EQ("Registers BC", processor.BC, (uint16)0x34ab);

    processor.C = 0xcd;
    EXPECT_EQ("Registers BC", processor.B, (uint8)0x34);
    EXPECT_EQ("Registers BC", processor.C, (uint8)0xcd);
    EXPECT_EQ("Registers BC", processor.BC, (uint16)0x34cd);
  }
}

void TestDE() {
  {
    GameBoy::CPU processor;
    processor.DE = 123;
    EXPECT_EQ("Registers DE", processor.DE, (uint16)123);
  }
  {
    GameBoy::CPU processor;
    processor.DE = 0x12ab;
    EXPECT_EQ("Registers DE", processor.D, (uint8)0x12);
    EXPECT_EQ("Registers DE", processor.E, (uint8)0xab);

    processor.D = 0x34;
    EXPECT_EQ("Registers DE", processor.D, (uint8)0x34);
    EXPECT_EQ("Registers DE", processor.E, (uint8)0xab);
    EXPECT_EQ("Registers DE", processor.DE, (uint16)0x34ab);

    processor.E = 0xcd;
    EXPECT_EQ("Registers DE", processor.D, (uint8)0x34);
    EXPECT_EQ("Registers DE", processor.E, (uint8)0xcd);
    EXPECT_EQ("Registers DE", processor.DE, (uint16)0x34cd);
  }
}

void TestHL() {
  {
    GameBoy::CPU processor;
    processor.HL = 123;
    EXPECT_EQ("Registers HL", processor.HL, (uint16)123);
  }
  {
    GameBoy::CPU processor;
    processor.HL = 0x12ab;
    EXPECT_EQ("Registers HL", processor.H, (uint8)0x12);
    EXPECT_EQ("Registers HL", processor.L, (uint8)0xab);

    processor.H = 0x34;
    EXPECT_EQ("Registers HL", processor.H, (uint8)0x34);
    EXPECT_EQ("Registers HL", processor.L, (uint8)0xab);
    EXPECT_EQ("Registers HL", processor.HL, (uint16)0x34ab);

    processor.L = 0xcd;
    EXPECT_EQ("Registers HL", processor.H, (uint8)0x34);
    EXPECT_EQ("Registers HL", processor.L, (uint8)0xcd);
    EXPECT_EQ("Registers HL", processor.HL, (uint16)0x34cd);
  }
}

void TestSP() {
  GameBoy::CPU processor;
  processor.SP = 123;
  EXPECT_EQ("Registers SP", processor.SP, (uint16)123);
}

void TestPC() {
  GameBoy::CPU processor;
  processor.PC = 123;
  EXPECT_EQ("Registers PC", processor.PC, (uint16)123);
}

void TestCF() {
  GameBoy::CPU processor;
  processor.F = 0b10100101;
  EXPECT_FALSE("Registers CF", processor.CF);
  processor.CF = true;
  EXPECT_EQ("Registers CF", processor.F, (uint8)0b10110101);
}

void TestHF() {
  GameBoy::CPU processor;
  processor.F = 0b10100101;
  EXPECT_TRUE("Registers HF", processor.HF);
  processor.HF = false;
  EXPECT_EQ("Registers HF", processor.F, (uint8)0b10000101);
}

void TestNF() {
  GameBoy::CPU processor;
  processor.F = 0b10100101;
  EXPECT_FALSE("Registers NF", processor.NF);
  processor.NF = true;
  EXPECT_EQ("Registers NF", processor.F, (uint8)0b11100101);
}

void TestZF() {
  GameBoy::CPU processor;
  processor.F = 0b10100101;
  EXPECT_TRUE("Registers ZF", processor.ZF);
  processor.ZF = false;
  EXPECT_EQ("Registers ZF", processor.F, (uint8)0b00100101);
}

void TestAll() {
  TestAF();
  TestBC();
  TestDE();
  TestHL();
  TestSP();
  TestPC();
  TestCF();
  TestHF();
}
}
