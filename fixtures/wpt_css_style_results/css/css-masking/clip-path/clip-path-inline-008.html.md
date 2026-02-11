# css/css-masking/clip-path/clip-path-inline-008.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-inline-008.html"
}
```

## style[0]

```css

  .container {
    font: 50px/1 Ahem;
    line-height: 50px;
  }
  .container > span {
    margin: 25px;
    border-radius: 50px;
    clip-path: margin-box;
    color: green;
    /* This outline draws far outside the margin box and should be partially clipped. */
    outline: 200px solid green;
    /* Disables margin collapsing. */
    float: left;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
