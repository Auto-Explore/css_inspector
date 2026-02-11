# css/css-conditional/at-supports-selector-004.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-selector-004.html"
}
```

## style[0]

```css

  div {
    background-color: green;
    height: 100px;
    width: 100px;
  }
  @supports selector(div, div) {
    div { background: red };
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
