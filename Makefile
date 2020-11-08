INSTALL_ARCHIVE_NAME ?= new-home-mqtt-server
CARGO_BUILD_ARGS ?= --release

ifdef TARGET
	CARGO_BUILD_ARGS += --target=$(TARGET)
endif

install:
	mkdir -p /etc/new-home-mqtt-server
	install new-home-mqtt-server /usr/bin/new-home-mqtt-server
	install new-home-mqtt-server.service /usr/lib/systemd/system/new-home-mqtt-server.service

packInstallArchive:
	cargo build $(CARGO_BUILD_ARGS)

	mkdir -p $(INSTALL_ARCHIVE_NAME)
	cp target$(if $(TARGET),/$(TARGET))/release/new-home-mqtt-server $(INSTALL_ARCHIVE_NAME)
	cp systemd/new-home-mqtt-server.service $(INSTALL_ARCHIVE_NAME)
	cp Makefile $(INSTALL_ARCHIVE_NAME)

	tar -cf $(INSTALL_ARCHIVE_NAME).tar $(INSTALL_ARCHIVE_NAME)
	gzip $(INSTALL_ARCHIVE_NAME).tar
