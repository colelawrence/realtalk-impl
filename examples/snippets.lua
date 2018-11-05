-- San Francisco map
Claim (you) is geomap of bbox ({left=-122.527, bottom=37.664, right=-122.348, top=37.851}).

-- Geokit...
local URL_TEMPLATE = "https://tile.openstreetmap.org/%d/%d/%d.png"
local ZOOM = 11
When /map/ is geomap of bbox /bbox/,
     /map/ has width /width/, /map/ has height /height/:

    local xtile, ytile = deg2num((bbox.left + bbox.right)/2,
                                 (bbox.top + bbox.bottom)/2,
                                 ZOOM)
    local url, path = get_tile_url_and_path(URL_TEMPLATE, ZOOM,
                                            math.floor(xtile),
                                            math.floor(ytile))
    local tilesize = math.min(width, height)
    When tile URL (url) is downloaded to path (path):
        local ill = new_illumination()
        ill:image { filename=path, scale=1,
                    x=0, y=0,
                    width=tilesize, height=tilesize }
        Wish (map) has illumination (ill).
    End

End

--
When /someone/ wishes /page/ shows tilelayer /url_template/ with options /opts/,
     /page/ wishes /target/ shows tilelayer /something/ with options /something/,
     page ~= target:
    opts = rt.table(opts) -- Make a mutable copy of opts
    opts.priority = 1 + (opts.priority or 1)
    Wish (target) shows tilelayer (url_template) with options (opts).
End

local ZOOM = 11
When /someone/ wishes /map/ shows tilelayer /url_template/ with options /opts/,
     /map/ is geomap of bbox /bbox/,
     /map/ has width /width/, /map/ has height /height/:

    -- ...
End

