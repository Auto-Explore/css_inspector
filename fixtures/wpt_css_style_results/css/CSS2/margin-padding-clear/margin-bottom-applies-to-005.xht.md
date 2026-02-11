# css/CSS2/margin-padding-clear/margin-bottom-applies-to-005.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-bottom-applies-to-005.xht"
}
```

## style[0]

```css

            #wrapper
            {
                border-bottom: 10px solid blue;
            }
            #test
            {
                display: table-column-group;
                margin-bottom: 50px;
            }
            #table
            {
                display: table;
                table-layout: fixed;
            }
            #row
            {
                display: table-row;
            }
            #cell
            {
                border-bottom: 10px solid orange;
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
