# css/CSS2/positioning/absolute-non-replaced-width-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-non-replaced-width-004.xht"
}
```

## style[0]

```css

            #div1
            {
                border: solid black;
                direction: ltr;
                height: 200px;
                position: relative;
                width: 200px;
            }
            div div
            {
                background: red;
                color: blue;
                font: 100px/1em Ahem;
                left: 100px;
                margin-left: auto;
                margin-right: auto;
                position: absolute;
                right: 100px;
                width: 100px;
            }
            /*
                left                    :   100px
              + margin-left             :   solve (auto)
              + border-left-width       :   0
              + padding-left            :   0
              + width                   :   100px
              + padding-right           :   0
              + border-right-width      :   0
              + margin-right            :   solve (auto)
              + right                   :   100px
              ====================================
              width of containing block :   200px

            So, margin-left and margin-right would be each -50px at this point.

            "...unless this would make them (the two margins) negative
            in which case when direction of the containing block is
            'ltr' ('rtl'), set 'margin-left' ('margin-right') to zero and
            solve for 'margin-right' ('margin-left')."

            So, under such extra constraint, 'margin-left' must become 0
            and 'margin-right' must become -100px.
            */
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
