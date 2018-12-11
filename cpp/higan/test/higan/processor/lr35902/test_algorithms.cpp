namespace algorithms {

void TestADD() {
  // CF false, HF false, ZF false
  {
    GameBoy::CPU processor;
    EXPECT_EQ("Algorithms ADD", processor.ADD(3, 4, false), (uint8)7);
    EXPECT_FALSE("Algorithms ADD", processor.CF);
    EXPECT_FALSE("Algorithms ADD", processor.HF);
    EXPECT_FALSE("Algorithms ADD", processor.NF);
    EXPECT_FALSE("Algorithms ADD", processor.ZF);
  }

  // CF false, HF false, ZF true
  {
    GameBoy::CPU processor;
    EXPECT_EQ("Algorithms ADD", processor.ADD(0, 0, false), (uint8)0);
    EXPECT_FALSE("Algorithms ADD", processor.CF);
    EXPECT_FALSE("Algorithms ADD", processor.HF);
    EXPECT_FALSE("Algorithms ADD", processor.NF);
    EXPECT_TRUE("Algorithms ADD", processor.ZF);
  }

  // CF false, HF true, ZF false
  {
    GameBoy::CPU processor;
    EXPECT_EQ("Algorithms ADD", processor.ADD(9, 8, false), (uint8)17);
    EXPECT_FALSE("Algorithms ADD", processor.CF);
    EXPECT_TRUE("Algorithms ADD", processor.HF);
    EXPECT_FALSE("Algorithms ADD", processor.NF);
    EXPECT_FALSE("Algorithms ADD", processor.ZF);
  }

  // CF false, HF true, ZF true impossible

  // CF true, HF false, ZF false
  {
    GameBoy::CPU processor;
    EXPECT_EQ("Algorithms ADD", processor.ADD(128, 128, true), (uint8)1);
    EXPECT_TRUE("Algorithms ADD", processor.CF);
    EXPECT_FALSE("Algorithms ADD", processor.HF);
    EXPECT_FALSE("Algorithms ADD", processor.NF);
    EXPECT_FALSE("Algorithms ADD", processor.ZF);
  }

  // CF true, HF false, ZF true
  {
    GameBoy::CPU processor;
    EXPECT_EQ("Algorithms ADD", processor.ADD(128, 128, false), (uint8)0);
    EXPECT_TRUE("Algorithms ADD", processor.CF);
    EXPECT_FALSE("Algorithms ADD", processor.HF);
    EXPECT_FALSE("Algorithms ADD", processor.NF);
    EXPECT_TRUE("Algorithms ADD", processor.ZF);
  }

  // Variant of previous case: sum is same but HF is different!
  {
    GameBoy::CPU processor;
    EXPECT_EQ("Algorithms ADD", processor.ADD(128, 127, true), (uint8)0);
    EXPECT_TRUE("Algorithms ADD", processor.CF);
    EXPECT_TRUE("Algorithms ADD", processor.HF);
    EXPECT_FALSE("Algorithms ADD", processor.NF);
    EXPECT_TRUE("Algorithms ADD", processor.ZF);
  }

  // CF true, HF true, ZF false
  {
    GameBoy::CPU processor;
    EXPECT_EQ("Algorithms ADD", processor.ADD(143, 143, false), (uint8)30);
    EXPECT_TRUE("Algorithms ADD", processor.CF);
    EXPECT_TRUE("Algorithms ADD", processor.HF);
    EXPECT_FALSE("Algorithms ADD", processor.NF);
    EXPECT_FALSE("Algorithms ADD", processor.ZF);
  }

  // CF true, HF true, ZF true
  {
    GameBoy::CPU processor;
    EXPECT_EQ("Algorithms ADD", processor.ADD(3, 252, true), (uint8)0);
    EXPECT_TRUE("Algorithms ADD", processor.CF);
    EXPECT_TRUE("Algorithms ADD", processor.HF);
    EXPECT_FALSE("Algorithms ADD", processor.NF);
    EXPECT_TRUE("Algorithms ADD", processor.ZF);
  }
}

void ExhaustiveTestAdd() {
  int outcomes[8] = {0};
  for (int x = 0; x <= 255; ++x) {
    for (int y = 0; y <= 255; ++y) {
      for (bool carry : {false, true}) {
        GameBoy::CPU processor;
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

void TestAll() {
  TestADD();
  ExhaustiveTestAdd();
}
}
