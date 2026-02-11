# css/CSS2/normal-flow/block-non-replaced-height-015.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/block-non-replaced-height-015.xht"
}
```

## style[0]

```css

            #div1
            {
                position: relative;
            }
            #div2
            {
                background: blue;
            }
            #div2 div
            {
                position: relative;
                top: 1in;
            }
            #div3
            {
                background: orange;
                left: 1in;
                position: absolute;
                top: 0;
            }
            #div2 div, #div3
            {
                height: 1in;
            }
            div div
            {
                width: 1in;
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
