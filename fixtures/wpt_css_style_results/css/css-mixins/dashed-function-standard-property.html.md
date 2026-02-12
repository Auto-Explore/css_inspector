# css/css-mixins/dashed-function-standard-property.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/dashed-function-standard-property.html"
}
```

## style[0]

```css

  @function --ten-px() {
    result: 10px;
  }

  @function --ten-px-typed() returns <length> {
    result: 10px;
  }

  @function --green() {
    result: green;
  }

  #target {
    width: --ten-px();
    height: --ten-px-typed();
    color: --green();
    padding: --ten-px();
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
