# css/selectors/invalidation/part-lang.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/part-lang.html"
}
```

## style[0]

```css

  my-element::part(inner) {
    background-color: #ff0000;
  }
  my-element::part(inner):lang(en) {
    background-color: #00ffff;
  }
  my-element::part(inner):lang(en-GB) {
    background-color: #00ff00;
  }
  my-element::part(inner):lang(fr) {
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
