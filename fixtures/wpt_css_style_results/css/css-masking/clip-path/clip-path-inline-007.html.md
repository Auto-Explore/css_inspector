# css/css-masking/clip-path/clip-path-inline-007.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-inline-007.html"
}
```

## style[0]

```css

  .container {
    font: 100px/1 Ahem;
    line-height: 100px;
  }
  .container > span {
    border-radius: 50px;
    clip-path: border-box;
    color: green;
    /* This outline draws outside the border box and should be clipped. */
    outline: 8px solid red;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
