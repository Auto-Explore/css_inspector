# css/CSS2/zindex/z-index-applies-to-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/zindex/z-index-applies-to-003.xht"
}
```

## style[0]

```css

            #test
            {
                background: green;
                display: table-footer-group;
                z-index: 1;
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
                display: table-cell;
            }
            #zindex
            {
                background: yellow;
                top: -1in;
            }
            #test, #zindex
            {
                position: relative;
            }
            #cell, #zindex
            {
                height: 1in;
                width: 1in;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
