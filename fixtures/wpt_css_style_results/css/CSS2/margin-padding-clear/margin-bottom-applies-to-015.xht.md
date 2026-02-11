# css/CSS2/margin-padding-clear/margin-bottom-applies-to-015.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-bottom-applies-to-015.xht"
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
                border-bottom: 10px solid orange;
                display: table-caption;
                height: 200px;
                margin-bottom: 50px;
                width: 200px;
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
