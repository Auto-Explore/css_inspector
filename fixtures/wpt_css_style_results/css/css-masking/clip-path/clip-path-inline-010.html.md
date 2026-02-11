# css/css-masking/clip-path/clip-path-inline-010.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-inline-010.html"
}
```

## style[0]

```css

  .container {
    font: 100px/1 Ahem;
    line-height: 100px;
  }
  .container > span {
    margin-left: -8px;
    border-radius: 58px;
    clip-path: padding-box;
    color: green;
    /* This border draws outside the padding box and should be clipped. */
    border: 8px solid red;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
