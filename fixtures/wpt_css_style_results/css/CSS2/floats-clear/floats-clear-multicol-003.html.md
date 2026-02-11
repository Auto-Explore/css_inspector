# css/CSS2/floats-clear/floats-clear-multicol-003.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats-clear/floats-clear-multicol-003.html"
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

.step {
  height: 10px;
  border: 15px aqua;
  border-style: none solid;
}
.float {
  float: right;
  width: 15px;
  background: aqua;
  height: 240px;
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
  height: 0;
  background: red;
}
.bar {
  border-bottom: orange solid;
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
