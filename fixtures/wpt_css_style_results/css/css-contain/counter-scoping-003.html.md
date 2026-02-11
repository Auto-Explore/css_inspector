# css/css-contain/counter-scoping-003.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/counter-scoping-003.html"
}
```

## style[0]

```css

div {
  contain: style;
  counter-increment: c 123;
}
span {
  counter-increment: c 1;
}
span::before {
  content: counter(c);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
