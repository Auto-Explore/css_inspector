# css/css-pseudo/marker-content-009.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-009.tentative.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:16px/1 monospace; padding:0; margin:0;
}
body { margin-left: 40px; }
li::marker { content: 'AB'; }
.v > li::marker { writing-mode: vertical-lr; }
.lr { writing-mode: vertical-lr; }
.big { font-size:xx-large; }
.big-marker > li::marker { font-size:xx-large; }
f { float: left; margin-right: 20px; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
