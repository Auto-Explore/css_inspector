# css/CSS2/lists/counter-increment-applies-to-014.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-increment-applies-to-014.xht"
}
```

## style[0]

```css

            #table
            {
                counter-increment: test 5;
                display: inline-table;
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
