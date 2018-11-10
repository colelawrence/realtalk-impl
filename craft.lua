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
        behs = {},
    }
end

function NEW_PAGE(sys, friendly_name)
    sys.last_page = sys.last_page + 1
    return {
        id = "page#"..tostring(sys.last_page),
        friendly_name = friendly_name,
    }
end

-- Token behavior

function ENSURE_TOKEN_BEHAVIOR(sys, token)
    local beh = sys.behs[token.id]
    if beh == nil then
        beh = {
            token = token,
            listeners = {},
            value = nil,
        }
        sys.behs[token.id] = beh
    end
    return beh
end

function TOKEN_BEHAVIOR_SUBSCRIBE(sys, token, fn)
    local beh = ENSURE_TOKEN_BEHAVIOR(sys, token)

    if beh.value ~= nil then
        fn(beh.value)
    end

    table.insert(beh.listeners, fn)
end

function TOKEN_BEHAVIOR_PUBLISH(sys, token, value)
    local beh = ENSURE_TOKEN_BEHAVIOR(sys, token)
    beh.value = value

    table.insert(beh.listeners, fn)
    for listener in ipairs(beh.listeners) do
        log("token beh", listener)
    end
end

function TOKEN_FROM(text)
    local n = 0
    local var = {}
    local idx = {}
    print("from", text)
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

function TOKEN_ID(text)
    print("id", text)
    local id = string.gsub(text, "/[%a]+[%a%d]*/",
        function()
            return "/"..tostring(n).."/"
        end
    )
    return id
end

log(
    TOKEN_FROM("/map/ is geomap of bbox /bbox/")
)

-- Register when like a listener
function WHEN(sys, page, token_id, fn)
    local listener_id = token_id
    print("WHEN", listener_id,  to_string(sys.behs[listener_id]))
end

function CLAIM(sys, page, token_id, ...)
    local listener_id = token_id
    TOKEN_BEHAVIOR_PUBLISH(sys, {id = token_id}, ...)
    print("CLAIM", listener_id,  to_string(sys.behs[listener_id]))
    
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