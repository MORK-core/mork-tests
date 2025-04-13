cmake_minimum_required(VERSION 3.10)
project(mork-tests)

if(CMAKE_SOURCE_DIR STREQUAL PROJECT_SOURCE_DIR)
    message("mork tests use local config")
    include(${PROJECT_SOURCE_DIR}/config.cmake)
endif()

set(TESTS_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set(TESTS_TARGET_DIR "${CMAKE_CURRENT_BINARY_DIR}/tests")

set(LINKER_SCRIPT "${CMAKE_CURRENT_SOURCE_DIR}/linker-qemu-user.ld")

set(RUSTFLAGS "-C link-arg=-T${LINKER_SCRIPT} -C relocation-model=static")

file(GLOB_RECURSE BIN_FILES "${TESTS_DIR}/src/bin/*.rs")

set(BIN_TARGETS "")
foreach(BIN_FILE ${BIN_FILES})
    get_filename_component(BIN_NAME ${BIN_FILE} NAME_WE)
    set(TESTS_FILE "${TESTS_TARGET_DIR}/${TARGET}/${CARGO_BUILD_TYPE}/${BIN_NAME}")

    # 首先添加自定义命令来生成输出文件
    add_custom_command(
        OUTPUT "${CMAKE_BINARY_DIR}/tests/${BIN_NAME}.bin"
        COMMAND ${CMAKE_COMMAND} -E
            env "RUSTFLAGS=${RUSTFLAGS}" "CARGO_TARGET_DIR=${TESTS_TARGET_DIR}"
            "TARGET_BIN_NAME=${BIN_NAME}"
            ${CARGO_COMMAND} build ${CARGO_ARGS} --bin ${BIN_NAME}
        COMMAND ${OBJCOPY} --strip-all "${TESTS_FILE}" "${TESTS_FILE}.bin"
        COMMAND ${CMAKE_COMMAND} -E copy "${TESTS_FILE}.bin" "${CMAKE_BINARY_DIR}/tests"
        WORKING_DIRECTORY ${TESTS_DIR}
        COMMENT "Building Rust binary: ${BIN_NAME}"
        DEPENDS ${BIN_FILE}
    )

    # 然后添加目标依赖于此命令
    add_custom_target(
        build_${BIN_NAME}
        DEPENDS "${CMAKE_BINARY_DIR}/tests/${BIN_NAME}.bin"
    )
    list(APPEND BIN_TARGETS build_${BIN_NAME})
endforeach()

# 添加总构建目标
add_custom_target(mork_tests ALL DEPENDS ${BIN_TARGETS})

# 确保输出目录存在
file(MAKE_DIRECTORY "${CMAKE_BINARY_DIR}/tests")
