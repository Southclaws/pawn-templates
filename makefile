# -
# Development
# -

build-release:
	cargo +stable-i686-pc-windows-msvc build --release
	cp target/release/pawn_templates.dll test/plugins/templates.dll

build-debug:
	cargo +stable-i686-pc-windows-msvc build --debug
	cp target/debug/pawn_templates.dll test/plugins/templates.dll

# -
# Setup test requirements
# -

test-setup:
	cd test && sampctl server ensure
	sampctl package ensure

# -
# Run Tests
# -

test-native:
	sampctl package build
	cd test && sampctl server run

test-container:
	sampctl package build
	cd test && sampctl server run --container

# -
# Build inside container
# -

build-container:
	rm -rf build
	docker build -t southclaws/projectname-build .
	docker run -v $(shell pwd)/test/plugins:/root/test/plugins southclaws/projectname-build

# this make target is only run inside the container
build-inside:
	cd build && cmake .. && make
