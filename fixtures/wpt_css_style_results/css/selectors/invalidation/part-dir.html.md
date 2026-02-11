# css/selectors/invalidation/part-dir.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/part-dir.html"
}
```

## style[0]

```css

  my-element::part(inner) {
    background-color: #ff0000;
  }
  my-element::part(inner):dir(ltr) {
    background-color: #00ff00;
  }
  my-element::part(inner):dir(rtl) {
    background-color: #0000ff;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
