# css/CSS2/margin-padding-clear/margin-applies-to-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-applies-to-001.xht"
}
```

## style[0]

```css

            #wrapper
            {
                border: 10px solid blue;
                position: absolute;
            }
            #test
            {
                display: table-row-group;
                margin: 50px;
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
                border: 10px solid orange;
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
