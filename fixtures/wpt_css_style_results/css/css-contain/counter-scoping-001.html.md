# css/css-contain/counter-scoping-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/counter-scoping-001.html"
}
```

## style[0]

```css

div {
  contain: style;
  counter-increment: n;
}
div::before, div::after {
  content: counters(n, '.') " ";
}
div::after {
  counter-increment: n 2;
}
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
