# css/CSS2/positioning/position-relative-007.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/position-relative-007.xht"
}
```

## style[0]

```css

            div
            {
                height: 1in;
                width: 1in;
            }
            #div1
            {
                margin-left: 1in;
            }
            #div2
            {
                background: blue;
            }
            #div3
            {
                background: orange;
                position: relative;
                left: auto;
                right: 1in;
                top: -1in;
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
