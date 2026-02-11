# css/css-fonts/variations/variable-opsz-size-adjust.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/variations/variable-opsz-size-adjust.html"
}
```

## style[0]

```css

    @font-face {
    font-family: variabletest_box;
    src: url(resources/variabletest_box.ttf);
    size-adjust: 150%;
    }

    body {
    font-family: variabletest_box, sans-serif;
    }

    div {
    line-height: 64px;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “size-adjust”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
