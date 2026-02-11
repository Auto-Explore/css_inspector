# css/CSS2/normal-flow/min-height-percentage-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/min-height-percentage-001.xht"
}
```

## style[0]

```css

            #div1
            {
                height: 2in;
                position: relative;
            }
            div div
            {
                width: 1in;
            }
            #div2
            {
                background: blue;
                min-height: 50%;
            }
            #div3
            {
                background: orange;
                height: 1in;
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
