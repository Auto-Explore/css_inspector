# css/css-fonts/metrics-override-normal-keyword.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/metrics-override-normal-keyword.html"
}
```

## style[0]

```css

@font-face {
  font-family: ascent-font;
  src: local(Ahem), url(/fonts/Ahem.ttf);
  ascent-override: 50%;
  ascent-override: normal;
}

@font-face {
  font-family: descent-font;
  src: local(Ahem), url(/fonts/Ahem.ttf);
  descent-override: 50%;
  descent-override: normal;
}

@font-face {
  font-family: line-gap-font;
  src: local(Ahem), url(/fonts/Ahem.ttf);
  line-gap-override: 50%;
  line-gap-override: normal;
}
```

```json
{
  "errors": 3,
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
      "message": "Unknown property “line-gap-override”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
