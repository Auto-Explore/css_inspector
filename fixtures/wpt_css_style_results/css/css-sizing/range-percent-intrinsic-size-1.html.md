# css/css-sizing/range-percent-intrinsic-size-1.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/range-percent-intrinsic-size-1.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:16px/1 monospace;
}

input { margin: 2px; }

input.i {
  width: 50%;
  min-width: min-content;
  background: lime;
}

input.mi {
  max-width: 50%;
  min-width: min-content;
  background: lime;
}

input.i.min-auto, input.mi.min-auto {
  min-width: auto;
}

.n {
  -webkit-appearance: none;
}

div {
  display: inline-block;
  border:1px solid;
}

.grid {
  display: inline-grid;
  grid: auto / min-content;
  place-items: start;
}

.outerFlex {
  display: flex;
  width: 100px;
  border: 1px solid black;
}
.innerFlex {
  display: flex;
  border: 1px solid pink;
}
.innerFlex > input {
  width: 50%;
  flex: 1;
  background: yellow;
  -moz-appearance: none;
  -webkit-appearance: none;
}

```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown property “-webkit-appearance”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-moz-appearance”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-appearance”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
