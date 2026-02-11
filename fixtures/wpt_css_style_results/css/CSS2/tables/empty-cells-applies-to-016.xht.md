# css/CSS2/tables/empty-cells-applies-to-016.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/empty-cells-applies-to-016.xht"
}
```

## style[0]

```css

            #table
            {
                display: table;
            }
            .tr
            {
                display: table-row;
            }
            .td
            {
                color: white;
                display: table-cell;
            }
            #test
            {
                background: red
                border: 5px solid red;
                display: none;
                empty-cells: hide;
                height: 1em;
                width: 1em;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Missing ';' between declarations.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
