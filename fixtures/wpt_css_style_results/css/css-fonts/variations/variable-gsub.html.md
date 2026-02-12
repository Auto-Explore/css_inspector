# css/css-fonts/variations/variable-gsub.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/variations/variable-gsub.html"
}
```

## style[0]

```css

    @font-face {
    font-family: variabletest_box;
    src: url(resources/variabletest_box.ttf);
    }

    body {
    font-family: variabletest_box, sans-serif;
    sans-serif;
    font-size: 100px;
    }

    .rvrn_replaced {
    font-variation-settings: "FVTT" 10;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
