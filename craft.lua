require 'util'

-- function NEW_TOKEN(id, ...)
--     return {
--         id = id, -- "has width /1/"
--         matchers = ...
--     }
-- end

function NEW_SYSTEM()
    return {
        last_page = 0,
        pages = {},
        listeners = {},
    }
end

function NEW_PAGE(sys, friendly_name)
    sys.last_page = sys.last_page + 1
    return {
        id = "page#"..tostring(sys.last_page),
        friendly_name = friendly_name,
    }
end

function TOKEN_FROM(text)
    local n = 0
    local var = {}
    local idx = {}
    local id = string.gsub(text, "/([%a]+[%a%d]*)/",
        function(value)
            n = n + 1
            print(n, value)
            idx[n] = value
            var[value] = n
            return "/"..tostring(n).."/"
        end
    )
    return {
        id = id,
        var = var,
        idx = idx,
    }
end

log(
    TOKEN_FROM("/map/ is geomap of bbox /bbox/")
)

-- Register when like a listener
function WHEN(sys, page, token_id, fn)
    local listener_id = page.id .. token_id
    print(to_string(sys.listeners[listener_id]))
end

function CLAIM(sys, page, token_id, ...)
    
end

-- WISH is just syntactic sugar for CLAIM
function WISH(sys, page, wish_id, ...)
    CLAIM(sys, page, "wishes "..wish_id, ...)
end

function TO_KNOW_WHEN(sys, page, id, tokens)
end

function TRACK_PAGE(sys, page, tracking_id)
    print("tracking", tracking_id, to_string(page))
end