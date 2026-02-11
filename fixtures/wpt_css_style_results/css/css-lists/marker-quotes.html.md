# css/css-lists/marker-quotes.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/marker-quotes.html"
}
```

## style[0]

```css

ol {
  quotes: "\2018" "\2019";
}
::marker {
  content: open-quote counter(list-item) close-quote " ";
}
:nth-child(2)::marker {
  quotes: "«" "»";
}
:nth-child(3)::marker {
  quotes: "‹" "›";
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
