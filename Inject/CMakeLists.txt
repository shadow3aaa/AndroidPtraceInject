cmake_minimum_required(VERSION 3.26)
project(Inject)
set(CMAKE_SYSTEM_NAME ANDROID)

##################### ⬇输出文件重定向⬇ #####################
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY 
    /${CMAKE_SOURCE_DIR}/outputs/${CMAKE_ANDROID_ARCH_ABI}/
) # 重定向输出产物(可执行文件)
##################### ⬆输出文件重定向⬆ #####################

##################### ⬇FLAGS⬇ #####################
set(CMAKE_CXX_FLAGS "-Os -flto -fmerge-all-constants -fno-exceptions -fomit-frame-pointer -fshort-enums -w -Wl,-O3,--lto-O3,--gc-sections,--as-needed,--icf=all,-z,norelro,--pack-dyn-relocs=android+relr")
#################### ⬆FLAGS⬆ #####################

##################### ⬇CMake头文件设置⬇ #####################
FILE(GLOB_RECURSE FILE_INCLUDES # 遍历子目录下所有符合情况的头文件
    ${CMAKE_CURRENT_SOURCE_DIR}/*.h*
)
include_directories( # 设置全局头文件目录 使其他源码文件可在任意目录<头文件.h>
    ${CMAKE_CURRENT_SOURCE_DIR}/include/
    ${CMAKE_CURRENT_SOURCE_DIR}/include/Utils/
)
##################### ⬆CMake头文件设置⬆ #####################


##################### ⬇CMake源文件设置⬇ #####################
FILE(GLOB_RECURSE FILE_SOURCES # 遍历子目录下所有符合情况的源文件
    ${CMAKE_CURRENT_SOURCE_DIR}/*.c*
)
##################### ⬆CMake源文件设置⬆ #####################


##################### ⬇添加产物⬇ #####################
add_executable(Inject # 生成动态库
    ${FILE_INCLUDES} # 头文件
    ${FILE_SOURCES} # 源文件
)
##################### ⬆添加产物 ⬆ #####################