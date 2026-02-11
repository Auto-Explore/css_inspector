# css/CSS2/lists/list-style-position-applies-to-007.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/list-style-position-applies-to-007.xht"
}
```

## style[0]

```css

            #table
            {
                display: table;
                margin-left: 1in;
            }
            #row
            {
                display: table-row;
            }
            #cell
            {
                display: table-cell;
                list-style-position: inside;
            }
            #cell div
            {
                background: orange;
                display: list-item;
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
