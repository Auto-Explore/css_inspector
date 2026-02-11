# css/css-fonts/variations/variable-box-font.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/variations/variable-box-font.html"
}
```

## style[0]

```css

    @font-face {
    font-family: variabletest_box;
    src: url(resources/variabletest_box.ttf);
    }

    body {
    font-family: variabletest_box,
    sans-serif;
    font-size: 200px;
    }

    .a_up {
    font-variation-settings: "UPWD" 350;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “font-variation-settings”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
