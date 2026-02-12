# css/css-masking/mask-image/mask-image-backdrop-filter-with-border-radius.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-image-backdrop-filter-with-border-radius.html"
}
```

## style[0]

```css

  body {
    background-color: rgb(173, 216, 230);
  }

  .wrapper {
    border-radius: 1px;
    overflow: hidden;
  }

  .element {
    backdrop-filter: saturate(300%);
    mask-image: linear-gradient(180deg, rgba(0, 0, 0, 0) 0, #000 100%);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
