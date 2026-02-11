# css/css-break/overflow-clip-012.html

```json
{
  "format_version": 3,
  "file": "css/css-break/overflow-clip-012.html"
}
```

## style[0]

```css

  #multicol {
    columns: 2;
    column-fill: auto;
    column-gap: 0px;
    height: 100px;
    width: 100px;
  }
  fieldset {
     overflow: clip;
     margin: 0;
     border: none;
     padding: 0;
     max-height: 200px;
  }
  fieldset > div {
    contain: size;
    width: 50px;
    background: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
