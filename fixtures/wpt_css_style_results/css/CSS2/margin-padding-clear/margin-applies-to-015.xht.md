# css/CSS2/margin-padding-clear/margin-applies-to-015.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-applies-to-015.xht"
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
                border: 10px solid orange;
                display: table-caption;
                height: 200px;
                margin: 50px;
            }
            #table
            {
                display: table;
                width: 320px;
            }
            #row
            {
                display: table-row;
            }
            #cell
            {
                display: table-cell;
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
