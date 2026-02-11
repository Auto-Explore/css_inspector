# css/CSS2/normal-flow/block-non-replaced-height-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/block-non-replaced-height-006.xht"
}
```

## style[0]

```css

            html, body
            {
                overflow: scroll;
            }
            div
            {
                position: relative;
            }
            #div1
            {
                background: red;
                width: 200px;
            }
            span
            {
                color: blue;
                display: inline;
                font: 100px/1 Ahem;
            }
            #div2
            {
                background: orange;
                height: 200px;
                left: 200px;
                position: absolute;
                top: 0;
                width: 200px;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
