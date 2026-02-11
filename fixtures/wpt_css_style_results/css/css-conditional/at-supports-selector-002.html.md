# css/css-conditional/at-supports-selector-002.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-selector-002.html"
}
```

## style[0]

```css

  div {
    background-color:red;
    height:100px;
    width:100px;
  }
  @supports selector(::before) {
    div { background: green };
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
