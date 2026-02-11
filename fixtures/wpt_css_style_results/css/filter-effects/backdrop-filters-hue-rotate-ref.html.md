# css/filter-effects/backdrop-filters-hue-rotate-ref.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/backdrop-filters-hue-rotate-ref.html"
}
```

## style[0]

```css

    .square {
        position: absolute;
        width: 100px;
        height: 100px;
        top: 100px;
        left: 100px;
        background: blue;
        filter: hue-rotate(45deg);
    }
    .filt {
        position: absolute;
        width: 200px;
        height: 200px;
        top: 50px;
        left: 50px;
        background: white;
        filter: hue-rotate(45deg);
    }
    .greenbox {
        position: absolute;
        width: 50px;
        height: 50px;
        top: 125px;
        left: 125px;
        background: green;
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
