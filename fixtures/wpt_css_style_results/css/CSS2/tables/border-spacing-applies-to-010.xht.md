# css/CSS2/tables/border-spacing-applies-to-010.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-spacing-applies-to-010.xht"
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
                display: table-footer-group;
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
                border: 10px solid orange;
            }
            .bottom
            {
                border: 10px solid blue;
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
