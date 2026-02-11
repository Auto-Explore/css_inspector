# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-010.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/column-auto-repeat-010.html"
}
```

## style[0]

```css

.grid-lanes {
    position: relative;
    display: inline-grid-lanes;
    grid-template-columns: repeat(auto-fill, 25%);
    min-width: 50%;
    height: 200px;
    float: left;
    background: pink;
}
.wrapper {
  width: 600px;
}
.item {
  background: lime;
  width: 100%;
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
