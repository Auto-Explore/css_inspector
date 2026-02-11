# css/CSS2/generated-content/content-counters-015.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/content-counters-015.xht"
}
```

## style[0]

```css


  body { white-space: nowrap; }


  body, #test { counter-reset: c; }
  p, #test span { counter-increment: c; }
  #test span:before { content: counters(c, ".", lower-greek); }

  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
