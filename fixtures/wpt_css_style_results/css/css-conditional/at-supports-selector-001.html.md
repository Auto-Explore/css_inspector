# css/css-conditional/at-supports-selector-001.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-selector-001.html"
}
```

## style[0]

```css

  div {
    background-color:red;
    height:100px;
    width:100px;
  }
  @supports selector(a:link.class#ident) {
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
