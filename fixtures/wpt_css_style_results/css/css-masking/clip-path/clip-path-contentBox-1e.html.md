# css/css-masking/clip-path/clip-path-contentBox-1e.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-contentBox-1e.html"
}
```

## style[0]

```css

  body { margin: 0; }
  .clipped {
    display: inline-block;
    clip-path: content-box;
    /* This background draws in the padding and should be clipped. */
    background-color: red;
    padding: 4px;
    border-radius: 58px;
    /* This border draws outside the content box and should be clipped. */
    border: 4px solid darkred;
    /* This outline draws outside the content box and should be clipped. */
    outline: 100px solid maroon;
  }
  .content {
    width: 100px;
    height: 100px;
    background: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
