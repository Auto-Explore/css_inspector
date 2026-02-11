# css/css-properties-values-api/registered-property-crosstalk.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/registered-property-crosstalk.html"
}
```

## style[0]

```css


  @property --x {
    syntax: "<number>";
    inherits: true;
    initial-value: 0;
  }

  #a {
    --y: 0;
  }

  #b {
    --z: 0;
  }

  #c {
    --x: 42;
  }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
