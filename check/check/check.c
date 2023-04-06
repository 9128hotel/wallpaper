#define _CRT_SECURE_NO_WARNINGS // i do not care

#include <stdio.h>
#include <stdlib.h>

// Include the appropriate header for checking access rights based on the platform
#ifdef _WIN32 // checks if program is running on windows
#include <io.h> // io.h for windows only
#else
#include <unistd.h> // unistd.h only works on linux/unix systems
#endif

int main() {
    const char* path = "TEMP";
    char temp_file_name[] = "TEMP\\tempfileXXXXXX";
    int result = -1; // result of access check, initially -1 (faliure)

    // create file in "path" then immideately delete it
    FILE* temp_file = tmpfile(); // create file
    if (temp_file) { // if file was created successfully
        char* temp_file_path = _mktemp(temp_file_name); // create a path to the file
        if (temp_file_path) { // if the file was created successfully
            fclose(temp_file); // close the temp file
            remove(temp_file_path); // delete the temp file
            result = 0; // set the result to 0 (success)
        }
    }

    // print the result
    if (result == 0) {
        printf("%s is writable\n", path);
    }
    else {
        printf("%s is not writable\n", path);
    }

    printf("Press any key to exit...");
    getchar(); // wait for input before closing

    return 0;
}