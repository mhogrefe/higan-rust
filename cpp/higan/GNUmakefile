build := optimize
include ../nall/GNUmakefile

binary := application
target := tomoko
objects := libco emulator audio video resource test test_higan test_higan_gb test_higan_gb_apu test_higan_processor test_higan_processor_lr35902

flags += -I. -I.. -Wno-shift-count-overflow

ifeq ($(platform),windows)
  link += $(if $(call streq,$(console),true),-mconsole,-mwindows)
  ifeq ($(binary),application)
    link += -mthreads -lpthread -luuid -lkernel32 -luser32 -lgdi32 -lcomctl32 -lcomdlg32 -lshell32
    link += -Wl,-enable-auto-import
    link += -Wl,-enable-runtime-pseudo-reloc
  else ifeq ($(binary),library)
    link += -shared
  endif
else ifeq ($(platform),macos)
  ifeq ($(binary),application)
  else ifeq ($(binary),library)
    flags += -fPIC
    link += -dynamiclib
  endif
else ifneq ($(filter $(platform),linux bsd),)
  flags += -fopenmp
  link += -fopenmp
  ifeq ($(binary),application)
    flags += -march=native
    link += -Wl,-export-dynamic
    link += -lX11 -lXext
  else ifeq ($(binary),library)
    flags += -fPIC
    link += -shared
  endif
else
  $(error "unsupported platform")
endif

compile = \
  $(strip \
    $(if $(filter %.c,$<), \
      $(compiler) $(cflags) $(flags) $1 -c $< -o $@, \
      $(if $(filter %.cpp,$<), \
        $(compiler) $(cppflags) $(flags) $1 -c $< -o $@ \
      ) \
    ) \
  )

%.o: $<; $(call compile)

all: build;

obj/libco.o: ../libco/libco.c $(call rwildcard,../libco)
obj/emulator.o: emulator/emulator.cpp $(call rwildcard,emulator)
obj/audio.o: audio/audio.cpp $(call rwildcard,audio)
obj/video.o: video/video.cpp $(call rwildcard,video)
obj/resource.o: resource/resource.cpp $(call rwildcard,resource)

obj/test.o: test/test_all.cpp $(call rwildcard,test)
obj/test_higan.o: test/higan/test_higan.cpp $(call rwildcard,test/test_higan)
obj/test_higan_gb.o: test/higan/gb/test_gb.cpp $(call rwildcard,test/higan/gb)
obj/test_higan_gb_apu.o: test/higan/gb/apu/test_apu.cpp $(call rwildcard,test/higan/gb/apu)
obj/test_higan_processor.o: test/higan/processor/test_processor.cpp $(call rwildcard,test/higan/processor)
obj/test_higan_processor_lr35902.o: test/higan/processor/lr35902/test_lr35902.cpp $(call rwildcard,test/higan/processor/lr35902)

ui := target-$(target)
include $(ui)/GNUmakefile

clean:
	-@$(call delete,out/*)
	-@$(call delete,obj/*)
