# css/css-transforms/preserve3d-overflow-percent.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/preserve3d-overflow-percent.html"
}
```

## style[0]

```css

:root {
  overflow: hidden;
}
body {
  margin: 0;
  font-size: 28px;
}
#map {
  width: 100%;
  height: 100%;
  backface-visibility: hidden;
}
.floor {
  width: 100%;
  height: 100vh;
}
.skew {
	height: 20em;
	width: 20em;
	position: relative;
	left: 50%;
	top: calc(50% + 1.5em);
	transform-origin: 0% 0%;
	transform: rotateX(45deg) rotateZ(-45deg) translateX(-50%) translateY(-50%);
	transform-style: preserve-3d;
}
.cylinder {
	position: absolute;
  background-color: green;
  transform-style: preserve-3d;
  width: 7em;
  height: 7em;
  transform: translateZ(1em);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
