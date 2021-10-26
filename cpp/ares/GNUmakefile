ifeq ($(platform),windows)
  options += -luuid -lkernel32 -luser32 -lgdi32 -lcomctl32 -lcomdlg32 -lshell32
  options += -Wl,-enable-auto-import
  options += -Wl,-enable-runtime-pseudo-reloc
else ifeq ($(platform),macos)
else ifneq ($(filter $(platform),linux bsd),)
  options += -Wl,-export-dynamic
  options += -lX11 -lXext
else
  $(error "ares is a library and cannot be built directly.")
endif

ares.objects := ares ares-fixed-allocator test test_ares test_ares_component test_ares_component_processor test_ares_component_processor_sm83 test_ares_gb test_ares_gb_apu

$(object.path)/ares.o: $(ares.path)/ares/ares.cpp
$(object.path)/ares-fixed-allocator.o: $(ares.path)/ares/memory/fixed-allocator.cpp

ifeq ($(vulkan),true)
  flags += -DVULKAN
endif

ifeq ($(profile),accuracy)
  flags += -DPROFILE_ACCURACY
endif

ifeq ($(profile),performance)
  flags += -DPROFILE_PERFORMANCE
endif

ifneq ($(filter $(cores),fc),)
  include $(ares.path)/fc/GNUmakefile
endif

ifneq ($(filter $(cores),sfc),)
  include $(ares.path)/sfc/GNUmakefile
endif

ifneq ($(filter $(cores),n64),)
  include $(ares.path)/n64/GNUmakefile
endif

ifneq ($(filter $(cores),sg),)
  include $(ares.path)/sg/GNUmakefile
endif

ifneq ($(filter $(cores),ms),)
  include $(ares.path)/ms/GNUmakefile
endif

ifneq ($(filter $(cores),md),)
  include $(ares.path)/md/GNUmakefile
endif

ifneq ($(filter $(cores),saturn),)
  include $(ares.path)/saturn/GNUmakefile
endif

ifneq ($(filter $(cores),ps1),)
  include $(ares.path)/ps1/GNUmakefile
endif

ifneq ($(filter $(cores),pce),)
  include $(ares.path)/pce/GNUmakefile
endif

ifneq ($(filter $(cores),msx),)
  include $(ares.path)/msx/GNUmakefile
endif

ifneq ($(filter $(cores),cv),)
  include $(ares.path)/cv/GNUmakefile
endif

ifneq ($(filter $(cores),gb),)
  include $(ares.path)/gb/GNUmakefile
endif

ifneq ($(filter $(cores),gba),)
  include $(ares.path)/gba/GNUmakefile
endif

ifneq ($(filter $(cores),ws),)
  include $(ares.path)/ws/GNUmakefile
endif

ifneq ($(filter $(cores),ng),)
  include $(ares.path)/ng/GNUmakefile
endif

ifneq ($(filter $(cores),ngp),)
  include $(ares.path)/ngp/GNUmakefile
endif

include $(ares.path)/component/GNUmakefile

flags += $(foreach c,$(call strupper,$(cores)),-DCORE_$c)

ares.objects := $(ares.objects:%=$(object.path)/%.o)

$(object.path)/test.o: $(ares.path)/test/test_all.cpp
$(object.path)/test_ares.o: $(ares.path)/test/ares/test_ares.cpp $(call rwildcard,test/test_ares)
$(object.path)/test_ares_component.o: $(ares.path)/test/ares/component/test_component.cpp $(call rwildcard,test/ares/component)
$(object.path)/test_ares_component_processor.o: $(ares.path)/test/ares/component/processor/test_processor.cpp $(call rwildcard,test/ares/component/processor)
$(object.path)/test_ares_component_processor_sm83.o: $(ares.path)/test/ares/component/processor/sm83/test_sm83.cpp $(call rwildcard,test/ares/component/processor/sm83)
$(object.path)/test_ares_gb.o: $(ares.path)/test/ares/gb/test_gb.cpp $(call rwildcard,test/ares/gb)
$(object.path)/test_ares_gb_apu.o: $(ares.path)/test/ares/gb/apu/test_apu.cpp $(call rwildcard,test/ares/gb/apu)

