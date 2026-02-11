# css/css-writing-modes/text-combine-upright-all-002-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-combine-upright-all-002-manual.html"
}
```

## style[0]

```css

@font-face {
    font-family: "webfont";
    src: url("/fonts/CSSTest/mplus-1p-regular.woff") format("woff");
    font-weight: normal;
    font-style: normal;
    }
.test, .ref { font-family: webfont, serif; font-size: 24px; height: 300px; width: 300px; }
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
.test span { text-combine-upright: all; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
