# css/css-fonts/variations/variable-gpos-m2b.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/variations/variable-gpos-m2b.html"
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

    .gpos_m2b_left {
    font-variation-settings: "VM2B" 0;
    }

    .gpos_m2b_middle {
    font-variation-settings: "VM2B" 500;
    }

    .gpos_m2b_right {
    font-variation-settings: "VM2B" 1000;
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
