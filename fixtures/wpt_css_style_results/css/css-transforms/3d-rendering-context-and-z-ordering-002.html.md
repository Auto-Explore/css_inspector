# css/css-transforms/3d-rendering-context-and-z-ordering-002.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/3d-rendering-context-and-z-ordering-002.html"
}
```

## style[0]

```css


#contain {
  position: relative;
  padding-top: 5px;
}

#wrapper {
  width: 30px;
  height: 30px;
  margin-top: 20px;
  margin-left: 20px;

  perspective: 700px;
}

#cube {
  width: 15px;
  height: 15px;
  transform-style: preserve-3d;
}

#face {
  width: 15px;
  height: 15px;
  transform: translateZ(75px);
  background: red;
}

#tooltip {
  position: absolute;
  background: lime;
  width: 100px;
  height: 100px;
  top: 0;
  left: 0;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
