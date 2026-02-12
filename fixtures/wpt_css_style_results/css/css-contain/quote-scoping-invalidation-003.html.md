# css/css-contain/quote-scoping-invalidation-003.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/quote-scoping-invalidation-003.html"
}
```

## style[0]

```css

  #root {
    quotes: "A" "Z" "1" "9" "(" ")" "+" "-";
  }
  #root::before, #root span::before {
    content: open-quote;
  }
  #root::after {
    content: close-quote;
  }
 .contain-style {
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
