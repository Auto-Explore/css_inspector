# css/css-masking/clip-path/clip-path-borderBox-1d.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-borderBox-1d.html"
}
```

## style[0]

```css

  .clipped {
    width: 50px;
    height: 50px;
    background-color: green;
    clip-path: border-box;
    border: 25px solid green;
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
