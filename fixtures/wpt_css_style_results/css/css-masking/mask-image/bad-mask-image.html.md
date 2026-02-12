# css/css-masking/mask-image/bad-mask-image.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/bad-mask-image.html"
}
```

## style[0]

```css

  .bad-mask {
    /* This invalid mask-image should fully-mask this element. */
    mask-image: url("invalid://nonexistent");
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
