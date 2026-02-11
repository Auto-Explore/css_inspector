# css/css-masking/clip-path/clip-path-marginBox-1c.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-marginBox-1c.html"
}
```

## style[0]

```css

  .clipped {
    width: 50px;
    height: 50px;
    background-color: green;
    clip-path: margin-box;
    border-radius: 50px;
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
