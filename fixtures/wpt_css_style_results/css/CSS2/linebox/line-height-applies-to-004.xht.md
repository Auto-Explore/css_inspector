# css/CSS2/linebox/line-height-applies-to-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/linebox/line-height-applies-to-004.xht"
}
```

## style[0]

```css

            #div1
            {
                position: relative;
            }
            #div2
            {
                background: orange;
                height: 2in;
                left: 1in;
                position: absolute;
                top: 0;
                width: 1in;
            }
            #table
            {
                display: table;
                table-layout: fixed;
            }
            #row
            {
                display: table-row;
                line-height: 2in;
            }
            #cell
            {
                background: blue;
                display: table-cell;
                width: 1in;
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
