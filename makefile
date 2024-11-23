all:
	@cargo build --release

install:
	@echo "copying the target/release/ls-trip to /usr/bin/ls-trip..."
	@if [ -e "/usr/bin/ls-trip" ]; then																						\
		echo "file /usr/bin/ls-trip already exists, do you want to replace it? [y/n]";										\
		read response;																										\
		if [ $response = "y" ] || [ $response = "Y" ]; then																	\
			cp -f target/release/ls-trip /usr/bin/ls-trip;																	\
			echo "copied target/release/ls-trip to /usr/bin/ls-trip";														\
		fi																													\
	else																													\
		cp target/release/ls-trip /usr/bin/ls-trip;																			\
	fi

	@echo "linking /usr/bin/ls-trip to /usr/bin/lsd..."
	@if [ -e "/usr/bin/lsd" ]; then																							\
		echo "file /usr/bin/lsd already exists, do you want to replace it? [y/n]";											\
		read response;																										\
		if [ $response = "y" ] || [ $response = "Y" ]; then																	\
			ln -s /usr/bin/ls-trip /usr/bin/lsd;																			\
			echo "linked /usr/bin/ls-trip to /usr/bin/lsd";																	\
		fi																													\
	else																													\
		ln -s /usr/bin/ls-trip /usr/bin/lsd;																				\
	fi

uninstall:
	@echo "removing /usr/bin/ls-trip..."
	@rm -f /usr/bin/ls-trip
	
	@echo "removing /usr/bin/lsd..."
	@if [ $(readlink "/usr/bin/lsd")="/usr/bin/ls-trip" ]; then																\
		rm -f /usr/bin/lsd;																									\
	else																													\
		echo "file /usr/bin/lsd is not a symlink to /usr/bin/ls-trip, so it won't be removed";								\
	fi

update:
	@echo "fetching the repo..."
	@git pull

	@echo "compiling the project..."
	@cargo build --release
