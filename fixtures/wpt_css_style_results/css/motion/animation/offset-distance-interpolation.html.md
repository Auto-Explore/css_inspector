# css/motion/animation/offset-distance-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/motion/animation/offset-distance-interpolation.html"
}
```

## style[0]

```css

    .parent {
      offset-distance: 30px;
    }
    .target {
      width: 10px;
      height: 10px;
      background-color: black;
      offset-path: path("M0 0H 400");
      offset-distance: 10px;
    }
    .expected {
      background-color: green;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
