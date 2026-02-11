# css/CSS2/linebox/line-height-applies-to-012.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/linebox/line-height-applies-to-012.xht"
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
                display: inline-block;
                line-height: 2in;
                width: 1in;
            }
            #div3
            {
                background: orange;
                height: 2in;
                left: 1in;
                position: absolute;
                top: 0;
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
