# css/CSS2/margin-padding-clear/margin-left-applies-to-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-left-applies-to-009.xht"
}
```

## style[0]

```css

            div, span
            {
                height: 200px;
            }
            div
            {
                border-left: 10px solid blue;
            }
            span
            {
                border-left: 10px solid orange;
                display: block;
                margin-left: 50px;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
