# css/css-fonts/font-display/font-display-failure-fallback.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-display/font-display-failure-fallback.html"
}
```

## style[0]

```css

@font-face {
  font-family: 'TestFace';
  src: url('/fonts/Ahem.ttf?pipe=trickle(d0.5)'),
       url('/fonts/Ahem.ttf?fallback-src');
  font-display: optional;
}

@font-face {
  font-family: 'FallbackFace';
  src: url('/fonts/Ahem.ttf?fallback-face');
}

.test {
  font-family: 'TestFace','FallbackFace';
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
