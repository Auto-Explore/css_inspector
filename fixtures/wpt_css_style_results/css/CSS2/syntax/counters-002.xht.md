# css/CSS2/syntax/counters-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/counters-002.xht"
}
```

## style[0]

```css

            div:before
            {
                content: "FAIL" counter(chapter, square, bogus) ":";
                color: red;
            }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
