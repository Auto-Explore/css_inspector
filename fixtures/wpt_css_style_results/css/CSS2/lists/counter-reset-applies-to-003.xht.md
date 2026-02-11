# css/CSS2/lists/counter-reset-applies-to-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-reset-applies-to-003.xht"
}
```

## style[0]

```css

            #test
            {
                counter-reset: test 5;
                display: table-footer-group;
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
