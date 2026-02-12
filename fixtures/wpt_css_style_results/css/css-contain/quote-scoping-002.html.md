# css/css-contain/quote-scoping-002.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/quote-scoping-002.html"
}
```

## style[0]

```css


div {
  quotes: "A" "Z" "1" "9";
}
div::before {
  content: open-quote;
}
div::after, span::after {
  content: close-quote;
}
span {
  contain: style;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
