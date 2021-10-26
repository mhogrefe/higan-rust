namespace registers {

void TestAF() {
  {
    ::ares::GameBoy::CPU processor;
    processor.AF = 123;
    EXPECT_EQ("Registers AF", processor.AF, (n16)123);
  }
  {
    ::ares::GameBoy::CPU processor;
    processor.AF = 0x12ab;
    EXPECT_EQ("Registers AF", processor.A, (n8)0x12);
    EXPECT_EQ("Registers AF", processor.F, (n8)0xab);

    processor.A = 0x34;
    EXPECT_EQ("Registers AF", processor.A, (n8)0x34);
    EXPECT_EQ("Registers AF", processor.F, (n8)0xab);
    EXPECT_EQ("Registers AF", processor.AF, (n16)0x34ab);

    processor.F = 0xcd;
    EXPECT_EQ("Registers AF", processor.A, (n8)0x34);
    EXPECT_EQ("Registers AF", processor.F, (n8)0xcd);
    EXPECT_EQ("Registers AF", processor.AF, (n16)0x34cd);
  }
}

void TestBC() {
  {
    ::ares::GameBoy::CPU processor;
    processor.BC = 123;
    EXPECT_EQ("Registers BC", processor.BC, (n16)123);
  }
  {
    ::ares::GameBoy::CPU processor;
    processor.BC = 0x12ab;
    EXPECT_EQ("Registers BC", processor.B, (n8)0x12);
    EXPECT_EQ("Registers BC", processor.C, (n8)0xab);

    processor.B = 0x34;
    EXPECT_EQ("Registers BC", processor.B, (n8)0x34);
    EXPECT_EQ("Registers BC", processor.C, (n8)0xab);
    EXPECT_EQ("Registers BC", processor.BC, (n16)0x34ab);

    processor.C = 0xcd;
    EXPECT_EQ("Registers BC", processor.B, (n8)0x34);
    EXPECT_EQ("Registers BC", processor.C, (n8)0xcd);
    EXPECT_EQ("Registers BC", processor.BC, (n16)0x34cd);
  }
}

void TestDE() {
  {
    ::ares::GameBoy::CPU processor;
    processor.DE = 123;
    EXPECT_EQ("Registers DE", processor.DE, (n16)123);
  }
  {
    ::ares::GameBoy::CPU processor;
    processor.DE = 0x12ab;
    EXPECT_EQ("Registers DE", processor.D, (n8)0x12);
    EXPECT_EQ("Registers DE", processor.E, (n8)0xab);

    processor.D = 0x34;
    EXPECT_EQ("Registers DE", processor.D, (n8)0x34);
    EXPECT_EQ("Registers DE", processor.E, (n8)0xab);
    EXPECT_EQ("Registers DE", processor.DE, (n16)0x34ab);

    processor.E = 0xcd;
    EXPECT_EQ("Registers DE", processor.D, (n8)0x34);
    EXPECT_EQ("Registers DE", processor.E, (n8)0xcd);
    EXPECT_EQ("Registers DE", processor.DE, (n16)0x34cd);
  }
}

void TestHL() {
  {
    ::ares::GameBoy::CPU processor;
    processor.HL = 123;
    EXPECT_EQ("Registers HL", processor.HL, (n16)123);
  }
  {
    ::ares::GameBoy::CPU processor;
    processor.HL = 0x12ab;
    EXPECT_EQ("Registers HL", processor.H, (n8)0x12);
    EXPECT_EQ("Registers HL", processor.L, (n8)0xab);

    processor.H = 0x34;
    EXPECT_EQ("Registers HL", processor.H, (n8)0x34);
    EXPECT_EQ("Registers HL", processor.L, (n8)0xab);
    EXPECT_EQ("Registers HL", processor.HL, (n16)0x34ab);

    processor.L = 0xcd;
    EXPECT_EQ("Registers HL", processor.H, (n8)0x34);
    EXPECT_EQ("Registers HL", processor.L, (n8)0xcd);
    EXPECT_EQ("Registers HL", processor.HL, (n16)0x34cd);
  }
}

void TestSP() {
  ::ares::GameBoy::CPU processor;
  processor.SP = 123;
  EXPECT_EQ("Registers SP", processor.SP, (n16)123);

  EXPECT_EQ("Registers SP", (int)processor.SP++, 123);
  EXPECT_EQ("Registers SP", processor.SP, (n16)124);

  processor.SP = 65535;
  EXPECT_EQ("Registers SP", (int)processor.SP++, 65535);
  EXPECT_EQ("Registers SP", processor.SP, (n16)0);

  processor.SP = 123;
  EXPECT_EQ("Registers SP", (int)--processor.SP, 122);

  processor.SP = 0;
  EXPECT_EQ("Registers SP", (int)--processor.SP, 65535);
}

void TestPC() {
  ::ares::GameBoy::CPU processor;
  processor.PC = 123;
  EXPECT_EQ("Registers PC", processor.PC, (n16)123);

  EXPECT_EQ("Registers PC", (int)processor.PC++, 123);
  EXPECT_EQ("Registers PC", processor.PC, (n16)124);

  processor.PC = 65535;
  EXPECT_EQ("Registers PC", (int)processor.PC++, 65535);
  EXPECT_EQ("Registers PC", processor.PC, (n16)0);
}

void TestCF() {
  ::ares::GameBoy::CPU processor;
  processor.F = 0b10100101;
  EXPECT_FALSE("Registers CF", processor.CF);
  processor.CF = true;
  EXPECT_EQ("Registers CF", processor.F, (n8)0b10110101);
}

void TestHF() {
  ::ares::GameBoy::CPU processor;
  processor.F = 0b10100101;
  EXPECT_TRUE("Registers HF", processor.HF);
  processor.HF = false;
  EXPECT_EQ("Registers HF", processor.F, (n8)0b10000101);
}

void TestNF() {
  ::ares::GameBoy::CPU processor;
  processor.F = 0b10100101;
  EXPECT_FALSE("Registers NF", processor.NF);
  processor.NF = true;
  EXPECT_EQ("Registers NF", processor.F, (n8)0b11100101);
}

void TestZF() {
  ::ares::GameBoy::CPU processor;
  processor.F = 0b10100101;
  EXPECT_TRUE("Registers ZF", processor.ZF);
  processor.ZF = false;
  EXPECT_EQ("Registers ZF", processor.F, (n8)0b00100101);
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
} // namespace registers
