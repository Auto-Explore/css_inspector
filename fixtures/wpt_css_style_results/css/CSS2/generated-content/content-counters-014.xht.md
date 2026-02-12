# css/CSS2/generated-content/content-counters-014.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/content-counters-014.xht"
}
```

## style[0]

```css


  body { white-space: nowrap; }


  body, #test { counter-reset: c; }
  p, #test span { counter-increment: c; }
  #test span:before { content: counters(c, ".", upper-alpha); }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
