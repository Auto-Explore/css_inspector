# css/css-masking/clip-path/clip-path-marginBox-1b.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-marginBox-1b.html"
}
```

## style[0]

```css

  .clipped {
    width: 50px;
    height: 50px;
    background-color: green;
    clip-path: margin-box;
    /* This outline draws far outside the margin box and should be partially clipped. */
    outline: 200px solid green;
    margin: 25px;
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
