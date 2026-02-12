# css/css-values/calc-in-counter-001.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-in-counter-001.xhtml"
}
```

## style[0]

```css


  body {
   white-space: nowrap;
  }

  #a { counter-reset: a calc(3 + 5); }
  #a::before { content: counter(a); }

  #b { counter-reset: b 0; counter-increment: b calc(4 + 6); }
  #b::before { content: counter(b); }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
