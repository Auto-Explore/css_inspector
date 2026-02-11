# css/css-text-decor/text-decoration-inset-019.html

```json
{
  "format_version": 3,
  "file": "css/css-text-decor/text-decoration-inset-019.html"
}
```

## style[0]

```css

body {
  background: white;
  color: black;
}
div.outer {
  display: inline-block;
  vertical-align: top;
  font: 10px/2 Ahem;
  position: relative;
  width: 12ch;
  height: 12ch;
  border: 1px solid gray;
  margin: 1em;
}
div.inner {
  position: absolute;
  text-decoration: underline;
  text-decoration-inset: 2ch -2ch;
  text-underline-offset: 3px;
  text-decoration-thickness: 2px;
  padding: 2ch;
}
div.inner > p {
  margin: 0;
}
p:dir(rtl) {
  unicode-bidi: bidi-override;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “text-decoration-inset”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
