#include <string>

int read_status() {
    return 0;
}

// Return c_str of local std::string
const char* return_local_string() {
    std::string message = "temporary";
    return message.c_str();
}

// FIXME: This is a 'fixme' comment.
int main() {
    int status = 0;

    // Ambiguous assignment of comparison result
    if (status = read_status() < 0) {
        return 1;
    }

    // Futile conditional
    if (status == 0) {
    }

    return 0;
}
