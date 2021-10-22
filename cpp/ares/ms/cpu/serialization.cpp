auto CPU::serialize(serializer& s) -> void {
  Z80::serialize(s);
  Thread::serialize(s);
  s(ram);
  s(state.nmiLine);
  s(state.irqLine);
  s(bus.ioEnable);
  s(bus.biosEnable);
  s(bus.ramEnable);
  s(bus.cardEnable);
  s(bus.cartridgeEnable);
  s(bus.expansionEnable);
  s(bus.pullup);
  s(bus.pulldown);
  s(bus.mdr);
  s(sio.parallelData);
  s(sio.dataDirection);
  s(sio.nmiEnable);
  s(sio.transmitData);
  s(sio.receiveData);
  s(sio.transmitFull);
  s(sio.receiveFull);
  s(sio.framingError);
  s(sio.unknown);
  s(sio.baudRate);
}
