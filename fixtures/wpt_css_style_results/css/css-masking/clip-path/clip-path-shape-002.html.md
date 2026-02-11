# css/css-masking/clip-path/clip-path-shape-002.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-shape-002.html"
}
```

## style[0]

```css

  #rect {
    width: 100px;
    height: 100px;
    background-color: green;
    clip-path: shape(evenodd from 10px 10px,
                     hline by 80px, vline by 80%, hline by -80px, close,
                     move to 25% 25px, hline by 50px, vline by 50px, hline by -50%);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
