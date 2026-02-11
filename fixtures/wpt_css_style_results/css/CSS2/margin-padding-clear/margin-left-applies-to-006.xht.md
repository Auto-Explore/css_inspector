# css/CSS2/margin-padding-clear/margin-left-applies-to-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-left-applies-to-006.xht"
}
```

## style[0]

```css

            #wrapper
            {
                border-left: 10px solid blue;
            }
            #test
            {
                display: table-column;
                margin-left: 50px;
            }
            #table
            {
                display: table;
            }
            #row
            {
                display: table-row;
            }
            #cell
            {
                border-left: 10px solid orange;
                display: table-cell;
                height: 200px;
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
