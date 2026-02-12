# css/CSS2/generated-content/counters-scope-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/counters-scope-003.xht"
}
```

## style[0]

```css


  body { white-space: nowrap; }


  span:before { counter-increment: c 1; content: "B" counters(c,".") "-" }
  span:after  { counter-increment: c 1; content: "A" counters(c,".") "-" }

  body, span#reset:after { counter-reset: c 0; }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
