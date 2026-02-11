# css/css-contain/quote-scoping-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/quote-scoping-001.html"
}
```

## style[0]

```css


div {
  quotes: "A" "Z" "1" "9";
}
div::before, span::before {
  content: open-quote;
}
div::after {
  content: close-quote;
}
span {
  contain: style;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “quotes”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
