const TEST = `
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
`

const TEST_0 = `
When /someone/ wishes /map/ shows tilelayer /url_template/ with options /opts/,
     /map/ is geomap of bbox /bbox/,
     /map/ has width /width/, /map/ has height /height/:

    print("Hello")
    -- ...
End
`

class LineError extends Error {
    constructor(public rel: number, public line_message: string) {
        super(line_message)
    }
}

function dissect_line_when(lns: string[]): string[] {
    console.log("dissect", lns)
    return []
}

function match_empty(peek_at: (i) => string, take: (n) => string[]): LineError | string[] {
    const res = []
    console.log(`match empty \'${peek_at(0)}\'`)
    while (peek_at(0) == "") {
        take(1)
        res.push("")
    }
    return res.length > 0 ? res : null
}

function match_line_when(peek_at: (i) => string, take: (n) => string[]): LineError | string[] {
    let ln = peek_at(0);
    if (ln.startsWith("When")) {
        let nln = ln;
        let index = 0;
        while (nln.endsWith(",")) {
            index += 1;
            nln = peek_at(index);
        }
        if (nln.endsWith(":")) {
            return dissect_line_when(take(index + 1));
        } else {
            return new LineError(index, "Expected When line to end with \":\"")
        }
    }
    return null
}

type Section = {
    source_lines: string[]
    result_lines: string[]
}

class Consume {
    private lines: string[]
    private sections: Section[]
    constructor(source: string) {
        this.lines = source.split(/\n\r?/)

        let index = 0;

        this.parse(
            (i) => this.lines[i + index],
            (n) => {
                let res = this.lines.slice(index, index + n)
                index += n
                console.log("take", n, index)
                return res
            }
        )

        console.log(this.lines)
    }

    private parse(peek_at: (i) => string, take: (n) => string[]) {
        const parsers = [
            match_empty,
            match_line_when,
        ]
        const parsers_len = parsers.length
        let done = false;
        while(peek_at(0) != null && !done) {
            let parser = null
            for(let i = 0; i < parsers_len; i++) {
                parser = parsers[i]
                let res = parser(peek_at, take)
                if (res instanceof LineError) {
                    console.error(`Error parsing line ${res.rel}: ${res.line_message}`)
                } else if (res instanceof Array) {
                    console.log("found", res)
                } else if (res == null) {
                    continue;
                }
            }
            // no parser could figure oput
            console.log("No parser could parse")
            break;
        }
    }
}

function test(t: string) {
    console.log(t)
    new Consume(t)

}

test(TEST_0)

const a1 = `
When /someone/ wishes /map/ shows tilelayer /url_template/ with options /opts/,
     /map/ is geomap of bbox /bbox/,
     /map/ has width /width/, /map/ has height /height/:

    print("Hello")
    -- ...
End
`
const a1a = [
    `/someone/ wishes /map/ shows tilelayer /url_template/ with options /opts/`, // STMT_A
    `/map/ is geomap of bbox /bbox/`, // STMT_B
    `/map/ has width /width/, /map/ has height /height/`, // STMT_C & STMT_D
]

const STMT_A_TABLE = [
    { someone: 1001, map: 1002, url_template: 4003, opts: 2001 },
    { someone: 1001, map: 1003, url_template: 4002, opts: 2003 }
]

const STMT_B_TABLE = [
    { map: 1002, bbox: 1004 },
    { map: 9001, bbox: 9004 }
]

const STMT_C_TABLE = [
    { map: 1002, width: 3004 },
    { map: 1003, width: 3005 },
    { map: 1001, width: 3006 },
    { map: 1004, width: 3007 },
]

const STMT_D_TABLE = [
    { map: 1002, height: 3104 },
    { map: 1003, height: 3105 },
    { map: 1001, height: 3106 },
    { map: 1004, height: 3107 },
]