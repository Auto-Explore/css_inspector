# css/css-transforms/preserve3d-and-filter-with-perspective-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/preserve3d-and-filter-with-perspective-ref.html"
}
```

## style[0]

```css


  .scene {
    width: 400px;
    height: 400px;
    position: absolute;
    left: 100px;
    top: 100px;
    perspective:1300px;
  }

  .chair {
    width: 400px;
    height: 400px;
    transform: rotateY(88deg);
    transform-style: preserve-3d;
    transform-origin: 50% 50% 50%;
  }

  .shadow {
    position: absolute;
    top: 250px;
    left: 0;
    width: 200px;
    height: 200px;
    background: white;
    transform: rotateX(90deg) translate3d(0,-60px,0);
  }

  svg.cover {
    position: absolute;
    z-index: 1;
    top: 0;
    left: 0;
    width: 500px;
    height: 500px;
  }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
