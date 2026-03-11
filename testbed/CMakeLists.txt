add_executable(testbed
	src/main.cpp
)

target_link_libraries(testbed
	PRIVATE
	engine::engine
)
