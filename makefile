all:
	@cargo build --release

install:
	@echo "copying the target/release/ls-trip to /usr/local/bin/ls-trip..."
	@if [ -e "/usr/local/bin/ls-trip" ]; then																				\
		echo "file /usr/local/bin/ls-trip already exists, do you want to replace it? [y/n]";								\
		read response;																										\
		if [ $response = "y" ] || [ $response = "Y" ]; then																	\
			cp -f target/release/ls-trip /usr/local/bin/ls-trip;															\
			echo "copied target/release/ls-trip to /usr/local/bin/ls-trip";													\
		fi																													\
	else																													\
		cp target/release/ls-trip /usr/local/bin/ls-trip;																	\
	fi

	@echo "linking /usr/local/bin/lsd to /usr/local/bin/ls-trip..."
	@if [ -e "/usr/local/bin/lsd" ]; then																					\
		echo "file /usr/local/bin/lsd already exists, do you want to replace it? [y/n]";									\
		read response;																										\
		if [ $response = "y" ] || [ $response = "Y" ]; then																	\
			cp -f target/release/ls-trip /usr/local/bin/ls-trip;															\
			echo "linked /usr/local/bin/lsd to /usr/local/bin/ls-trip";														\
		fi																													\
	else																													\
		cp target/release/ls-trip /usr/local/bin/ls-trip;																	\
	fi

uninstall:
	@echo "removing /usr/local/bin/ls-trip..."
	@rm -f /usr/local/bin/ls-trip
	
	@echo "removing /usr/local/bin/lsd..."
	@if [ $(readlink "/usr/local/bin/lsd")="/usr/local/bin/ls-trip" ]; then										\
		rm -f /usr/local/bin/lsd;																				\
	else																										\
		echo "file /usr/local/bin/lsd is not a symlink to /usr/local/bin/ls-trip, so it won't be removed";		\
	fi

update:
	@echo "fetching the repo..."
	@git pull

	@echo "compiling the project..."
	@cargo build --release
