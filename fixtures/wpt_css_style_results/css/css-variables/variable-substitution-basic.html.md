# css/css-variables/variable-substitution-basic.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-substitution-basic.html"
}
```

## style[0]

```css

        #testArea {
            color: orange;
        }
        #testArea  > div {
            width: 50px !important;
        }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
