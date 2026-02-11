# css/CSS2/margin-padding-clear/margin-right-applies-to-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-right-applies-to-001.xht"
}
```

## style[0]

```css

            #wrapper
            {
                border-right: 10px solid orange;
                float: left;
            }
            #table
            {
                display: table;
            }
            #test
            {
                display: table-row-group;
                margin-right: 50px;
            }
            #row
            {
                display: table-row;
            }
            #cell
            {
                border-right: 10px solid blue;
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
