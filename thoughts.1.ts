import * as _ from 'lodash';
namespace t1 {
    const STMT_KEY = '@'
    const new_sys: () => System = () => ({
        parse: {
            stmts_by_template: {}
        }
    })
    const new_page: () => Page = () => ({
        id: "Page",
        stmts: []
    })

    const a1a = [
        `/someone:SA1/ wishes /map:SA2/ shows tilelayer /template_url:SA3/ with options /opts:SA4/`, // STMT_A
        `/map:SB1/ is geomap of bbox /bbox:SB2/`, // STMT_B
        `/map:SC1/ has width /width:SC2/`, // STMT_C
        `/map:SD1/ has height /height:SD2/`, // STMT_D
    ]

    const a1a_QUERY = [
        ['SA2', 'SB1', 'SC1', 'SD1'],
        ['SA1'],
        ['SA3'],
        ['SA4'],
        ['SB2'],
        // ['SC1'],
        ['SC2'],
        // ['SD1'],
        ['SD2']
    ]

    const a1b_test = `
    When /stmt/ wishes /map/ shows tilelayer
            /template_url/
            with  options /opts/,
        /map/ is geomap of bbox /bbox/, -- comment:
        /map/ has width /width/, /map/ has height /height/:
    `
    console.log(parse_when(a1b_test))
    console.log(translate_when(parse_when(a1b_test)))

    interface Stmt { id: string }
    interface Page { id: string, stmts: Stmt[] }
    interface System {
        parse: {
            stmts_by_template: {
                [template: string]: { id: Symbol, references: Page[] }
            }
        }
    }
    function sys_parse_stmt(sys: System, src: Page, stmt: string) {
        const into_key = (stmt) => stmt.replace(/\/\s*([^\/]+?)\s*\//g, '@')
        const key = into_key(stmt)
        console.log(key)
        const existing = sys.parse.stmts_by_template[key]
        if (existing != null) {

        }
    }
    function parse_when(source: string) {
        return source
            .replace(/\s*--[^\n]+\n/g, '\n') // remove comments
            .replace(/^\s*When\s*/, '')
            .replace(/:\s*$/, '')
            .replace(/\s+/g, ' ') // replace all repeating spaces / newlines with one space
            .split(/,\s*/g)
    }
    function translate_when(stmts: string[]) {
        let letters = 'ABCDEFGHI'.split('');
        let sys = new_sys()
        let origin = new_page()

        return stmts.map((stmt, index) => sys_parse_stmt(sys, origin, stmt))
    }

    // Wish handler
    const a1b = [
        // first argument looks like the subject
        `/stmt:SA1/ wishes /map:SA2/ shows tilelayer /template_url:SA3/ with options /opts:SA4/`, // STMT_A
        `/map:SB1/ is geomap of bbox /bbox:SB2/`, // STMT_B
        `/map:SC1/ has width /width:SC2/`, // STMT_C
        `/map:SD1/ has height /height:SD2/`, // STMT_D
        `time /t:GE1/`, // GLOBAL_E
    ]

    // Wish maker
    const a0b = [
        ['STMT_A', { SA1: 10101, SA2: 1002, SA3: 4003, SA4: 2001 }]
    ]

    const a1b_QUERY = [
        ['SA1'], // stmt
        ['SA2', 'SB1', 'SC1', 'SD1'], // map
        ['SA3'], // template_url
        ['SA4'], // opts
        ['SB2'], // bbox
        ['SC2'], // width
        ['SD2'], // height
        ['GE1'] // t
    ]

    // /SA1/ wishes /SA2/ shows tilelayer /SA3/ with options /SA4/
    const STMT_A_TABLE = [
        { SA1: 10101, SA2: 1002, SA3: 4003, SA4: 2001 },
        { SA1: 10102, SA2: 1003, SA3: 4002, SA4: 2003 }
    ]

    // /SB1/ is geomap of bbox /SB2/
    const STMT_B_TABLE = [
        { SB1: 1002, SB2: 1004 },
        { SB1: 9001, SB2: 9004 }
    ]

    // /SC1/ has width /SC2/
    const STMT_C_TABLE = [
        { SC1: 1001, SC2: 3006 },
        { SC1: 1002, SC2: 3004 },
        { SC1: 1003, SC2: 3005 },
        { SC1: 1004, SC2: 3007 },
    ]

    // /SD1/ has height /SD2/
    const STMT_D_TABLE = [
        { SD1: 1001, SD2: 3106 },
        { SD1: 1002, SD2: 3104 },
        { SD1: 1003, SD2: 3105 },
        { SD1: 1004, SD2: 3107 },
    ]

    // time /GE1/
    const G_E1 = 998001

    const TABLES_o1: any[][] = [STMT_A_TABLE, STMT_B_TABLE, STMT_C_TABLE, STMT_D_TABLE]

    const o1 = _.flatten(TABLES_o1)
    // console.log("vars", o1)
}

namespace t2 {
    const a1a = [
        `/page:SA1/ blahblahblah`, // STMT_2A
    ]

    const a1a_QUERY = [
        ['SA1'],
    ]

    const STMT_A_TABLE = []

    const TABLES: any[][] = [STMT_A_TABLE]
}