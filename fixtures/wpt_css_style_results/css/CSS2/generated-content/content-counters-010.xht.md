# css/CSS2/generated-content/content-counters-010.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/content-counters-010.xht"
}
```

## style[0]

```css


  body { white-space: nowrap; }


  body, #test span:first-child { counter-reset: c; }
  p, #test span { counter-increment: c; }
  #test span:before { content: counters(c, ".", armenian); }

  
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
