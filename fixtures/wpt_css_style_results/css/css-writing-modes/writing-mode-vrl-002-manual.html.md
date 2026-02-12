# css/css-writing-modes/writing-mode-vrl-002-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/writing-mode-vrl-002-manual.html"
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
.test, .ref { font-family: webfont, serif; font-size: 24px; height: 300px; width: 300px; }
.test span { background-color:orange; color:orange;  }
.test span#end { background-color:blue; color:blue;  }
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
