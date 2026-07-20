add_executable(gl-triangle
    main.c
    gloam/src/gl.c
)
target_include_directories(gl-triangle PRIVATE gloam/include)
target_link_libraries(gl-triangle PRIVATE SDL3::SDL3)
