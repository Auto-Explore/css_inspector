# css/CSS2/lists/counter-increment-applies-to-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-increment-applies-to-004.xht"
}
```

## style[0]

```css

            #table
            {
                display: table;
            }
            #row
            {
                counter-increment: test 5;
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
