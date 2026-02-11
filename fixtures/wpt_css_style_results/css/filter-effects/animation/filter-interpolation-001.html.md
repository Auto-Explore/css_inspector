# css/filter-effects/animation/filter-interpolation-001.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/animation/filter-interpolation-001.html"
}
```

## style[0]

```css

      .parent {
        filter: hue-rotate(30deg);
      }
      .target {
        display: inline-block;
        width: 50px;
        height: 50px;
        background-color: green;
        color: white;
        margin-right: 2px;
        filter: hue-rotate(10deg);
      }
      .expected {
        margin-right: 20px;
      }
      .test {
        padding-bottom: 10px;
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
