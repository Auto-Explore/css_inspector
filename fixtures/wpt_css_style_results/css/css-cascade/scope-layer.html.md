# css/css-cascade/scope-layer.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-layer.html"
}
```

## style[0]

```css

  @scope (.a) {
    /* The theme layer wins over the base layer. Note that @layer statements
       are allowed here, but aren't affected by the enclosing @scope. */
    @layer base, theme;

    @layer theme {
      :scope {
        z-index: 1;
      }

      .b {
        background-color: green;
      }
    }
  }

  @layer base {
    .a {
      z-index: 0;
    }
    .a .b {
      background-color: red;
    }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
