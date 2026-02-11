# css/css-inline/text-box-trim/text-box-trim-set-dynamically-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-set-dynamically-ref.html"
}
```

## style[0]

```css

@import "support/TestMetricsFont.css";

#container {
  font: 100px/1 MetricsTestFont;
  text-box: trim-both ex alphabetic;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    },
    {
      "message": "Unknown property “text-box”.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
