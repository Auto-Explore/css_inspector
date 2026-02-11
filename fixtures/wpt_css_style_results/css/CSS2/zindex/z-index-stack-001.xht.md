# css/CSS2/zindex/z-index-stack-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/zindex/z-index-stack-001.xht"
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
                height: 1in;
                position: absolute;
                width: 1in;
            }
            #div2
            {
                background: orange;
                left: 30px;
                top: 30px;
                z-index: 2;
            }
            #div3
            {
                background: yellow;
                left: 60px;
                top: 60px;
                z-index: 3;
            }
            #div4
            {
                background: blue;
                left: 0;
                top: 0;
                z-index: 1;
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
