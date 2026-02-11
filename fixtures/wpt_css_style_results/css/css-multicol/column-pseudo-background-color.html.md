# css/css-multicol/column-pseudo-background-color.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/column-pseudo-background-color.html"
}
```

## style[0]

```css

  @supports not selector(::column) {
    #multicol::before {
      content: "FAIL";
    }
  }
  #multicol {
    width: 100px;
    height: 100px;
    column-count: 1;
  }
  #multicol::column {
    background-color: red;
  }
  #content {
    width: 100px;
    height: 100px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
