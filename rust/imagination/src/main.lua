local ffi = require('ffi')

local ext

if ffi.os == 'Linux' then
    ext = 'so'
else
    ext = 'dylib'
end

ffi.cdef[[
char* reverse_call(const char *source);
void reverse_free(char *source);
void system_debug();
void system_tests();
]]

local lib = ffi.load('target/debug/libmanage_facts.' .. ext)
local reverse_call = lib.reverse_call
local reverse_free = lib.reverse_free
local system_debug = lib.system_debug
local system_tests = lib.system_tests

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
    system_debug()
    print("running tests")
    system_tests()
    system_debug()
    print("input number of cycles:")
    local n = io.read("*n")
    
    while n > 0 do
        n = n - 1
        test()
    end
end