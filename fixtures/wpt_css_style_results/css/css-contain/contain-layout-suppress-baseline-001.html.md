# css/css-contain/contain-layout-suppress-baseline-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-layout-suppress-baseline-001.html"
}
```

## style[0]

```css

  .flexBaselineCheck {
    display: flex;
    border: 1px solid black;
    height: 100px;
  }
  .flexBaselineCheck > * {
    contain: layout;
    border: 2px solid teal;
    align-self: baseline;
  }
  canvas {
    background: purple;
    width: 20px;
    height: 80px;
    box-sizing: border-box;
  }
  .flex {
    display: flex;
  }
  .grid {
    display: grid;
  }
  .multicol {
    column-count: 2;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
