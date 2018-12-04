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
void im_system_debug(void *system);
void im_system_tests(void *system);
void im_system_drop(void *system);
void* im_system_new(const char *source);
]]

local lib = ffi.load('target/debug/libmanage_facts.' .. ext)
local reverse_call = lib.reverse_call
local reverse_free = lib.reverse_free
local im_system_debug = lib.im_system_debug
local im_system_tests = lib.im_system_tests
local im_system_drop = lib.im_system_drop

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

local sys1 = lib.im_system_new("sys1")
-- im_system_debug(sys1)
-- print("running tests")
-- im_system_tests(sys1)
-- im_system_debug(sys1)
-- im_system_drop(sys1)

matches = {{a = 1, {x = 1, y = 2}, 3}, {5, {x = 0, y = 0}}, {2, {x = 1, y = 2}, "yo yo yo"}}
for _, match in ipairs(matches) do
    a, b = unpack(match)
    print(a, b)
end


-- local empty_state = {}
-- print("state")
-- print(state)

while true do
    print("input number of cycles:")
    local n = io.read("*n")
    
    while n > 0 do
        n = n - 1
        test()
    end
end