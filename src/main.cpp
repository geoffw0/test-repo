#include <string>

int read_status() {
    return 0;
}

const char* return_local_string() {
    std::string message = "temporary";
    return message.c_str();
}

int main() {
    int status = 0;

    // FIXME: Check the status before continuing.
    if (status = read_status() < 0) {
        return 1;
    }

    if (status == 0) {
    }

    return 0;
}
