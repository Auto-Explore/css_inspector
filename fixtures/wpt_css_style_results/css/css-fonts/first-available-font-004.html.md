# css/css-fonts/first-available-font-004.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/first-available-font-004.html"
}
```

## style[0]

```css

/* Two arbitrary fonts with different metrics */
@font-face {
  font-family: 'A';
  font-style: normal;
  font-weight: 400;
  src:  url(/fonts/Revalia.woff) format('woff');
}
@font-face {
  font-family: 'B-no-space';
  font-style: normal;
  font-weight: 400;
  src: url(/fonts/AD.woff) format('woff');
  unicode-range: U+0062;
}

div {
  position: absolute;
  line-height: normal;
  font-size: 100px;
  color: transparent;
  border: solid black 1px;
  width: 100px;
}

.a { font-family: A; }
.ba { font-family: B-no-space, A; margin-left: 100px; }
.loader { font-family: B-no-space; border: none; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
