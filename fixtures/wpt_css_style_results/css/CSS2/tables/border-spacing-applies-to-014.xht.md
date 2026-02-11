# css/CSS2/tables/border-spacing-applies-to-014.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-spacing-applies-to-014.xht"
}
```

## style[0]

```css

            #table
            {
                display: table;
            }
            #tr
            {
                display: table-row;
            }
            .td
            {
                border-spacing: 20px;
                display: table-cell;
                height: 100px;
                width: 100px;
            }
            .left
            {
                border: 10px solid blue;
            }
            .right
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
