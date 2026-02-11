# css/CSS2/generated-content/quotes-page-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/quotes-page-001.xht"
}
```

## style[0]

```css

            #test
            {
                border: solid blue;
                quotes: '"' '"' "'" "'";
            }
            #test div:before
            {
                content: open-quote;
            }
            #test div:after
            {
                content: close-quote;
            }
            #reference
            {
                border: solid orange;
            }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “quotes”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
