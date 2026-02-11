# css/css-masking/clip-path/clip-path-shape-006.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-shape-006.html"
}
```

## style[0]

```css

  #rect {
    width: 140px;
    height: 140px;
    padding: 10px;
    border: 10px solid red;
    box-sizing: border-box;
    background-color: green;
    /* The size of the content-box is 100x100. */
    clip-path: shape(from -10px -10%,
                     hline by 80px, vline by 80%, hline by -80%, close)
               content-box;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
