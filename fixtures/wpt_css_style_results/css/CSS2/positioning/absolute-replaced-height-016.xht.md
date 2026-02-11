# css/CSS2/positioning/absolute-replaced-height-016.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-replaced-height-016.xht"
}
```

## style[0]

```css

            #div1
            {
                border-top: solid black;
                position: relative;
            }
            div div
            {
                background: blue;
                height: 15px;
                left: 15px;
                position: absolute;
                top: 1in;
                width: 15px;
            }
            img
            {
                bottom: 1in;
                /*
                The equation gets overconstrained; and so, the used value for
                bottom in that test will be ignored and will be solved
                as minus (1in + 15px)  (-111px) because the height of
                containing block is 0.
                */
                height: auto;
                position: absolute;
                top: 1in;
                width: auto;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
