# css/CSS2/bidi-text/direction-applies-to-005.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/bidi-text/direction-applies-to-005.xht"
}
```

## style[0]

```css

            #test
            {
                background: orange;
                direction: rtl;
                display: table-column-group;
            }
            #table
            {
                display: table;
                table-layout: fixed;
                width: 100px;
            }
            #row
            {
                display: table-row;
            }
            #cell
            {
                display: table-cell;
                font: 50px/1em Ahem;
                height: 100px;
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
