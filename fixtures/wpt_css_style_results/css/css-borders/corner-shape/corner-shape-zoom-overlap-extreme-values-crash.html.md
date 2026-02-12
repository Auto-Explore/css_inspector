# css/css-borders/corner-shape/corner-shape-zoom-overlap-extreme-values-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/corner-shape/corner-shape-zoom-overlap-extreme-values-crash.html"
}
```

## style[0]

```css

  .target {
    display: inline;
    clip-path: border-box;
    corner-shape: bevel square scoop;
    zoom: 10%;
    border-radius: 100%;
    border: 5px solid purple;
    bottom: -1e100px;
    position: relative;
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
