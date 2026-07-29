add_executable(vk-cube
    main.c
    gloam/src/vk.c
)
target_include_directories(vk-cube PRIVATE gloam/include)
target_link_libraries(vk-cube PRIVATE SDL3::SDL3)
if(NOT WIN32)
    target_link_libraries(vk-cube PRIVATE m)
endif()
