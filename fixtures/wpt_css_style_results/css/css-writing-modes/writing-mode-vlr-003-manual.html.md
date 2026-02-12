# css/css-writing-modes/writing-mode-vlr-003-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/writing-mode-vlr-003-manual.html"
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
.test { font-family: webfont, serif; font-size: 24px; height: 250px; width: 250px; border: 1px solid orange; }
.test span { background-color:orange; color:orange;  }
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

.test { writing-mode: vertical-lr; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
