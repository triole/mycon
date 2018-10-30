APP_NAME=$(shell cat Cargo.toml | grep -Po "(?<=name\s=\s\")[a-zA-Z0-9-_]+")
TARGET_FOLDER=build
TARGET_BUILD=${TARGET_FOLDER}/${APP_NAME}
COMMIT_NO=$(shell git rev-list --all --count)

ARGS_SRC="config/args.yaml"
ARGS_TRG=".argsprod.yaml"

all: make_args run_test run_build
build: make_args run_build
test: run_test

make_args:
	cat "${ARGS_SRC}" | sed '/version/s/\.X\"/\.${COMMIT_NO}\"/g' > ${ARGS_TRG}

run_build:
	cargo build --release
	mkdir -p ${TARGET_FOLDER}
	mv "target/release/${APP_NAME}" "${TARGET_BUILD}"
	strip "${TARGET_BUILD}"
	cp config/${APP_NAME}.yaml ${TARGET_FOLDER}/
	hyperfine "${TARGET_BUILD}"

run_test:
	cargo test
