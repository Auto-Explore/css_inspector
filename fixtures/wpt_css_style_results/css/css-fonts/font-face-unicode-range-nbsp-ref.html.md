# css/css-fonts/font-face-unicode-range-nbsp-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-face-unicode-range-nbsp-ref.html"
}
```

## style[0]

```css

@font-face {
  font-family: test;
  src: url(/fonts/Ahem.ttf);
  ascent-override: 80%;
  descent-override: 20%;
}
@font-face {
  font-family: fallback;
  src: url(/fonts/GentiumPlus-R.woff);
  ascent-override: 80%;
  descent-override: 20%;
}
#test, #ref {
  font: 24px/2 fallback, serif;
}
span {
  font-family: test;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “ascent-override”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “descent-override”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “ascent-override”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “descent-override”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
