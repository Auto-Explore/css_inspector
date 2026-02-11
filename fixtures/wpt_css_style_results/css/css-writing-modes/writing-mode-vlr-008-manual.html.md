# css/css-writing-modes/writing-mode-vlr-008-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/writing-mode-vlr-008-manual.html"
}
```

## style[0]

```css

@font-face {
    font-family: 'webfont';
    src: url('/fonts/noto/NotoSansMongolian-regular.woff2') format('woff2');
    font-weight: normal;
    font-style: normal;
	}
.test, .ref { font-family: webfont, serif; font-size: 24px; float: left; margin-right: 30px; color: #ccc; }
span[lang=ar] { color: black; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

.test { writing-mode: vertical-rl; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
