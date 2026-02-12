# css/CSS2/generated-content/content-counters-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/content-counters-009.xht"
}
```

## style[0]

```css


  body { white-space: nowrap; }


  body, #test span:first-child { counter-reset: c; }
  p, #test span { counter-increment: c; }
  #test span:before { content: counters(c, ".", georgian); }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
