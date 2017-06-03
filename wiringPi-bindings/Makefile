unexport CFLAGS
wiringpi:
	@echo >&2 $(CFLAGS)
	$(MAKE) -C ../WiringPi/wiringPi clean
	$(MAKE) -C ../WiringPi/wiringPi static CC=arm-linux-gnueabihf-gcc DEBUG=-O2
	rm -f $(OUT_DIR)/libwiringpi.a
	cp ../WiringPi/wiringPi/libwiringPi.a $(OUT_DIR)/libwiringpi.a
