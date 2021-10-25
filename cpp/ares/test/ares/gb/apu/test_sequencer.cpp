namespace sequencer {

void TestPower() {
  APU::Sequencer sequencer;

  sequencer.leftVolume = 2;
  sequencer.power();
  EXPECT_EQ("Sequencer power", sequencer.leftVolume, (n3)0);
}

void TestAll() { TestPower(); }
} // namespace sequencer
