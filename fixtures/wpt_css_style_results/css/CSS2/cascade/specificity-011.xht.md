# css/CSS2/cascade/specificity-011.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/cascade/specificity-011.xht"
}
```

## style[0]

```css

    p:after { content: "FAILED"; border: none; }
    div :after { content: "PASSED"; border-top: solid teal; }
    body div :after { border-bottom: solid blue; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
