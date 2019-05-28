APP_NAME=$(shell cat Cargo.toml | grep -Po "(?<=name\s=\s\")[a-zA-Z0-9-_]+")
TARGET_FOLDER=build
TARGET_BUILD=${TARGET_FOLDER}/${APP_NAME}
COMMIT_NO=$(shell git rev-list --all --count)

CURDIR=$(shell pwd)
DC_MASTER="dc_master.yaml"
DC_TEMP="docker-compose.yaml"

OLIBGREP=$(shell cat Cargo.toml | grep -Po "(?<=\").*olib(?=\/)" | head -n 1)
OLIBDIR=${CURDIR}/${OLIBGREP}

ARGS_SRC="config/args.yaml"
ARGS_TRG=".argsprod.yaml"


all: dc_preparations make_args do_test do_build do_benchmark
benchmark: do_benchmark
build: dc_preparations make_args do_build do_benchmark
test: do_test


dc_preparations:
	cat ${DC_MASTER} \
		| sed 's|<CURDIR>|${CURDIR}|g' \
		| sed 's|<APP_NAME>|${APP_NAME}|g' \
		| sed 's|<OLIBDIR>|${OLIBDIR}|g' \
		> ${DC_TEMP}

make_args:
	cat "${ARGS_SRC}" | sed '/version/s/\.X\"/\.${COMMIT_NO}\"/g' > ${ARGS_TRG}

do_benchmark:
	hyperfine "${TARGET_BUILD}"

do_build:
	sudo docker-compose up --build
	sudo docker-compose down --rmi all
	sudo docker-compose rm --force
	mkdir -p "${TARGET_FOLDER}"
	mv "target/x86_64-unknown-linux-musl/release/${APP_NAME}" "${TARGET_BUILD}"
	strip "${TARGET_BUILD}"
	cp config/${APP_NAME}.yaml ${TARGET_FOLDER}/

do_test:
	cargo test
