# css/css-writing-modes/writing-mode-vrl-forms-006-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/writing-mode-vrl-forms-006-manual.html"
}
```

## style[0]

```css

@font-face {
    font-family: "webfont";
    src: url("/fonts/mplus-1p-regular.woff") format("woff");
    font-weight: normal;
    font-style: normal;
    }
select { font-family: webfont; font-size: 24px; }
.test { text-align: center; vertical-align: middle; width: 100%; }
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
