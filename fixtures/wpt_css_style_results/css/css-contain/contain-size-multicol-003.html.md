# css/css-contain/contain-size-multicol-003.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-size-multicol-003.html"
}
```

## style[0]

```css

  .contain {
    contain: size;
    border: 1em solid green;
    background: red;
    column-count: 3;
  }
  .innerContents {
    color: transparent;
    height: 100px;
    width: 100px;
  }
  .col-width {
    column-width: 20px;
  }
  .col-gap {
    column-gap: 5px;
  }
  .flexBaselineCheck {
    display: flex;
    align-items: baseline;
  }
  .min {
    width: min-content;
  }
  .max {
    width: max-content;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
