# css/css-lists/counters-005.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counters-005.html"
}
```

## style[0]

```css

    body { white-space: nowrap; }
    .i { counter-increment: c 1; }
    .ib:before { counter-increment: c 1; content: "B" }
    .r { counter-reset: c 0; }
    .u:before { content: counters(c, ".") " "; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
