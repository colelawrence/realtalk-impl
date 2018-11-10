require 'util'
require 'craft'

function recog(sys)
    local p10095_1 = NEW_PAGE(sys, "Recognition")

    -- San Francisco map
    function page17595_SF_map(sys, you)
        -- Claim (you) is geomap of bbox ({left=-122.527, bottom=37.664, right=-122.348, top=37.851}).
        CLAIM(sys, you, "/1/ is geomap of bbox /2/", you, {left=-122.527, bottom=37.664, right=-122.348, top=37.851})
    end

    -- See San Francisco map
    -- start
    local p17595_1 = NEW_PAGE(sys, "San Francisco map")
    -- recognizer loop
    CLAIM(sys, p10095_1, "/1/ has width /2/", p17595_1, 7.2)
    CLAIM(sys, p10095_1, "/1/ has height /2/", p17595_1, 10.5)
    -- source file
    page17595_SF_map(sys, p17595_1)
    -- in page discovery
    TRACK_PAGE(sys, p17595_1, "p17595_1")
    -- End


    -- Geokit v1
    function page17597_Geokit_v1(sys, you)
        -- Geokit 1st try: blue
        -- On each geomap, draws blue.

        -- When /map/ is geomap of box /bbox/:
        WHEN(sys, you, "/1/ is geomap of bbox /2/",
            function (map, bbox)
                print("WHEN", to_string(map), "is geomap of bbox", to_string(bbox))
                WISH
            end
        )

        --     Wish (map) is highlighted "blue".

        -- End
        -- WHEN(sys, you, )
    end

    -- See Geokit v1
    -- start
    local p17597_1 = NEW_PAGE(sys, "Geokit v1")
    -- recognizer loop
    CLAIM(sys, p10095_1, "/1/ has width /2/", p17597_1, 7.2)
    CLAIM(sys, p10095_1, "/1/ has height /2/", p17597_1, 10.5)
    -- source file
    page17597_Geokit_v1(sys, p17597_1)
    -- in page discovery
    TRACK_PAGE(sys, p17597_1, "p17597_1")
    -- End
end

local s1 = NEW_SYSTEM()
recog(s1)
