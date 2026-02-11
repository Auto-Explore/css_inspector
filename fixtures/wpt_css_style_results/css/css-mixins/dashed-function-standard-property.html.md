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
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
