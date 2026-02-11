# css/CSS2/tables/border-spacing-applies-to-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-spacing-applies-to-008.xht"
}
```

## style[0]

```css

            #table
            {
                display: table;
            }
            .test
            {
                border-spacing: 20px;
                display: table-row-group;
            }
            .tr
            {
                display: table-row;
            }
            .td
            {
                display: table-cell;
                height: 100px;
                width: 100px;
            }
            .top
            {
                border: 10px solid blue;
            }
            .bottom
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
