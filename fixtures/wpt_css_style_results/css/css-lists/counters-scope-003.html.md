# css/css-lists/counters-scope-003.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/counters-scope-003.html"
}
```

## style[0]

```css

body, span#reset:before { counter-reset: c 0; }
span::before { counter-increment: c 1; content: "B" counters(c,".") "-" }
span::after  { counter-increment: c 1; content: "A" counters(c,".") "-" }
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
