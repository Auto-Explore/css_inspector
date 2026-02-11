# css/css-exclusions/wrap-flow-006.html

```json
{
  "format_version": 3,
  "file": "css/css-exclusions/wrap-flow-006.html"
}
```

## style[0]

```css

            * {
                margin: 0px;
                padding: 0px;
            }
            .exclusion {
                wrap-flow: maximum;
                position: absolute;
                top: 40px;
                left: 40px;
                width: 60px;
                height: 25px;
                background: blue;
            }
            #content {
                width: 400px;
                line-height: 12px;
                font: 12px Ahem;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “wrap-flow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
