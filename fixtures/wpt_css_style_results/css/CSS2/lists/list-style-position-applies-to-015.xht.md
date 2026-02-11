# css/CSS2/lists/list-style-position-applies-to-015.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/list-style-position-applies-to-015.xht"
}
```

## style[0]

```css

            #test
            {
                display: table-caption;
                list-style-position: inside;
            }
            #table
            {
                display: table;
                margin-left: 1in;
                width: 5em;
            }
            #row
            {
                display: table-row;
            }
            #cell
            {
                display: table-cell;
            }
            #test div
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
