# css/css-transforms/3dtransform-and-position-sticky-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/3dtransform-and-position-sticky-ref.html"
}
```

## style[0]

```css


div, img {
  height: 100px;
  width: 100px;
}

#outer {
  transform-style: preserve-3d;
  perspective: 300px;
}

#middle {
  transform-style: preserve-3d;
  position: relative;
  background: aqua;
}

#inner2 {
  background: olive;
  position: absolute;
  top: 0;
  left: 0;
  transform: rotateX(30deg);
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
