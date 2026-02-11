# css/CSS2/positioning/absolute-non-replaced-width-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-non-replaced-width-009.xht"
}
```

## style[0]

```css

            #containingblock
            {
                border: solid black;
                direction: rtl;
                height: 200px;
                position: relative;
                width: 300px;
            }
            div div
            {
                /* left + margin-left + border-left-width + padding-left + width + padding-right + border-right-width + margin-right + right = width of containing block */
                /* 100px  + 100px         + 0                 + 0            + 100px   + 0             + 0                  + solve        + 100px = 300px */
                background: red;
                color: blue;
                font: 100px/1 Ahem;
                left: 100px;
                margin-left: 100px;
                margin-right: auto; /* value is solved to -100px */
                position: absolute;
                right: 100px;
                width: 100px;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
