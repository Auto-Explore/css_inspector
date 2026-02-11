# css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-auto-no-items-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-lanes/tentative/track-sizing/auto-repeat/intrinsic-auto-repeat/row-auto-repeat-auto-no-items-crash.html"
}
```

## style[0]

```css

#grid-lanes {
    display: grid-lanes;
    grid-lanes-direction: row;
    grid-template-rows: repeat(auto-fill, minmax(20px, 30px) auto);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “grid-lanes-direction”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
