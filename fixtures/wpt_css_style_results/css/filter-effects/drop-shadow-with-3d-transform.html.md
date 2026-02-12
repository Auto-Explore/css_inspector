# css/filter-effects/drop-shadow-with-3d-transform.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/drop-shadow-with-3d-transform.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

#container {
  position: relative;
  width: 400px;
  height: 400px;
  overflow: hidden;
  border: 1px solid black;
}

#box1 {
  position: absolute;
  width: 100px;
  height: 100px;
  background: green;
  left: -50px;
  top: -50px;
  transform: translateZ(0);
  filter: drop-shadow(100px 100px 0 lightgreen);
}

#box2 {
  position: absolute;
  width: 100px;
  height: 100px;
  background: blue;
  left: 350px;
  top: 350px;
  transform: translateZ(0);
  filter: drop-shadow(-100px -100px 0 lightblue);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
