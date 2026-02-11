# css/CSS2/margin-padding-clear/margin-right-applies-to-015.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-right-applies-to-015.xht"
}
```

## style[0]

```css

            #wrapper
            {
                border-right: 10px solid orange;
				float: left;
            }
            #test
            {
                border-right: 10px solid blue;
                display: table-caption;
                height: 200px;
                margin-right: 50px;
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
