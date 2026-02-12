# css/CSS2/generated-content/content-counters-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/content-counters-008.xht"
}
```

## style[0]

```css


  body { white-space: nowrap; }


  body, #test span:first-child { counter-reset: c; }
  p, #test span { counter-increment: c; }
  #test span:before { content: counters(c, ".", upper-roman); }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
