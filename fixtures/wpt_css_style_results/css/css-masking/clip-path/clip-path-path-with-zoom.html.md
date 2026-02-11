# css/css-masking/clip-path/clip-path-path-with-zoom.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/clip-path/clip-path-path-with-zoom.html"
}
```

## style[0]

```css

  #red {
    position: absolute;
    width: 100px;
    height: 100px;
    background: red;
  }
  #rect {
    width: 100px;
    height: 100px;
    background-color: green;
    clip-path: path(nonzero, 'M0,0 L100,0  L0,100  L0,0');
    zoom: 2;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
