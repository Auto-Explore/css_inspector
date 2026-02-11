# css/css-multicol/multicol-overflow-clip.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-overflow-clip.html"
}
```

## style[0]

```css

.multicol {
  column-count: 3;
}
.parent {
  background: green;
  height: 50px;
  overflow: hidden;
}
.child2 {
  margin-top: 50px;
  background: darkred;
  color: red;
  height: 100px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
