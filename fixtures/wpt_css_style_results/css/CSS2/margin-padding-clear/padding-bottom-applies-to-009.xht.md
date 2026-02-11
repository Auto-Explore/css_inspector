# css/CSS2/margin-padding-clear/padding-bottom-applies-to-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-bottom-applies-to-009.xht"
}
```

## style[0]

```css

            span
            {
                display: block;
                width: 200px;
            }
            #span1
            {
                border-bottom: 10px solid blue;
                padding-bottom: 50px;
            }
            span span
            {
                border-bottom: 10px solid orange;
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
