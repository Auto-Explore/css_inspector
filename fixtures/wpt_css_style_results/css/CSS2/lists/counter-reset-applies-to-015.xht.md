# css/CSS2/lists/counter-reset-applies-to-015.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-reset-applies-to-015.xht"
}
```

## style[0]

```css

            #table
            {
                display: table;
            }
            #caption
            {
                display: table-caption;
                counter-reset: test 5;
            }
            #row
            {
                display: table-row;
            }
            #cell
            {
                display: table-cell;
            }
            #cell:before
            {
                content: counter(test);
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
