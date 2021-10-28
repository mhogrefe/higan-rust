namespace algorithms {

void TestADD() {
  // CF false, HF false, ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms ADD", processor.ADD(3, 4, false), (n8)7);
    EXPECT_FALSE("Algorithms ADD", processor.CF);
    EXPECT_FALSE("Algorithms ADD", processor.HF);
    EXPECT_FALSE("Algorithms ADD", processor.NF);
    EXPECT_FALSE("Algorithms ADD", processor.ZF);
  }

  // CF false, HF false, ZF true
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms ADD", processor.ADD(0, 0, false), (n8)0);
    EXPECT_FALSE("Algorithms ADD", processor.CF);
    EXPECT_FALSE("Algorithms ADD", processor.HF);
    EXPECT_FALSE("Algorithms ADD", processor.NF);
    EXPECT_TRUE("Algorithms ADD", processor.ZF);
  }

  // CF false, HF true, ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms ADD", processor.ADD(9, 8, false), (n8)17);
    EXPECT_FALSE("Algorithms ADD", processor.CF);
    EXPECT_TRUE("Algorithms ADD", processor.HF);
    EXPECT_FALSE("Algorithms ADD", processor.NF);
    EXPECT_FALSE("Algorithms ADD", processor.ZF);
  }

  // CF false, HF true, ZF true impossible

  // CF true, HF false, ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms ADD", processor.ADD(128, 128, true), (n8)1);
    EXPECT_TRUE("Algorithms ADD", processor.CF);
    EXPECT_FALSE("Algorithms ADD", processor.HF);
    EXPECT_FALSE("Algorithms ADD", processor.NF);
    EXPECT_FALSE("Algorithms ADD", processor.ZF);
  }

  // CF true, HF false, ZF true
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms ADD", processor.ADD(128, 128, false), (n8)0);
    EXPECT_TRUE("Algorithms ADD", processor.CF);
    EXPECT_FALSE("Algorithms ADD", processor.HF);
    EXPECT_FALSE("Algorithms ADD", processor.NF);
    EXPECT_TRUE("Algorithms ADD", processor.ZF);
  }

  // Variant of previous case: sum is same but HF is different!
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms ADD", processor.ADD(128, 127, true), (n8)0);
    EXPECT_TRUE("Algorithms ADD", processor.CF);
    EXPECT_TRUE("Algorithms ADD", processor.HF);
    EXPECT_FALSE("Algorithms ADD", processor.NF);
    EXPECT_TRUE("Algorithms ADD", processor.ZF);
  }

  // CF true, HF true, ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms ADD", processor.ADD(143, 143, false), (n8)30);
    EXPECT_TRUE("Algorithms ADD", processor.CF);
    EXPECT_TRUE("Algorithms ADD", processor.HF);
    EXPECT_FALSE("Algorithms ADD", processor.NF);
    EXPECT_FALSE("Algorithms ADD", processor.ZF);
  }

  // CF true, HF true, ZF true
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms ADD", processor.ADD(3, 252, true), (n8)0);
    EXPECT_TRUE("Algorithms ADD", processor.CF);
    EXPECT_TRUE("Algorithms ADD", processor.HF);
    EXPECT_FALSE("Algorithms ADD", processor.NF);
    EXPECT_TRUE("Algorithms ADD", processor.ZF);
  }
}

void ExhaustiveTestADD() {
  int outcomes[8] = {0};
  for (int x = 0; x <= 255; ++x) {
    for (int y = 0; y <= 255; ++y) {
      for (bool carry : {false, true}) {
        ::ares::GameBoy::CPU processor;
        processor.ADD(x, y, carry);
        int index = 0;
        if (processor.CF) {
          index |= 4;
        }
        if (processor.HF) {
          index |= 2;
        }
        if (processor.ZF) {
          index |= 1;
        }
        outcomes[index] += 1;
      }
    }
  }
  // CF false, HF false, ZF false
  EXPECT_EQ("Algorithms ADD", outcomes[0b000], 34815);

  // CF false, HF false, ZF true
  EXPECT_EQ("Algorithms ADD", outcomes[0b001], 1);

  // CF false, HF true, ZF false
  EXPECT_EQ("Algorithms ADD", outcomes[0b010], 30720);

  // CF false, HF true, ZF true
  EXPECT_EQ("Algorithms ADD", outcomes[0b011], 0);

  // CF true, HF false, ZF false
  EXPECT_EQ("Algorithms ADD", outcomes[0b100], 30705);

  // CF true, HF false, ZF true
  EXPECT_EQ("Algorithms ADD", outcomes[0b101], 15);

  // CF true, HF true, ZF false
  EXPECT_EQ("Algorithms ADD", outcomes[0b110], 34320);

  // CF true, HF true, ZF true
  EXPECT_EQ("Algorithms ADD", outcomes[0b111], 496);
}

void TestAND() {
  // ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms AND", processor.AND(6, 7), (n8)6);
    EXPECT_FALSE("Algorithms AND", processor.CF);
    EXPECT_TRUE("Algorithms AND", processor.HF);
    EXPECT_FALSE("Algorithms AND", processor.NF);
    EXPECT_FALSE("Algorithms AND", processor.ZF);
  }

  // ZF true
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms AND", processor.AND(6, 8), (n8)0);
    EXPECT_FALSE("Algorithms AND", processor.CF);
    EXPECT_TRUE("Algorithms AND", processor.HF);
    EXPECT_FALSE("Algorithms AND", processor.NF);
    EXPECT_TRUE("Algorithms AND", processor.ZF);
  }
}

void ExhaustiveTestAND() {
  int outcomes[2] = {0};
  for (int x = 0; x <= 255; ++x) {
    for (int y = 0; y <= 255; ++y) {
      ::ares::GameBoy::CPU processor;
      processor.AND(x, y);
      int index = 0;
      if (processor.ZF) {
        index |= 1;
      }
      outcomes[index] += 1;
    }
  }
  // ZF false
  EXPECT_EQ("Algorithms AND", outcomes[0b0], 58975);

  // ZF true
  EXPECT_EQ("Algorithms AND", outcomes[0b1], 6561);
}

void TestBIT() {
  // ZF false
  {
    ::ares::GameBoy::CPU processor;
    processor.BIT(1, 0b10100101);
    EXPECT_TRUE("Algorithms BIT", processor.HF);
    EXPECT_FALSE("Algorithms BIT", processor.NF);
    EXPECT_TRUE("Algorithms BIT", processor.ZF);
  }

  // ZF true
  {
    ::ares::GameBoy::CPU processor;
    processor.BIT(2, 0b10100101);
    EXPECT_TRUE("Algorithms BIT", processor.HF);
    EXPECT_FALSE("Algorithms BIT", processor.NF);
    EXPECT_FALSE("Algorithms BIT", processor.ZF);
  }
}

void ExhaustiveTestBIT() {
  int outcomes[2] = {0};
  for (int index = 0; index <= 7; ++index) {
    for (int x = 0; x <= 255; ++x) {
      ::ares::GameBoy::CPU processor;
      processor.BIT(index, x);
      int outcome_index = 0;
      if (processor.ZF) {
        outcome_index |= 1;
      }
      outcomes[outcome_index] += 1;
    }
  }
  // ZF false
  EXPECT_EQ("Algorithms BIT", outcomes[0b0], 1024);

  // ZF true
  EXPECT_EQ("Algorithms BIT", outcomes[0b1], 1024);
}

void TestCP() {
  // CF false, HF false, ZF false
  {
    ::ares::GameBoy::CPU processor;
    processor.CP(10, 5);
    EXPECT_FALSE("Algorithms CP", processor.CF);
    EXPECT_FALSE("Algorithms CP", processor.HF);
    EXPECT_TRUE("Algorithms CP", processor.NF);
    EXPECT_FALSE("Algorithms CP", processor.ZF);
  }

  // CF false, HF false, ZF true
  {
    ::ares::GameBoy::CPU processor;
    processor.CP(10, 10);
    EXPECT_FALSE("Algorithms CP", processor.CF);
    EXPECT_FALSE("Algorithms CP", processor.HF);
    EXPECT_TRUE("Algorithms CP", processor.NF);
    EXPECT_TRUE("Algorithms CP", processor.ZF);
  }

  // CF false, HF true, ZF false
  {
    ::ares::GameBoy::CPU processor;
    processor.CP(100, 10);
    EXPECT_FALSE("Algorithms CP", processor.CF);
    EXPECT_TRUE("Algorithms CP", processor.HF);
    EXPECT_TRUE("Algorithms CP", processor.NF);
    EXPECT_FALSE("Algorithms CP", processor.ZF);
  }

  // CF false, HF true, ZF true impossible

  // CF true, HF false, ZF false
  {
    ::ares::GameBoy::CPU processor;
    processor.CP(2, 17);
    EXPECT_TRUE("Algorithms CP", processor.CF);
    EXPECT_FALSE("Algorithms CP", processor.HF);
    EXPECT_TRUE("Algorithms CP", processor.NF);
    EXPECT_FALSE("Algorithms CP", processor.ZF);
  }

  // CF true, HF false, ZF true impossible

  // CF true, HF true, ZF false
  {
    ::ares::GameBoy::CPU processor;
    processor.CP(2, 19);
    EXPECT_TRUE("Algorithms CP", processor.CF);
    EXPECT_TRUE("Algorithms CP", processor.HF);
    EXPECT_TRUE("Algorithms CP", processor.NF);
    EXPECT_FALSE("Algorithms CP", processor.ZF);
  }

  // CF true, HF true, ZF true impossible
}

void ExhaustiveTestCP() {
  int outcomes[8] = {0};
  for (int x = 0; x <= 255; ++x) {
    for (int y = 0; y <= 255; ++y) {
      ::ares::GameBoy::CPU processor;
      processor.CP(x, y);
      int index = 0;
      if (processor.CF) {
        index |= 4;
      }
      if (processor.HF) {
        index |= 2;
      }
      if (processor.ZF) {
        index |= 1;
      }
      outcomes[index] += 1;
    }
  }
  // CF false, HF false, ZF false
  EXPECT_EQ("Algorithms CP", outcomes[0b000], 18240);

  // CF false, HF false, ZF true
  EXPECT_EQ("Algorithms CP", outcomes[0b001], 256);

  // CF false, HF true, ZF false
  EXPECT_EQ("Algorithms CP", outcomes[0b010], 14400);

  // CF false, HF true, ZF true
  EXPECT_EQ("Algorithms CP", outcomes[0b011], 0);

  // CF true, HF false, ZF false
  EXPECT_EQ("Algorithms CP", outcomes[0b100], 16320);

  // CF true, HF false, ZF true
  EXPECT_EQ("Algorithms CP", outcomes[0b101], 0);

  // CF true, HF true, ZF false
  EXPECT_EQ("Algorithms CP", outcomes[0b110], 16320);

  // CF true, HF true, ZF true
  EXPECT_EQ("Algorithms CP", outcomes[0b111], 0);
}

void TestDEC() {
  // HF false, ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms DEC", processor.DEC(10), (n8)9);
    EXPECT_FALSE("Algorithms DEC", processor.HF);
    EXPECT_TRUE("Algorithms DEC", processor.NF);
    EXPECT_FALSE("Algorithms DEC", processor.ZF);
  }

  // HF false, ZF true
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms DEC", processor.DEC(1), (n8)0);
    EXPECT_FALSE("Algorithms DEC", processor.HF);
    EXPECT_TRUE("Algorithms DEC", processor.NF);
    EXPECT_TRUE("Algorithms DEC", processor.ZF);
  }

  // HF true, ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms DEC", processor.DEC(32), (n8)31);
    EXPECT_TRUE("Algorithms DEC", processor.HF);
    EXPECT_TRUE("Algorithms DEC", processor.NF);
    EXPECT_FALSE("Algorithms DEC", processor.ZF);
  }

  // HF true, ZF true impossible

  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms DEC", processor.DEC(0), (n8)255);
    EXPECT_TRUE("Algorithms DEC", processor.HF);
    EXPECT_TRUE("Algorithms DEC", processor.NF);
    EXPECT_FALSE("Algorithms DEC", processor.ZF);
  }
}

void ExhaustiveTestDEC() {
  int outcomes[4] = {0};
  for (int x = 0; x <= 255; ++x) {
    ::ares::GameBoy::CPU processor;
    processor.DEC(x);
    int index = 0;
    if (processor.HF) {
      index |= 2;
    }
    if (processor.ZF) {
      index |= 1;
    }
    outcomes[index] += 1;
  }
  // HF false, ZF false
  EXPECT_EQ("Algorithms DEC", outcomes[0b00], 239);

  // HF false, ZF true
  EXPECT_EQ("Algorithms DEC", outcomes[0b01], 1);

  // HF true, ZF false
  EXPECT_EQ("Algorithms DEC", outcomes[0b10], 16);

  // HF true, ZF true
  EXPECT_EQ("Algorithms DEC", outcomes[0b11], 0);
}

void TestINC() {
  // HF false, ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms INC", processor.INC(10), (n8)11);
    EXPECT_FALSE("Algorithms INC", processor.HF);
    EXPECT_FALSE("Algorithms INC", processor.NF);
    EXPECT_FALSE("Algorithms INC", processor.ZF);
  }

  // HF false, ZF true impossible

  // HF true, ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms INC", processor.INC(31), (n8)32);
    EXPECT_TRUE("Algorithms INC", processor.HF);
    EXPECT_FALSE("Algorithms INC", processor.NF);
    EXPECT_FALSE("Algorithms INC", processor.ZF);
  }

  // HF true, ZF true
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms INC", processor.INC(255), (n8)0);
    EXPECT_TRUE("Algorithms INC", processor.HF);
    EXPECT_FALSE("Algorithms INC", processor.NF);
    EXPECT_TRUE("Algorithms INC", processor.ZF);
  }
}

void ExhaustiveTestINC() {
  int outcomes[4] = {0};
  for (int x = 0; x <= 255; ++x) {
    ::ares::GameBoy::CPU processor;
    processor.INC(x);
    int index = 0;
    if (processor.HF) {
      index |= 2;
    }
    if (processor.ZF) {
      index |= 1;
    }
    outcomes[index] += 1;
  }
  // HF false, ZF false
  EXPECT_EQ("Algorithms INC", outcomes[0b00], 240);

  // HF false, ZF true
  EXPECT_EQ("Algorithms INC", outcomes[0b01], 0);

  // HF true, ZF false
  EXPECT_EQ("Algorithms INC", outcomes[0b10], 15);

  // HF true, ZF true
  EXPECT_EQ("Algorithms INC", outcomes[0b11], 1);
}

void TestOR() {
  // ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms OR", processor.OR(6, 9), (n8)15);
    EXPECT_FALSE("Algorithms OR", processor.CF);
    EXPECT_FALSE("Algorithms OR", processor.HF);
    EXPECT_FALSE("Algorithms OR", processor.NF);
    EXPECT_FALSE("Algorithms OR", processor.ZF);
  }

  // ZF true
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms OR", processor.OR(0, 0), (n8)0);
    EXPECT_FALSE("Algorithms OR", processor.CF);
    EXPECT_FALSE("Algorithms OR", processor.HF);
    EXPECT_FALSE("Algorithms OR", processor.NF);
    EXPECT_TRUE("Algorithms OR", processor.ZF);
  }
}

void ExhaustiveTestOR() {
  int outcomes[2] = {0};
  for (int x = 0; x <= 255; ++x) {
    for (int y = 0; y <= 255; ++y) {
      ::ares::GameBoy::CPU processor;
      processor.OR(x, y);
      int index = 0;
      if (processor.ZF) {
        index |= 1;
      }
      outcomes[index] += 1;
    }
  }
  // ZF false
  EXPECT_EQ("Algorithms OR", outcomes[0b0], 65535);

  // ZF true
  EXPECT_EQ("Algorithms OR", outcomes[0b1], 1);
}

void TestRL() {
  // CF false, ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms RL", processor.RL(10), (n8)20);
    EXPECT_FALSE("Algorithms RL", processor.CF);
    EXPECT_FALSE("Algorithms RL", processor.HF);
    EXPECT_FALSE("Algorithms RL", processor.NF);
    EXPECT_FALSE("Algorithms RL", processor.ZF);
  }

  // CF false, ZF true
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms RL", processor.RL(0), (n8)0);
    EXPECT_FALSE("Algorithms RL", processor.CF);
    EXPECT_FALSE("Algorithms RL", processor.HF);
    EXPECT_FALSE("Algorithms RL", processor.NF);
    EXPECT_TRUE("Algorithms RL", processor.ZF);
  }

  // CF true, ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms RL", processor.RL(130), (n8)4);
    EXPECT_TRUE("Algorithms RL", processor.CF);
    EXPECT_FALSE("Algorithms RL", processor.HF);
    EXPECT_FALSE("Algorithms RL", processor.NF);
    EXPECT_FALSE("Algorithms RL", processor.ZF);
  }

  // CF true, ZF true
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms RL", processor.RL(0b10000000), (n8)0);
    EXPECT_TRUE("Algorithms RL", processor.CF);
    EXPECT_FALSE("Algorithms RL", processor.HF);
    EXPECT_FALSE("Algorithms RL", processor.NF);
    EXPECT_TRUE("Algorithms RL", processor.ZF);
  }

  {
    ::ares::GameBoy::CPU processor;
    processor.CF = true;
    EXPECT_EQ("Algorithms RL", processor.RL(0), (n8)1);
    EXPECT_FALSE("Algorithms RL", processor.CF);
    EXPECT_FALSE("Algorithms RL", processor.HF);
    EXPECT_FALSE("Algorithms RL", processor.NF);
    EXPECT_FALSE("Algorithms RL", processor.ZF);
  }

  {
    ::ares::GameBoy::CPU processor;
    processor.CF = true;
    EXPECT_EQ("Algorithms RL", processor.RL(0b10000000), (n8)1);
    EXPECT_TRUE("Algorithms RL", processor.CF);
    EXPECT_FALSE("Algorithms RL", processor.HF);
    EXPECT_FALSE("Algorithms RL", processor.NF);
    EXPECT_FALSE("Algorithms RL", processor.ZF);
  }
}

void ExhaustiveTestRL() {
  int outcomes[4] = {0};
  for (int x = 0; x <= 255; ++x) {
    ::ares::GameBoy::CPU processor;
    processor.RL(x);
    int index = 0;
    if (processor.CF) {
      index |= 2;
    }
    if (processor.ZF) {
      index |= 1;
    }
    outcomes[index] += 1;
  }
  // CF false, ZF false
  EXPECT_EQ("Algorithms RL", outcomes[0b00], 127);

  // CF false, ZF true
  EXPECT_EQ("Algorithms RL", outcomes[0b01], 1);

  // CF true, ZF false
  EXPECT_EQ("Algorithms RL", outcomes[0b10], 127);

  // CF true, ZF true
  EXPECT_EQ("Algorithms RL", outcomes[0b11], 1);
}

void TestRLC() {
  // CF false, ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms RLC", processor.RL(10), (n8)20);
    EXPECT_FALSE("Algorithms RLC", processor.CF);
    EXPECT_FALSE("Algorithms RLC", processor.HF);
    EXPECT_FALSE("Algorithms RLC", processor.NF);
    EXPECT_FALSE("Algorithms RLC", processor.ZF);
  }

  // CF false, ZF true
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms RLC", processor.RL(0), (n8)0);
    EXPECT_FALSE("Algorithms RLC", processor.CF);
    EXPECT_FALSE("Algorithms RLC", processor.HF);
    EXPECT_FALSE("Algorithms RLC", processor.NF);
    EXPECT_TRUE("Algorithms RLC", processor.ZF);
  }

  // CF true, ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms RLC", processor.RLC(130), (n8)5);
    EXPECT_TRUE("Algorithms RLC", processor.CF);
    EXPECT_FALSE("Algorithms RLC", processor.HF);
    EXPECT_FALSE("Algorithms RLC", processor.NF);
    EXPECT_FALSE("Algorithms RLC", processor.ZF);
  }

  // CF true, ZF true impossible
}

void ExhaustiveTestRLC() {
  int outcomes[4] = {0};
  for (int x = 0; x <= 255; ++x) {
    ::ares::GameBoy::CPU processor;
    processor.RLC(x);
    int index = 0;
    if (processor.CF) {
      index |= 2;
    }
    if (processor.ZF) {
      index |= 1;
    }
    outcomes[index] += 1;
  }
  // CF false, ZF false
  EXPECT_EQ("Algorithms RLC", outcomes[0b00], 127);

  // CF false, ZF true
  EXPECT_EQ("Algorithms RLC", outcomes[0b01], 1);

  // CF true, ZF false
  EXPECT_EQ("Algorithms RLC", outcomes[0b10], 128);

  // CF true, ZF true
  EXPECT_EQ("Algorithms RLC", outcomes[0b11], 0);
}

void TestRR() {
  // CF false, ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms RR", processor.RR(10), (n8)5);
    EXPECT_FALSE("Algorithms RR", processor.CF);
    EXPECT_FALSE("Algorithms RR", processor.HF);
    EXPECT_FALSE("Algorithms RR", processor.NF);
    EXPECT_FALSE("Algorithms RR", processor.ZF);
  }

  // CF false, ZF true
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms RR", processor.RR(0), (n8)0);
    EXPECT_FALSE("Algorithms RR", processor.CF);
    EXPECT_FALSE("Algorithms RR", processor.HF);
    EXPECT_FALSE("Algorithms RR", processor.NF);
    EXPECT_TRUE("Algorithms RR", processor.ZF);
  }

  // CF true, ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms RR", processor.RR(3), (n8)1);
    EXPECT_TRUE("Algorithms RR", processor.CF);
    EXPECT_FALSE("Algorithms RR", processor.HF);
    EXPECT_FALSE("Algorithms RR", processor.NF);
    EXPECT_FALSE("Algorithms RR", processor.ZF);
  }

  // CF true, ZF true
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms RR", processor.RR(1), (n8)0);
    EXPECT_TRUE("Algorithms RR", processor.CF);
    EXPECT_FALSE("Algorithms RR", processor.HF);
    EXPECT_FALSE("Algorithms RR", processor.NF);
    EXPECT_TRUE("Algorithms RR", processor.ZF);
  }

  {
    ::ares::GameBoy::CPU processor;
    processor.CF = true;
    EXPECT_EQ("Algorithms RR", processor.RR(0), (n8)0b10000000);
    EXPECT_FALSE("Algorithms RR", processor.CF);
    EXPECT_FALSE("Algorithms RR", processor.HF);
    EXPECT_FALSE("Algorithms RR", processor.NF);
    EXPECT_FALSE("Algorithms RR", processor.ZF);
  }

  {
    ::ares::GameBoy::CPU processor;
    processor.CF = true;
    EXPECT_EQ("Algorithms RR", processor.RR(1), (n8)0b10000000);
    EXPECT_TRUE("Algorithms RR", processor.CF);
    EXPECT_FALSE("Algorithms RR", processor.HF);
    EXPECT_FALSE("Algorithms RR", processor.NF);
    EXPECT_FALSE("Algorithms RR", processor.ZF);
  }
}

void ExhaustiveTestRR() {
  int outcomes[4] = {0};
  for (int x = 0; x <= 255; ++x) {
    ::ares::GameBoy::CPU processor;
    processor.RR(x);
    int index = 0;
    if (processor.CF) {
      index |= 2;
    }
    if (processor.ZF) {
      index |= 1;
    }
    outcomes[index] += 1;
  }
  // CF false, ZF false
  EXPECT_EQ("Algorithms RR", outcomes[0b00], 127);

  // CF false, ZF true
  EXPECT_EQ("Algorithms RR", outcomes[0b01], 1);

  // CF true, ZF false
  EXPECT_EQ("Algorithms RR", outcomes[0b10], 127);

  // CF true, ZF true
  EXPECT_EQ("Algorithms RR", outcomes[0b11], 1);
}

void TestRRC() {
  // CF false, ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms RRC", processor.RR(2), (n8)1);
    EXPECT_FALSE("Algorithms RRC", processor.CF);
    EXPECT_FALSE("Algorithms RRC", processor.HF);
    EXPECT_FALSE("Algorithms RRC", processor.NF);
    EXPECT_FALSE("Algorithms RRC", processor.ZF);
  }

  // CF false, ZF true
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms RRC", processor.RR(0), (n8)0);
    EXPECT_FALSE("Algorithms RRC", processor.CF);
    EXPECT_FALSE("Algorithms RRC", processor.HF);
    EXPECT_FALSE("Algorithms RRC", processor.NF);
    EXPECT_TRUE("Algorithms RRC", processor.ZF);
  }

  // CF true, ZF false
  {
    ::ares::GameBoy::CPU processor;
    EXPECT_EQ("Algorithms RRC", processor.RRC(1), (n8)0b10000000);
    EXPECT_TRUE("Algorithms RRC", processor.CF);
    EXPECT_FALSE("Algorithms RRC", processor.HF);
    EXPECT_FALSE("Algorithms RRC", processor.NF);
    EXPECT_FALSE("Algorithms RRC", processor.ZF);
  }

  // CF true, ZF true impossible
}

void ExhaustiveTestRRC() {
  int outcomes[4] = {0};
  for (int x = 0; x <= 255; ++x) {
    ::ares::GameBoy::CPU processor;
    processor.RRC(x);
    int index = 0;
    if (processor.CF) {
      index |= 2;
    }
    if (processor.ZF) {
      index |= 1;
    }
    outcomes[index] += 1;
  }
  // CF false, ZF false
  EXPECT_EQ("Algorithms RRC", outcomes[0b00], 127);

  // CF false, ZF true
  EXPECT_EQ("Algorithms RRC", outcomes[0b01], 1);

  // CF true, ZF false
  EXPECT_EQ("Algorithms RRC", outcomes[0b10], 128);

  // CF true, ZF true
  EXPECT_EQ("Algorithms RRC", outcomes[0b11], 0);
}

void TestAll() {
  TestADD();
  ExhaustiveTestADD();
  TestAND();
  ExhaustiveTestAND();
  TestBIT();
  ExhaustiveTestBIT();
  TestCP();
  ExhaustiveTestCP();
  TestDEC();
  ExhaustiveTestDEC();
  TestINC();
  ExhaustiveTestINC();
  TestOR();
  ExhaustiveTestOR();
  TestRL();
  ExhaustiveTestRL();
  TestRLC();
  ExhaustiveTestRLC();
  TestRRC();
  ExhaustiveTestRRC();
}
} // namespace algorithms
