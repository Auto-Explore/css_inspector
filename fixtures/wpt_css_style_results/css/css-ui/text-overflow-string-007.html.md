# css/css-ui/text-overflow-string-007.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/text-overflow-string-007.html"
}
```

## style[0]

```css

  div {
    font-family: monospace;
    font-size: 10px;
    overflow: hidden;
    /* This custom ellipsis string has mixed LTR/RTL characters
     * ("A" and "ום"), but its base direction (determined by the first strong
     * char 'A') is LTR. */
    text-overflow: "A ום";
    width: 130px;
  }
  span {
    font-family: Ahem;
    font-size: 30px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
