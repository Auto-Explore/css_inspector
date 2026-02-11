# css/css-counter-styles/hebrew/counter-hebrew-nested.html

```json
{
  "format_version": 3,
  "file": "css/css-counter-styles/hebrew/counter-hebrew-nested.html"
}
```

## style[0]

```css


  body, #test span:first-child { counter-reset: c; }
  p, #test span { counter-increment: c; }
  #test span:before { content: counters(c, ".", hebrew); }
  #test span { direction: rtl; }

  
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
