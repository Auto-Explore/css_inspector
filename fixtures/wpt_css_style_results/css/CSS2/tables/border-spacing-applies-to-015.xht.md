# css/CSS2/tables/border-spacing-applies-to-015.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-spacing-applies-to-015.xht"
}
```

## style[0]

```css

            #table
            {
                display: table;
            }
            .caption1, .caption2
            {
                border-spacing: 20px;
                display: table-caption;
                height: 100px;
                width: 100px;
            }
            .caption1
            {
                border: 10px solid blue;
            }
            .caption2
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
