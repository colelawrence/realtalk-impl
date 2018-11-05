-- Touch shapes
When (you) points "up" at /page/,
        /page/ has touches /touches/:

    log(touches)
    if touches.KEY_SPACE then
        When the time is /t/:
            local il1 = new_illumination()
            il1.translate { x=1, y=2 }
            il1.rotate { deg2rads * t % 180 }
            il1.rectangel { x=1. y=1. width=4, height=4, fill="blue", stroke="white" }

            Wish (you) has illuminations (il1).
        End
    end
End
