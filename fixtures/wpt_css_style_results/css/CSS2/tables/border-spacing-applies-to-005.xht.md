# css/CSS2/tables/border-spacing-applies-to-005.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-spacing-applies-to-005.xht"
}
```

## style[0]

```css

            #parent
            {
                width: 240px;
            }
            .inline
            {
                color: white;
                display: inline-block;
                height: 100px;
                width: 100px;
            }
            .test
            {
                border: 10px solid blue;
                border-spacing: 20px;
            }
            .reference
            {
                border: 10px solid orange;
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
