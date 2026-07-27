add_executable(vk-info
    main.c
    gloam/src/vk.c
)
target_include_directories(vk-info PRIVATE gloam/include)
# The generated --loader code dlopens the Vulkan library at runtime; no
# Vulkan SDK or import library is needed to build.
if(UNIX)
    target_link_libraries(vk-info PRIVATE ${CMAKE_DL_LIBS})
endif()
