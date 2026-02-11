# css/CSS2/positioning/absolute-replaced-height-017.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-replaced-height-017.xht"
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
                height: 1in;
                left: 1in;
                position: absolute;
                top: 1in;
            }
            img
            {
                bottom: 1in;
                /*
                The equation gets overconstrained; and so, the used value for
                bottom in that test will be ignored and will be solved
                as minus (1in + 1in)  (-192px) because the height of
                containing block is 0.
                */
                height: auto;
                position: absolute;
                top: 1in;
            }
            div div, img
            {
                width: 1in;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
