# css/CSS2/margin-padding-clear/margin-top-applies-to-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-top-applies-to-002.xht"
}
```

## style[0]

```css

            #wrapper
            {
                border-top: 10px solid blue;
            }
            #test
            {
                display: table-header-group;
                margin-top: 50px;
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
                border-top: 10px solid orange;
                display: table-cell;
                height: 200px;
                width: 200px;
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
