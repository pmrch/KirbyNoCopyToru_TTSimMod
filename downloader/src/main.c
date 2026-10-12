#include <curl/curl.h>
#include <stdio.h>

#define LOG_ERROR(msg) ((void)fprintf(stderr, "\x1b[31m[ERROR]\x1b[0m: [%s:%d:%s]: %s", __FILE__, __LINE__, __FUNCTION__, msg))
static size_t write_data(void *ptr, size_t size, size_t nmemb, FILE *stream) { return fwrite(ptr, size, nmemb, stream); }

// NOLINTNEXTLINE(readability-function-cognitive-complexity)
static int download_file(const char *const URL, CURL *handle, const char *const FILENAME) {
    FILE    *fp = NULL;
    CURLcode res;

    fp = fopen(FILENAME, "wb");
    if (fp == NULL) {
        LOG_ERROR("Failed to open file!");
        return -1;
    }

    curl_easy_setopt(handle, CURLOPT_URL, URL);
    curl_easy_setopt(handle, CURLOPT_WRITEDATA, fp);

    res = curl_easy_perform(handle);

    if (res != CURLE_OK) {
        LOG_ERROR("Failed to perform cURL request!");
        return -1;
    }

    (void)fclose(fp);
    return 0;
}

// NOLINTBEGIN
int main(void) {
    CURL *curl = NULL;

    curl = curl_easy_init();
    if (curl == NULL) {
        LOG_ERROR("Failed to initialize the cURL handle");
        return -1;
    }

    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_data);
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1L); // follow redirects

    // for () {}
    download_file("https://gitlab.com", curl, "gitlab.html");
    curl_easy_cleanup(curl);
    return 0;
}
// NOLINTEND
