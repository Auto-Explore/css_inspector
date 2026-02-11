# css/css-borders/tentative/border-shape/border-shape-stroke-from-border-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-shape/border-shape-stroke-from-border-ref.html"
}
```

## style[0]

```css

body {
  margin: 0;
}
.container {
  display: flex;
  gap: 32px;
  padding: 20px;
}
.target {
  width: 120px;
  height: 120px;
  overflow: visible;
  box-sizing: padding-box;
}
.case1 {
  border-width: 18px 10px 4px 6px;
  border-color: transparent;
  border-style: solid;
}
.case2 {
  writing-mode: vertical-rl;
  border-width: 18px 10px 4px 6px;
  border-color: transparent;
  border-style: solid;
}
.case3 {
  writing-mode: vertical-rl;
  border-width: 18px 10px 4px 0px;
  border-color: transparent;
  border-style: solid;
}
.case4 {
  writing-mode: vertical-lr;
  border-width: 18px 10px 4px 0px;
  border-color: transparent;
  border-style: solid;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
