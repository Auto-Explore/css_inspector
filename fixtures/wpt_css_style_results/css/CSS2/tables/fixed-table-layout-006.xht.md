# css/CSS2/tables/fixed-table-layout-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/fixed-table-layout-006.xht"
}
```

## style[0]

```css

            table
            {
                background: blue;
                border-collapse: separate;
                border-spacing: 10px 0;
                table-layout: fixed;
                width: 130px;
            }
            td
            {
                background: blue;
                border-left: 10px solid orange;
                border-right: 10px solid orange;
                height: 100px;
                padding: 0;
            }
            #specified
            {
                width: 10px;
            }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
