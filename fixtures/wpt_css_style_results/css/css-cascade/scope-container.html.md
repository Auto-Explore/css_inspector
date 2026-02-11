# css/css-cascade/scope-container.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-container.html"
}
```

## style[0]

```css

  main {
    width: 100px;
    height: 100px;
    container-type: size;
  }

  @scope (.a) {
    @container (width > 0px) {
      :scope {
        z-index: 1;
      }

      .b {
        background-color: green;
      }
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
