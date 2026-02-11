# css/CSS2/normal-flow/max-height-percentage-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/max-height-percentage-002.xht"
}
```

## style[0]

```css

            #div1
            {
                position: relative;
            }
            div div
            {
                width: 1in;
            }
            #div2
            {
                background: blue;
                height: 2in;
                max-height: 50%;
            }
            #div3
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
