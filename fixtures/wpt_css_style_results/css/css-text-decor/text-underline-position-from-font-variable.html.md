# css/css-text-decor/text-underline-position-from-font-variable.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-underline-position-from-font-variable.html"
}
```

## style[0]

```css

@font-face {
font-family: underline-variable;
src: url(resources/UnderlineTest-VF.ttf);
}

.test {
text-underline-position: from-font;
font-size: 64px;
line-height: 1.8;
}

.close_underline {
text-decoration: underline;
font-family: underline-variable, sans-serif;
font-variation-settings: 'UNDO' 1;
}

.far_underline {
text-decoration: underline;
font-family: underline-variable, sans-serif;
font-variation-settings: 'UNDO' 1000;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “font-variation-settings”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-variation-settings”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
