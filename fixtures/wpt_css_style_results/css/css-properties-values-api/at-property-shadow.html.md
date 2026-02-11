# css/css-properties-values-api/at-property-shadow.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/at-property-shadow.html"
}
```

## style[0]

```css

  @property --x {
    syntax: "<length>";
    inherits: false;
    initial-value: 0px;
  }
  #outside {
    --x: calc(1px + 1px);
    --y: calc(1px + 1px);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

    /* This rule should be globally registered */
    @property --y {
      syntax: "<length>";
      inherits: false;
      initial-value: 0px;
    }
    #inside {
      --x: calc(1px + 1px);
      --y: calc(1px + 1px);
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
