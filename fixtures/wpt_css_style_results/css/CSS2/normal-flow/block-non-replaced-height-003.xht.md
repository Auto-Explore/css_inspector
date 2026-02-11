# css/CSS2/normal-flow/block-non-replaced-height-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/block-non-replaced-height-003.xht"
}
```

## style[0]

```css

            div
            {
                position: relative;
            }
            div div
            {
                width: 1in;
            }
            #div1 div
            {
                background: blue;
                height: 2in;
            }
            #div2
            {
                background: orange;
                height: 2in;
                left: 1in;
                position: absolute;
                top: 0;
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
