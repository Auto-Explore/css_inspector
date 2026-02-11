# css/CSS2/margin-padding-clear/padding-left-applies-to-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-left-applies-to-009.xht"
}
```

## style[0]

```css

            span
            {
                display: block;
            }
            #span1
            {
                border-left: 10px solid blue;
                padding-left: 50px;
            }
            span span
            {
                border-left: 10px solid orange;
                height: 200px;
                width: 200px;
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
