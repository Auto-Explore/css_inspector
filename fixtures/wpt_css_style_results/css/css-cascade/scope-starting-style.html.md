# css/css-cascade/scope-starting-style.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-starting-style.html"
}
```

## style[0]

```css

  @scope (.a) {
    @starting-style {
      :scope {
        width: 100px;
      }

      .b {
        width: 100px;
      }
    }
  }

  .a, .b {
    transition: width 100s steps(2, start); /* 50% progress */
    width: 200px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
