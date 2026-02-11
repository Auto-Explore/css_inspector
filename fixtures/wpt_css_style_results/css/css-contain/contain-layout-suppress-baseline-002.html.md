# css/css-contain/contain-layout-suppress-baseline-002.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-layout-suppress-baseline-002.html"
}
```

## style[0]

```css

  .flexBaselineCheck {
    display: flex;
    border: 1px solid black;
  }
  .flexBaselineCheck > * {
    contain: layout;
    border: 2px solid teal;
    align-self: baseline;
    margin: 2px;
  }
  canvas {
    background: purple;
    width: 20px;
    height: 50px;
    box-sizing: border-box;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
