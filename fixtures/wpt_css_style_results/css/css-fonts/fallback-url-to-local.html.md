# css/css-fonts/fallback-url-to-local.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/fallback-url-to-local.html"
}
```

## style[0]

```css

@font-face {
  font-family: remote-font;
  src: url("/fonts/Revalia.woff?pipe=trickle(d1)") format(woff);
}

@font-face {
  font-family: local-font;
  src: local(Ahem);
}

#target {
  font: 25px/1 remote-font, local-font, monospace;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
