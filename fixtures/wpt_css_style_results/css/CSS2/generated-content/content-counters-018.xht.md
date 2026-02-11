# css/CSS2/generated-content/content-counters-018.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/content-counters-018.xht"
}
```

## style[0]

```css


  body { white-space: nowrap; }


  body, #test { counter-reset: c 0 f 1000; }
  p, #test span { counter-increment: c; }
  #test span:before {
    content: counters(c, ".");
    content: counters(f);
    content: counters(f, decimal);
    content: counters(f, decimal, ".");
    content: counters(f, ".", decimal, decimal);
    content: counters(f, ".", decimal, ".");
  }

  
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
