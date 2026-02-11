# css/CSS2/generated-content/counter-reset-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/counter-reset-002.xht"
}
```

## style[0]

```css


  body { white-space: nowrap; }


  #test, .reset { counter-reset: c; }
  .increment:before { content: counters(c, ".") "-"; }
  .increment { counter-increment: c; }

  
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
