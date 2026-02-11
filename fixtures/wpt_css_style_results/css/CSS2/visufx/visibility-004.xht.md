# css/CSS2/visufx/visibility-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/visufx/visibility-004.xht"
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
                background: orange;
            }
            #div2
            {
                visibility: hidden;
            }
            div div
            {
                background: red;
                visibility: inherit;
            }
            #div3
            {
                background: blue;
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
