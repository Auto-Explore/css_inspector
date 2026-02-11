# css/css-exclusions/wrap-through-001.html

```json
{
  "format_version": 3,
  "file": "css/css-exclusions/wrap-through-001.html"
}
```

## style[0]

```css

            * {
                margin: 0px;
                padding: 0px;
            }
            .exclusion {
                wrap-flow: both;
                position: absolute;
                top: 15px;
                left: 70px;
                width: 60px;
                height: 36px;
                background: blue;
            }
            #content1, #content2 {
                width: 400px;
                line-height: 12px;
                font: 12px Ahem;
            }
            #content2 {
                wrap-through: none;
            }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “wrap-flow”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “wrap-through”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
