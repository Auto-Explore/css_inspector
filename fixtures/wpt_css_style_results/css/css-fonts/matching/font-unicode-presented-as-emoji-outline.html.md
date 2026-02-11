# css/css-fonts/matching/font-unicode-presented-as-emoji-outline.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/matching/font-unicode-presented-as-emoji-outline.html"
}
```

## style[0]

```css

@font-face {
    font-family: 'Ahem Crossmark';
    src: url('./resources/AhemCrossmarkSupport.otf');
    unicode-range: U+0020, U+274C; /* include U+0020 to act as "first available font" and
                                      avoid line-height being derived from fallback */
}
.emoji {
    font-family: 'Ahem Crossmark', system-ui;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
