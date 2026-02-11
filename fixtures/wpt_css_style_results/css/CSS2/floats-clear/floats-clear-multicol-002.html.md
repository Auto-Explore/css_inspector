# css/CSS2/floats-clear/floats-clear-multicol-002.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/floats-clear-multicol-002.html"
}
```

## style[0]

```css

.multicol {
  margin: 1em;
  border: solid silver;
  width: 300px;
  column-width: 100px;
  column-gap: 0;
  column-fill: auto;
  height: 100px;
}

.float {
  float: right;
  width: 15px;
  background: aqua;
  height: 250px;
}
.L {
  float: left;
}

.container {
  width: 100%;
  background: red;
}

.clear {
  clear: left;
  border-bottom: solid orange;
  background: red;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
