# css/css-grid/grid-lanes/tentative/items/row-minimum-contribution-with-percentages.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/items/row-minimum-contribution-with-percentages.html"
}
```

## style[0]

```css

#grid-lanes {
  display: grid-lanes;
  grid-lanes-direction: row;
  height: 50px;
  width: 50px;
  grid-template-rows: auto;
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
      "message": "Unknown property “grid-lanes-direction”.",
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
