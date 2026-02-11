# css/CSS2/generated-content/counters-scope-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/generated-content/counters-scope-002.xht"
}
```

## style[0]

```css


  body { white-space: nowrap; }


  span:before { counter-increment: c 1; content: "B" counters(c,".") "-" }
  span:after  { counter-increment: c 1; content: "A" counters(c,".") "-" }

  body, span#reset:before { counter-reset: c 0; }

  
```

```json
{
  "errors": 2,
  "messages": [
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
