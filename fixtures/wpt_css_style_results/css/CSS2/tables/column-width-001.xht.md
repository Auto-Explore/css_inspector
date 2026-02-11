# css/CSS2/tables/column-width-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/column-width-001.xht"
}
```

## style[0]

```css

            #table
            {
                display: table;
            }
            #column
            {
                background: orange;
                display: table-column;
            }
            #row
            {
                display: table-row;
            }
            #cell
            {
                display: table-cell;
                width: 0.5in;
            }
            #div1
            {
                background: blue;
            }
            #div1, #cell
            {
                height: 1in;
            }
            #column, #div1
            {
                width: 1in;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
