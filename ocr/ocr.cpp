#include <charconv>
#include <cstring>
#include <iostream>
#include <string>
#include <system_error>

#include <boost/algorithm/string/trim.hpp>

#include <leptonica/allheaders.h>
#include <tesseract/baseapi.h>

using tesseract::TessBaseAPI;

void panic(const std::string& msg) {
    std::cerr << msg << '\n';
    std::exit(1);
}

void ocr_number(TessBaseAPI& api, const int x, const int y, const int w, const int h) {
    constexpr int NUM_MIN = 1;
    constexpr int NUM_MAX = 512;

    api.SetRectangle(x, y, w, h);
    std::string text(api.GetUTF8Text());

    boost::trim(text);
    const auto* text_first = text.data();
    const auto* text_last = text.data() + text.size();

    int num;
    const auto res = std::from_chars(text_first, text_last, num);
    const auto ok = res.ec == std::errc {} && res.ptr == text_last && NUM_MIN <= num && num <= NUM_MAX;

    if (ok)
        std::cout << num;
    else
        std::cout << '?';
}

void ocr_cell(TessBaseAPI& api, const int r, const int c) {
    const int x0 = 44 + 71 * c;
    const int y0 = 40 + 72 * r;

    {
        const int x = x0;
        const int y = y0;
        const int w = 66;
        const int h = 32;

        ocr_number(api, x, y, w, h);
    }

    std::cout << '/';

    {
        const int x = x0;
        const int y = y0 + 36;
        const int w = 66;
        const int h = 32;

        ocr_number(api, x, y, w, h);
    }
}

void ocr(TessBaseAPI& api) {
    constexpr int N = 16; // 行数/列数

    for (int r = 0; r < N; ++r) {
        for (int c = 0; c < N; ++c) {
            if (c != 0)
                std::cout << ' ';

            ocr_cell(api, r, c);
        }
        std::cout << '\n';
    }
}

int main() {
    TessBaseAPI api;
    if (api.Init(nullptr, "eng") != 0)
        panic("cannot init tesseract");

    auto* const pix = pixRead("atan.png");
    if (!pix)
        panic("pixRead() failed");
    if (pixSetResolution(pix, 72, 72) != 0)
        panic("pixSetResolution() failed");
    api.SetImage(pix);

    ocr(api);

    return 0;
}
