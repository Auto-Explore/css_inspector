# css/css-transforms/preserve3d-button.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/preserve3d-button.html"
}
```

## style[0]

```css

.scene {
  width: 200px;
  height: 200px;
  perspective: 5000px;
}

.card {
  width: 100%;
  height: 100%;
  transform-style: preserve-3d;
  position: relative;
  background: none;
  border: 0;
  padding: 0;
  transform: rotateY(180deg);
}

.face {
  position: absolute;
  top: 0;
  width: 100%;
  height: 100%;
  backface-visibility: hidden;
  background: red;
}

.backface {
  background: green;
  transform: rotateY(180deg);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
