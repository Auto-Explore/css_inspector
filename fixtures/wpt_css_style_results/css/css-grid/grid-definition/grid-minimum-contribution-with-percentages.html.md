# css/css-grid/grid-definition/grid-minimum-contribution-with-percentages.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-definition/grid-minimum-contribution-with-percentages.html"
}
```

## style[0]

```css

#grid {
  display: grid;
  height: 50px;
  width: 50px;
  grid: auto / auto;
}
#item {
  background: cyan;
}
#content {
  height: 100px;
  width: 100px;
}
.min {
  min-height: calc(100% + 50px);
  min-width: calc(100% + 50px);
}
.max {
  max-height: calc(100% - 50px);
  max-width: calc(100% - 50px);
}
.size {
  height: calc(100% + 10px);
  width: calc(100% + 10px);
}
```

```json
{
  "errors": 2,
  "messages": [
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
