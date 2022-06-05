.PHONY: all clean

CXXFLAGS := -std=c++17 -Wall -Wextra -O2
LDFLAGS := -lleptonica -ltesseract

TARGET := ocr
OBJS := ocr.o

all: $(TARGET)

$(TARGET): $(OBJS)
	$(CXX) -o $@ $(OBJS) $(LDFLAGS)

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $<

clean:
	-$(RM) $(TARGET) $(OBJS)
