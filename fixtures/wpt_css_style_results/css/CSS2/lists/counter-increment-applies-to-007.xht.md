# css/CSS2/lists/counter-increment-applies-to-007.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-increment-applies-to-007.xht"
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
                display: table-row;
            }
            #cell
            {
                counter-increment: test 5;
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
