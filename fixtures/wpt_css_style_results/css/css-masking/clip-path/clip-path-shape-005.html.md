# css/css-masking/clip-path/clip-path-shape-005.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-shape-005.html"
}
```

## style[0]

```css

  #rect {
    /* The size of the padding-box is 100x100. */
    width: 120px;
    height: 120px;
    padding: 10px;
    border: 10px solid red;
    box-sizing: border-box;
    background-color: green;
    clip-path: shape(from 0px 0px,
                     hline by 80px, vline by 80%, hline by -80%, close)
               padding-box;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
