# css/css-writing-modes/writing-mode-vlr-table-001-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/writing-mode-vlr-table-001-manual.html"
}
```

## style[0]

```css

@font-face {
    font-family: 'webfont';
    src: 	url('/fonts/noto/NotoSansMongolian-regular.woff2') format('woff2');
    font-weight: normal;
    font-style: normal;
	}
.test, .ref { font-family: webfont, serif; font-size: 14px; height: 300px; width: 300px; }
.test span { background-color:orange; color:orange;  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
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
