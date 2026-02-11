# css/css-masking/clip-path/clip-path-paddingBox-1e.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-paddingBox-1e.html"
}
```

## style[0]

```css

  body { margin: 0; }
  .clipped {
    width: 100px;
    height: 100px;
    background-color: green;
    clip-path: padding-box;
    /* This border draws outside the padding box and should be clipped. */
    border: 8px solid red;
    border-radius: 58px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
