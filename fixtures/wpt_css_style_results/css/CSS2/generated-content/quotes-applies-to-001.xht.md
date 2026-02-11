# css/CSS2/generated-content/quotes-applies-to-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/quotes-applies-to-001.xht"
}
```

## style[0]

```css

            #test
            {
                display: table-row-group;
                quotes: "P" "S" "A" "S";
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
                content: open-quote open-quote close-quote close-quote;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “quotes”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
