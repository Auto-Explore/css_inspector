# css/CSS2/linebox/line-height-applies-to-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/linebox/line-height-applies-to-006.xht"
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
                height: 1in;
                left: 1in;
                position: absolute;
                top: 0;
                width: 1in;
            }
            #test
            {
                display: table-column;
                line-height: 2in;
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
                background: blue;
                display: table-cell;
                height: 1in;
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
