# css/CSS2/margin-padding-clear/padding-right-applies-to-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/padding-right-applies-to-009.xht"
}
```

## style[0]

```css

            div
            {
                float: left;
            }
            span
            {
                display: block;
                height: 200px;
            }
            #span1
            {
                border-right: 10px solid orange;
                padding-right: 50px;
            }
            span span
            {
                border-right: 10px solid blue;
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
