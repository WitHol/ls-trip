all:
	@cargo build --release

install:
	@echo "copying the executable to /usr/local/bin/ls-trip..."
	@if [ -e "/usr/local/bin/ls-trip" ]; then																				\
		echo "could not create an executable at /usr/local/bin/ls-trip, because said file already exists";					\
	else																													\
		cp target/release/ls-trip /usr/local/bin/ls-trip;																		\
		echo "linking /usr/local/bin/lsd to /usr/local/bin/ls-trip...";														\
		if [ -e "/usr/local/bin/lsd" ]; then																				\
			echo "a file /usr/local/bin/lsd exists, so a symlink of that name won't be created";							\
		else																												\
			ln -s /usr/local/bin/ls-trip /usr/local/bin/lsd;																\
		fi																													\
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
