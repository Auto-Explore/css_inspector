# css/css-exclusions/wrap-flow-004.html

```json
{
  "format_version": 3,
  "file": "css/css-exclusions/wrap-flow-004.html"
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
