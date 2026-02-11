# css/css-inline/text-box-trim/text-box-trim-om-001.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-om-001.html"
}
```

## style[0]

```css

@font-face {
  font-family: test-font;
  src: url(support/cap-x-height.ttf);
}
.spacer {
  background: lightgray;
  block-size: 100px;
}
#target {
  font-family: test-font;
  font-size: 100px;
  line-height: 2;
  text-box-trim: both;
  text-box-edge: ex alphabetic;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “text-box-trim”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “text-box-edge”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
