# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.27

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/archeoss/playground/smhasher/smhasher

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/archeoss/playground/smhasher/smhasher

# Utility rule file for TAGS.

# Include any custom commands dependencies for this target.
include CMakeFiles/TAGS.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/TAGS.dir/progress.make

CMakeFiles/TAGS:
	etags --language=c++ *.cpp *.cc *.h

TAGS: CMakeFiles/TAGS
TAGS: CMakeFiles/TAGS.dir/build.make
.PHONY : TAGS

# Rule to build all files generated by this target.
CMakeFiles/TAGS.dir/build: TAGS
.PHONY : CMakeFiles/TAGS.dir/build

CMakeFiles/TAGS.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/TAGS.dir/cmake_clean.cmake
.PHONY : CMakeFiles/TAGS.dir/clean

CMakeFiles/TAGS.dir/depend:
	cd /home/archeoss/playground/smhasher/smhasher && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/archeoss/playground/smhasher/smhasher /home/archeoss/playground/smhasher/smhasher /home/archeoss/playground/smhasher/smhasher /home/archeoss/playground/smhasher/smhasher /home/archeoss/playground/smhasher/smhasher/CMakeFiles/TAGS.dir/DependInfo.cmake "--color=$(COLOR)"
.PHONY : CMakeFiles/TAGS.dir/depend
