# css/css-view-transitions/new-content-container-writing-modes-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/new-content-container-writing-modes-ref.html"
}
```

## style[0]

```css

.tb { writing-mode: horizontal-tb; }
.lr { writing-mode: vertical-lr; }
.rl { writing-mode: vertical-rl; }
.shared {
  margin: 2px;
  width: 100px;
  height: 50px;
  background: green;
  contain: paint;
  border: 1px solid black;
}
html { background: lightpink; }
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
