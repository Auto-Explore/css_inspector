# css/css-grid/grid-definition/flex-item-grid-container-percentage-rows-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-definition/flex-item-grid-container-percentage-rows-001.html"
}
```

## style[0]

```css

.flex {
  display: flex;
  flex-direction: column;
  width: 200px;
  height: 200px;
  border: 5px solid;
}

.flexitem {
  flex: 1;
  background: magenta;
}

.grid {
  display: grid;
  grid: 50% / 1fr;
}

.griditem {
  background: cyan;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
