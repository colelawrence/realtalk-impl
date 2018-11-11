local ffi = require('ffi')

local ext

if ffi.os == 'Linux' then
    ext = 'so'
else
    ext = 'dylib'
end

ffi.cdef[[
int32_t double_input(int32_t input);
char* reverse_call(const char *source);
void reverse_free(char *source);
]]

local lib = ffi.load('target/debug/libdouble_input.' .. ext)
local double_input = lib.double_input
local reverse_call = lib.reverse_call
local reverse_free = lib.reverse_free

function test()
    local input = "Hello, world yada ".. math.random(1000) .." boo booyada yada boo ".. math.random(1000) .." yada boo booyada yada boo " .. math.random(1000) .. " yada boo booyada yada boo booyada yada boo boo"
    -- print("in:", input)
    local rec_cstr = reverse_call(input)
    local rec = ffi.string(rec_cstr)
    -- print("recc:", ffi.string(rec_cstr))
    reverse_free(rec_cstr)
    -- print("recc:", ffi.string(rec_cstr))
    -- print("rec:", rec)
end


while true do
    print("input number of cycles:")
    local n = io.read("*n")
    
    while n > 0 do
        n = n - 1
        test()
    end
end