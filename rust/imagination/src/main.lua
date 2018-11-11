local ffi = require('ffi')

local ext

if ffi.os == 'Linux' then
    ext = 'so'
else
    ext = 'dylib'
end

ffi.cdef[[
int32_t double_input(int32_t input);
// const size_t test(const char *source, char *dest);
char* test(const char *source);
void test_free(char *source);
]]

local lib = ffi.load('target/debug/libdouble_input.' .. ext)
local double_input = lib.double_input
local test = lib.test
local test_free = lib.test_free

local rec_cstr = test("Hello, world")
local rec = ffi.string(rec_cstr)
print("recc:", ffi.string(rec_cstr))
test_free(rec_cstr)
print("recc:", ffi.string(rec_cstr))
print("rec:", rec)

-- local input = 4
-- local output = double_input(input)
-- print(input .. " * 2 = " .. output)
