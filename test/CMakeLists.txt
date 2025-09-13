add_test(
        NAME test_grid_init
        COMMAND bash test.sh test/test_grid_init.c
        WORKING_DIRECTORY ${PROJECT_SOURCE_DIR}
)

add_test(
        NAME test_grid_find
        COMMAND bash test.sh test/test_grid_find.c
        WORKING_DIRECTORY ${PROJECT_SOURCE_DIR}
)