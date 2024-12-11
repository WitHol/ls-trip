all:
	@cargo build --release

install:
	@echo "copying the target/release/ls-trip to /usr/bin/ls-trip..."
	@cp -f target/release/ls-trip /usr/bin/ls-trip;
	@echo "linking /usr/bin/ls-trip /usr/bin/lsd"
	@ln -sf /usr/bin/ls-trip /usr/bin/lsd;
	
uninstall:
	@echo "removing /usr/bin/ls-trip..."
	@rm -f /usr/bin/ls-trip
	
	@echo "removing /usr/bin/lsd..."
	@if [ $(readlink "/usr/bin/lsd")="/usr/bin/ls-trip" ]; then																\
		rm -f /usr/bin/lsd;																									\
	else																													\
		echo "file /usr/bin/lsd is not a symlink to /usr/bin/ls-trip, so it won't be removed";								\
	fi

