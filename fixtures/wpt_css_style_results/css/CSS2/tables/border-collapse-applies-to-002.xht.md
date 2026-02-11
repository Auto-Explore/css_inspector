# css/CSS2/tables/border-collapse-applies-to-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-collapse-applies-to-002.xht"
}
```

## style[0]

```css

            #bottom, span
            {
                border-collapse: collapse;
                display: block;
                height: 100px;
                width: 100px;
            }
            span
            {
                border-bottom: 10px solid blue;
            }
            #bottom
            {
                border-top: 10px dotted orange;
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
