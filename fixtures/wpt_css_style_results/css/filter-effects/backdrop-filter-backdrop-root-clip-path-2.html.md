# css/filter-effects/backdrop-filter-backdrop-root-clip-path-2.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/backdrop-filter-backdrop-root-clip-path-2.html"
}
```

## style[0]

```css

  .grandparent {
    display: block;
    background: yellow;
    padding: 100px;
  }

  .clipped-container {
    position: relative;
    clip-path: inset(-50px);
    background-color: rgb(255, 0, 255);
    padding: 100px;
    display: inline;
  }

  .backdrop-filtered {
    backdrop-filter: invert(100%);
    position: absolute;
    top: -50px;
    left: -50px;
    width: 400px;
    height: 400px;
    display: inline;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
