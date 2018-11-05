
function table_print (tt, indent, done)
    done = done or {}
    indent = indent or 0
    if type(tt) == "table" then
        local sb = {}
        for key, value in pairs (tt) do
            -- table.insert(sb, string.rep (" ", indent)) -- indent it
            if type (value) == "table" and not done [value] then
                done [value] = true
                table.insert(sb, string.format("%s = {", to_string (key)));
                table.insert(sb, table_print (value, indent + 2, done))
                -- table.insert(sb, string.rep (" ", indent)) -- indent it
                table.insert(sb, "}");
            else
                table.insert(sb, string.format(
                    "%s = %s, ", to_string (key), to_string(value)))
            end
        end
        return table.concat(sb)
    else
        return tt .. ""
    end
end

function to_string( tbl )
    if  "nil"        == type( tbl ) then
        return tostring(nil)
    elseif  "table"  == type( tbl ) then
        return table_print(tbl)
    elseif  "string" == type( tbl ) then
        return string.format("\"%s\"", tbl)
    else
        return tostring(tbl)
    end
end

function log(...)
    local a = {}
    for i, val in ... do
        a[i] = to_string(val)
    end
    print(unpack(a))
end
