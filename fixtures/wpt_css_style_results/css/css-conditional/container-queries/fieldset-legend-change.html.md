# css/css-conditional/container-queries/fieldset-legend-change.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/fieldset-legend-change.html"
}
```

## style[0]

```css

  fieldset {
    width: 200px;
    container-type: inline-size;
  }
  .wide { width: 400px; }

  @container (min-width: 300px) {
    #fail {
      display: none;
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
