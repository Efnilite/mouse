add_executable(mouse
        main.c
        vec2d.c
        vec2d.h
        vec2i.c
        vec2i.h
        maze.c
        maze.h
        hardware/motor.c
        hardware/motor.h
        hardware/sensor.c
        hardware/sensor.h
)

target_link_libraries(mouse m)

target_compile_definitions(mouse PUBLIC DEBUG)
