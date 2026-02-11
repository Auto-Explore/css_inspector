# css/css-inline/initial-letter/crashtests/initial-letter-dynamic-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/initial-letter/crashtests/initial-letter-dynamic-crash.html"
}
```

## style[0]

```css

p::first-letter {
  background-image: url(../../../../images/blue.png);
}
p.initial-letter::first-letter {
  initial-letter: 2;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
